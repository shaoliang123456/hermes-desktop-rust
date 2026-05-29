use napi_derive::napi;

#[napi(object)]
pub struct ParsedUsage {
    pub prompt_tokens: f64,
    pub completion_tokens: f64,
    pub total_tokens: f64,
    pub cost: Option<f64>,
    pub rate_limit_remaining: Option<f64>,
    pub rate_limit_reset: Option<f64>,
}

#[napi(object)]
pub struct SseChunkResult {
    pub done: bool,
    pub has_content: bool,
    pub text: Option<String>,
    pub tool_progress: Option<String>,
    pub usage: Option<ParsedUsage>,
    pub error: Option<String>,
}

#[napi(object)]
pub struct SseBlock {
    pub event_type: String,
    pub data: String,
}

fn extract_delta_content(delta: &serde_json::Value) -> Option<String> {
    delta.get("content")?.as_str().map(|s| s.to_string())
}

fn extract_usage(parsed: &serde_json::Value) -> Option<ParsedUsage> {
    let usage = parsed.get("usage")?;
    Some(ParsedUsage {
        prompt_tokens: usage.get("prompt_tokens").and_then(|v| v.as_f64()).unwrap_or(0.0),
        completion_tokens: usage.get("completion_tokens").and_then(|v| v.as_f64()).unwrap_or(0.0),
        total_tokens: usage.get("total_tokens").and_then(|v| v.as_f64()).unwrap_or(0.0),
        cost: usage.get("cost").and_then(|v| v.as_f64()),
        rate_limit_remaining: usage.get("rate_limit_remaining").and_then(|v| v.as_f64()),
        rate_limit_reset: usage.get("rate_limit_reset").and_then(|v| v.as_f64()),
    })
}

#[napi]
pub fn parse_sse_chunk(raw: String) -> napi::Result<SseChunkResult> {
    let data = raw.trim();

    if data == "[DONE]" {
        return Ok(SseChunkResult {
            done: true,
            has_content: false,
            text: None,
            tool_progress: None,
            usage: None,
            error: None,
        });
    }

    let parsed: serde_json::Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(_) => {
            return Ok(SseChunkResult {
                done: false,
                has_content: false,
                text: None,
                tool_progress: None,
                usage: None,
                error: None,
            });
        }
    };

    if let Some(err) = parsed.get("error") {
        let msg = err.get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown error")
            .to_string();
        return Ok(SseChunkResult {
            done: false,
            has_content: false,
            text: None,
            tool_progress: None,
            usage: None,
            error: Some(msg),
        });
    }

    let usage = extract_usage(&parsed);

    let delta = parsed.get("choices")
        .and_then(|c| c.get("0"))
        .and_then(|c| c.get("delta"));

    if let Some(delta) = delta {
        if let Some(content) = extract_delta_content(delta) {
            let trimmed = content.trim();
            if trimmed.starts_with('`') && trimmed.ends_with('`') && trimmed.chars().filter(|c| *c == ' ').count() == 1 {
                let inner = trimmed.trim_matches('`');
                return Ok(SseChunkResult {
                    done: false,
                    has_content: false,
                    text: None,
                    tool_progress: Some(inner.to_string()),
                    usage,
                    error: None,
                });
            }
            return Ok(SseChunkResult {
                done: false,
                has_content: true,
                text: Some(content),
                tool_progress: None,
                usage,
                error: None,
            });
        }
    }

    Ok(SseChunkResult {
        done: false,
        has_content: false,
        text: None,
        tool_progress: None,
        usage,
        error: None,
    })
}

#[napi]
pub fn parse_sse_block(block: String) -> napi::Result<Option<SseBlock>> {
    let mut event_type = String::new();
    let mut data_line = String::new();

    for line in block.lines() {
        if let Some(rest) = line.strip_prefix("event: ") {
            event_type = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("data: ") {
            data_line = rest.to_string();
        }
    }

    if data_line.is_empty() {
        return Ok(None);
    }

    Ok(Some(SseBlock {
        event_type,
        data: data_line,
    }))
}

#[napi]
pub fn parse_sse_stream(raw_stream: String) -> napi::Result<Vec<SseChunkResult>> {
    let mut results = Vec::new();

    for block in raw_stream.split("\n\n") {
        if block.trim().is_empty() {
            continue;
        }

        let parsed_block = parse_sse_block(block.to_string())?;
        if let Some(b) = parsed_block {
            let chunk = parse_sse_chunk(b.data)?;
            results.push(chunk);
        }
    }

    Ok(results)
}
