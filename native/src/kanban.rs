use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::process::Command;

#[napi(object)]
pub struct KanbanResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

fn run_hermes(args: &[&str], profile: Option<&str>) -> napi::Result<String> {
    let mut cmd = Command::new("hermes");
    for arg in args { cmd.arg(arg); }
    if let Some(p) = profile { cmd.arg("--profile").arg(p); }
    cmd.arg("--json");
    let output = cmd.output().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound { napi::Error::from_reason("Hermes not installed") }
        else { napi::Error::from_reason(e.to_string()) }
    })?;
    if !output.status.success() {
        return Err(napi::Error::from_reason(String::from_utf8_lossy(&output.stderr).to_string()));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn kanban_cmd(sub: &str, extra: &[&str], profile: Option<String>) -> napi::Result<KanbanResponse> {
    let mut args = vec!["kanban".to_string(), sub.to_string()];
    for e in extra { args.push(e.to_string()); }
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match run_hermes(&refs, profile.as_deref()) {
        Ok(json) => {
            let parsed: serde_json::Value = serde_json::from_str(&json)
                .map_err(|e| napi::Error::from_reason(e.to_string()))?;
            Ok(KanbanResponse { success: true, data: Some(parsed), error: None })
        }
        Err(e) => Ok(KanbanResponse { success: false, data: None, error: Some(e.reason) }),
    }
}

#[napi]
pub fn kanban_list_boards(include_archived: Option<bool>, profile: Option<String>) -> napi::Result<KanbanResponse> {
    let mut extra = vec![];
    if include_archived.unwrap_or(false) { extra.push("--archived".to_string()); }
    kanban_cmd("list-boards", &extra, profile)
}

#[napi]
pub fn kanban_list_tasks(board: Option<String>, include_archived: Option<bool>, profile: Option<String>) -> napi::Result<KanbanResponse> {
    let mut extra = vec![];
    if let Some(b) = board { extra.extend(["--board".to_string(), b]); }
    if include_archived.unwrap_or(false) { extra.push("--archived".to_string()); }
    kanban_cmd("list-tasks", &extra, profile)
}

#[napi]
pub fn kanban_create_task(title: String, body: Option<String>, priority: Option<i32>, board: Option<String>, profile: Option<String>) -> napi::Result<KanbanResponse> {
    let mut extra = vec![title];
    if let Some(b) = body { extra.extend(["--body".to_string(), b]); }
    if let Some(p) = priority { extra.extend(["--priority".to_string(), p.to_string()]); }
    if let Some(b) = board { extra.extend(["--board".to_string(), b]); }
    kanban_cmd("create-task", &extra, profile)
}

#[napi]
pub fn kanban_create_board(slug: String, name: Option<String>, profile: Option<String>) -> napi::Result<KanbanResponse> {
    let mut extra = vec![slug];
    if let Some(n) = name { extra.extend(["--name".to_string(), n]); }
    kanban_cmd("create-board", &extra, profile)
}

#[napi]
pub fn kanban_dispatch(board: Option<String>, profile: Option<String>) -> napi::Result<KanbanResponse> {
    let extra = board.map(|b| vec!["--board".to_string(), b]).unwrap_or_default();
    kanban_cmd("dispatch", &extra, profile)
}

#[napi]
pub fn kanban_move_task(task_id: String, target: String, profile: Option<String>) -> napi::Result<KanbanResponse> {
    kanban_cmd("move", &[task_id, target], profile)
}

#[napi]
pub fn kanban_complete_task(task_id: String, profile: Option<String>) -> napi::Result<KanbanResponse> {
    kanban_cmd("complete", &[task_id], profile)
}

#[napi]
pub fn kanban_block_task(task_id: String, reason: Option<String>, profile: Option<String>) -> napi::Result<KanbanResponse> {
    let mut extra = vec![task_id];
    if let Some(r) = reason { extra.extend(["--reason".to_string(), r]); }
    kanban_cmd("block", &extra, profile)
}

#[napi]
pub fn kanban_unblock_task(task_id: String, profile: Option<String>) -> napi::Result<KanbanResponse> {
    kanban_cmd("unblock", &[task_id], profile)
}

#[napi]
pub fn kanban_archive_task(task_id: String, profile: Option<String>) -> napi::Result<KanbanResponse> {
    kanban_cmd("archive", &[task_id], profile)
}

#[napi]
pub fn kanban_specify_task(task_id: String, profile: Option<String>) -> napi::Result<KanbanResponse> {
    kanban_cmd("specify", &[task_id], profile)
}

#[napi]
pub fn kanban_reclaim_task(task_id: String, profile: Option<String>) -> napi::Result<KanbanResponse> {
    kanban_cmd("reclaim", &[task_id], profile)
}
