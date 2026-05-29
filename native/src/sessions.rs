use crate::wrap_err;
use crate::common::{hermes_home, desktop_dir};
use napi_derive::napi;
use rusqlite::Connection;
use std::fs;

fn state_db_path() -> napi::Result<std::path::PathBuf> {
    Ok(hermes_home()?.join("state.db"))
}

fn get_db() -> napi::Result<Connection> {
    let db_path = state_db_path()?;
    if !db_path.exists() {
        return Err(napi::Error::from_reason("state.db not found"));
    }
    let conn = Connection::open_with_flags(&db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| napi::Error::from_reason(format!("DB open error: {e}")))?;
    conn.execute_batch("PRAGMA busy_timeout = 5000;")
        .map_err(|e| napi::Error::from_reason(format!("PRAGMA error: {e}")))?;
    Ok(conn)
}

fn get_db_writable() -> napi::Result<Connection> {
    let db_path = state_db_path()?;
    if !db_path.exists() {
        return Err(napi::Error::from_reason("state.db not found"));
    }
    let conn = Connection::open(&db_path)
        .map_err(|e| napi::Error::from_reason(format!("DB open error: {e}")))?;
    conn.execute_batch("PRAGMA busy_timeout = 5000;")
        .map_err(|e| napi::Error::from_reason(format!("PRAGMA error: {e}")))?;
    Ok(conn)
}

#[napi(object)]
#[derive(serde::Serialize)]
pub struct CachedSession {
    pub id: String,
    pub title: String,
    pub started_at: i64,
    pub source: String,
    pub message_count: i32,
    pub model: String,
}

fn read_session_cache(dir: &std::path::Path) -> napi::Result<serde_json::Value> {
    let cache_path = dir.join("sessions.json");
    if !cache_path.exists() {
        return Ok(serde_json::json!({}));
    }
    let data = wrap_err!(fs::read_to_string(&cache_path))?;
    serde_json::from_str(&data).map_err(|e| napi::Error::from_reason(e.to_string()))
}

fn write_session_cache(dir: &std::path::Path, sessions: &[CachedSession], last_sync: u64) -> napi::Result<()> {
    let cache_data = serde_json::json!({ "sessions": sessions, "lastSync": last_sync });
    let json = serde_json::to_string_pretty(&cache_data)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    wrap_err!(fs::write(dir.join("sessions.json"), json))
}

fn generate_title(db: &Connection, session_id: &str) -> String {
    let title: String = db.query_row(
        "SELECT content FROM messages WHERE session_id = ? AND role = 'user' AND content IS NOT NULL ORDER BY timestamp, id LIMIT 1",
        [session_id], |row| row.get(0)
    ).unwrap_or_else(|_| "New conversation".to_string());
    if title.len() > 50 {
        format!("{}...", &title[..title.ceil_char_boundary(50)])
    } else {
        title
    }
}

fn fill_missing_titles(db: &Connection, sessions: &mut [CachedSession]) {
    for session in sessions.iter_mut() {
        if session.title.is_empty() {
            session.title = generate_title(db, &session.id);
        }
    }
}

fn query_sessions_since(db: &Connection, since: i64) -> napi::Result<Vec<CachedSession>> {
    let mut stmt = db.prepare(
        "SELECT s.id, s.started_at, s.source, s.message_count, s.model, s.title
         FROM sessions s WHERE s.started_at > ? ORDER BY s.started_at DESC"
    ).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let rows = stmt.query_map([since], |row| {
        Ok(CachedSession {
            id: row.get(0)?,
            started_at: row.get(1)?,
            source: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            message_count: row.get::<_, Option<i32>>(3)?.unwrap_or(0),
            model: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            title: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
        })
    }).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let mut sessions = Vec::new();
    for row in rows {
        let session = row.map_err(|e| napi::Error::from_reason(format!("Row parse error: {e}")))?;
        sessions.push(session);
    }
    Ok(sessions)
}

#[napi]
pub fn list_cached_sessions(limit: Option<u32>, profile: Option<String>) -> napi::Result<Vec<CachedSession>> {
    let limit = limit.unwrap_or(50) as usize;
    let dir = desktop_dir(profile.as_deref())?;
    let cache = read_session_cache(&dir)?;
    let sessions = cache
        .get("sessions")
        .and_then(|s| s.as_array())
        .map(|arr| {
            arr.iter().filter_map(|v| {
                Some(CachedSession {
                    id: v.get("id")?.as_str()?.to_string(),
                    title: v.get("title")?.as_str().unwrap_or("").to_string(),
                    started_at: v.get("startedAt")?.as_i64()?,
                    source: v.get("source")?.as_str().unwrap_or("").to_string(),
                    message_count: v.get("messageCount")?.as_i64()? as i32,
                    model: v.get("model")?.as_str().unwrap_or("").to_string(),
                })
            }).collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(sessions.into_iter().take(limit).collect())
}

#[napi]
pub fn sync_session_cache(profile: Option<String>) -> napi::Result<Vec<CachedSession>> {
    let dir = desktop_dir(profile.as_deref())?;
    wrap_err!(fs::create_dir_all(&dir))?;
    let db = get_db()?;
    let last_sync = read_session_cache(&dir)?
        .get("lastSync").and_then(|v| v.as_i64()).unwrap_or(0);
    let since = if last_sync > 0 { last_sync - 300 } else { 0 };
    let mut sessions = query_sessions_since(&db, since)?;
    fill_missing_titles(&db, &mut sessions);
    sessions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?
        .as_secs();
    write_session_cache(&dir, &sessions, now)?;
    Ok(sessions)
}

#[napi]
pub fn delete_session(session_id: String, profile: Option<String>) -> napi::Result<()> {
    let db_path = state_db_path()?;
    if db_path.exists() {
        let conn = get_db_writable()?;
        conn.execute("DELETE FROM messages WHERE session_id = ?", [&session_id])
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        conn.execute("DELETE FROM sessions WHERE id = ?", [&session_id])
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }
    let cache_path = desktop_dir(profile.as_deref())?.join("sessions.json");
    if cache_path.exists() {
        let data = wrap_err!(fs::read_to_string(&cache_path))?;
        let mut cache: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        if let Some(sessions) = cache.get_mut("sessions").and_then(|s| s.as_array_mut()) {
            sessions.retain(|s| s.get("id").and_then(|v| v.as_str()).map(|id| id != session_id).unwrap_or(true));
        }
        let json = serde_json::to_string_pretty(&cache)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        wrap_err!(fs::write(&cache_path, json))?;
    }
    Ok(())
}

#[napi]
pub fn update_session_title(session_id: String, title: String, profile: Option<String>) -> napi::Result<()> {
    let cache_path = desktop_dir(profile.as_deref())?.join("sessions.json");
    if cache_path.exists() {
        let data = wrap_err!(fs::read_to_string(&cache_path))?;
        let mut cache: serde_json::Value = serde_json::from_str(&data)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        if let Some(sessions) = cache.get_mut("sessions").and_then(|s| s.as_array_mut()) {
            for session in sessions.iter_mut() {
                if session.get("id").and_then(|v| v.as_str()) == Some(session_id.as_str()) {
                    session["title"] = serde_json::Value::String(title.clone());
                }
            }
        }
        let json = serde_json::to_string_pretty(&cache)
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;
        wrap_err!(fs::write(&cache_path, json))?;
    }
    Ok(())
}

#[napi(object)]
pub struct SearchResult {
    pub session_id: String,
    pub title: String,
    pub started_at: i64,
    pub source: String,
    pub message_count: i32,
    pub model: String,
    pub snippet: String,
}

#[napi]
pub fn search_sessions(query: String) -> napi::Result<Vec<SearchResult>> {
    let db = get_db()?;
    let mut stmt = db.prepare(
        "SELECT s.id, s.title, s.started_at, s.source, s.message_count, s.model,
                snippet(messages_fts, 0, '<<', '>>', '...', 32) as snippet
         FROM messages_fts m JOIN sessions s ON s.id = m.session_id
         WHERE messages_fts MATCH ? GROUP BY s.id ORDER BY s.started_at DESC LIMIT 50"
    ).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let rows = stmt.query_map([&query], |row| {
        Ok(SearchResult {
            session_id: row.get(0)?,
            title: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
            started_at: row.get(2)?,
            source: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
            message_count: row.get::<_, Option<i32>>(4)?.unwrap_or(0),
            model: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            snippet: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
        })
    }).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let mut results = Vec::new();
    for row in rows {
        let r = row.map_err(|e| napi::Error::from_reason(format!("Row parse error: {e}")))?;
        results.push(r);
    }
    Ok(results)
}
