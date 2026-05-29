use crate::wrap_err;
use crate::common::hermes_home;
use napi_derive::napi;
use std::fs;

#[napi]
pub fn get_config() -> napi::Result<serde_json::Value> {
    let config_path = hermes_home()?.join("config.yaml");
    if !config_path.exists() { return Ok(serde_json::json!({})); }
    let content = wrap_err!(fs::read_to_string(&config_path))?;
    serde_yaml::from_str(&content).map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn save_config(config: serde_json::Value) -> napi::Result<()> {
    let config_path = hermes_home()?.join("config.yaml");
    let yaml = serde_yaml::to_string(&config).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    wrap_err!(fs::write(&config_path, yaml))
}
