use napi::bindgen_prelude::*;
use napi_derive::napi;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;

fn hermes_home() -> PathBuf {
    std::env::var("HERMES_HOME")
        .ok()
        .map(|p| PathBuf::from(p.trim()))
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".hermes"))
}

fn desktop_dir(profile: Option<String>) -> PathBuf {
    let home = hermes_home();
    match profile {
        Some(p) if p != "default" => home.join("profiles").join(p).join("desktop"),
        _ => home.join("desktop"),
    }
}

fn state_db_path() -> PathBuf {
    hermes_home().join("state.db")
}

fn get_db() -> napi::Result<Connection> {
    let db_path = state_db_path();
    if !db_path.exists() {
        return Err(napi::Error::from_reason("state.db not found"));
    }
    let conn = Connection::open_with_flags(&db_path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| napi::Error::from_reason(format!("DB open error: {e}")))?;
    conn.execute_batch("PRAGMA busy_timeout = 5000;")
        .map_err(|e| napi::Error::from_reason(format!("PRAGMA error: {e}")))?;
    Ok(conn)
}

#[napi(object)]
pub struct CachedSession {
    pub id: String,
    pub title: String,
    pub started_at: i64,
    pub source: String,
    pub message_count: i32,
    pub model: String,
}

#[napi]
pub fn list_cached_sessions(limit: Option<u32>) -> napi::Result<Vec<CachedSession>> {
    let limit = limit.unwrap_or(50) as usize;
    let cache_path = desktop_dir(None).join("sessions.json");
    if !cache_path.exists() {
        return Ok(vec![]);
    }
    let data = fs::read_to_string(&cache_path)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let cache: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
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
    let dir = desktop_dir(profile.clone());
    fs::create_dir_all(&dir).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let db = get_db()?;
    let last_sync = {
        let cache_path = dir.join("sessions.json");
        if cache_path.exists() {
            let data = fs::read_to_string(&cache_path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
            let cache: serde_json::Value = serde_json::from_str(&data).map_err(|e| napi::Error::from_reason(e.to_string()))?;
            cache.get("lastSync").and_then(|v| v.as_i64()).unwrap_or(0)
        } else { 0 }
    };
    let since = if last_sync > 0 { last_sync - 300 } else { 0 };
    let mut stmt = db.prepare(
        "SELECT s.id, s.started_at, s.source, s.message_count, s.model, s.title
         FROM sessions s WHERE s.started_at > ? ORDER BY s.started_at DESC"
    ).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let rows = stmt.query_map([since], |row| {
        Ok(CachedSession {
            id: row.get(0)?,
            title: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
            started_at: row.get(1)?,
            source: row.get::<_, Option<String>>(2)?.unwrap_or_default(),
            message_count: row.get::<_, Option<i32>>(3)?.unwrap_or(0),
            model: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
        })
    }).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let mut sessions: Vec<CachedSession> = rows.filter_map(|r| r.ok()).collect();
    for session in &mut sessions {
        if session.title.is_empty() {
            let title: String = db.query_row(
                "SELECT content FROM messages WHERE session_id = ? AND role = 'user' AND content IS NOT NULL ORDER BY timestamp, id LIMIT 1",
                [&session.id], |row| row.get(0)
            ).unwrap_or_else(|_| "New conversation".to_string());
            session.title = if title.len() > 50 { format!("{}...", &title[..title.ceil_char_boundary(50)]) } else { title };
        }
    }
    sessions.sort_by(|a, b| b.started_at.cmp(&a.started_at));
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    let cache_data = serde_json::json!({ "sessions": &sessions, "lastSync": now });
    fs::write(dir.join("sessions.json"), serde_json::to_string_pretty(&cache_data).unwrap())
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    Ok(sessions)
}

#[napi]
pub fn delete_session(session_id: String) -> napi::Result<()> {
    let db_path = state_db_path();
    if db_path.exists() {
        let conn = Connection::open(&db_path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        conn.execute("DELETE FROM messages WHERE session_id = ?", [&session_id]).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        conn.execute("DELETE FROM sessions WHERE id = ?", [&session_id]).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }
    let cache_path = desktop_dir(None).join("sessions.json");
    if cache_path.exists() {
        let data = fs::read_to_string(&cache_path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        let mut cache: serde_json::Value = serde_json::from_str(&data).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        if let Some(sessions) = cache.get_mut("sessions").and_then(|s| s.as_array_mut()) {
            sessions.retain(|s| s.get("id").and_then(|v| v.as_str()).map(|id| id != session_id).unwrap_or(true));
        }
        fs::write(&cache_path, serde_json::to_string_pretty(&cache).unwrap()).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    }
    Ok(())
}

#[napi]
pub fn update_session_title(session_id: String, title: String) -> napi::Result<()> {
    let cache_path = desktop_dir(None).join("sessions.json");
    if cache_path.exists() {
        let data = fs::read_to_string(&cache_path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        let mut cache: serde_json::Value = serde_json::from_str(&data).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        if let Some(sessions) = cache.get_mut("sessions").and_then(|s| s.as_array_mut()) {
            for session in sessions.iter_mut() {
                if session.get("id").and_then(|v| v.as_str()) == Some(session_id.as_str()) {
                    session["title"] = serde_json::Value::String(title.clone());
                }
            }
        }
        fs::write(&cache_path, serde_json::to_string_pretty(&cache).unwrap()).map_err(|e| napi::Error::from_reason(e.to_string()))?;
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
    Ok(rows.filter_map(|r| r.ok()).collect())
}
