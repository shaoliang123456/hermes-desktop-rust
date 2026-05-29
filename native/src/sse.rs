use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct SseEvent { pub event: Option<String>, pub data: String, pub id: Option<String> }

#[napi]
pub fn parse_sse_chunk(chunk: Buffer) -> napi::Result<Vec<SseEvent>> {
    let text = String::from_utf8_lossy(&chunk);
    let mut events = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(':') { continue; }
        if let Some(data) = line.strip_prefix("data: ") {
            events.push(SseEvent { event: None, data: data.to_string(), id: None });
        } else if let Some(event) = line.strip_prefix("event: ") {
            if let Some(last) = events.last_mut() { last.event = Some(event.to_string()); }
        } else if let Some(id) = line.strip_prefix("id: ") {
            if let Some(last) = events.last_mut() { last.id = Some(id.to_string()); }
        }
    }
    Ok(events)
}
