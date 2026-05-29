use napi_derive::napi;
use std::fs;
use std::path::PathBuf;

fn hermes_home() -> PathBuf {
    std::env::var("HERMES_HOME").ok().map(PathBuf::from).filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".hermes"))
}

#[napi]
pub fn get_config() -> napi::Result<serde_json::Value> {
    let config_path = hermes_home().join("config.yaml");
    if !config_path.exists() { return Ok(serde_json::json!({})); }
    let content = fs::read_to_string(&config_path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    serde_yaml::from_str(&content).map_err(|e| napi::Error::from_reason(e.to_string()))
}

#[napi]
pub fn save_config(config: serde_json::Value) -> napi::Result<()> {
    let config_path = hermes_home().join("config.yaml");
    let yaml = serde_yaml::to_string(&config).map_err(|e| napi::Error::from_reason(e.to_string()))?;
    fs::write(&config_path, yaml).map_err(|e| napi::Error::from_reason(e.to_string()))
}
