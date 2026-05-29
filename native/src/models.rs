use napi_derive::napi;

#[napi(object)]
#[derive(serde::Deserialize)]
pub struct Model { pub id: String, pub name: String, pub provider: String }

#[napi]
pub fn list_models_cached() -> napi::Result<Vec<Model>> {
    let home = std::env::var("HERMES_HOME").ok().map(std::path::PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".hermes"));
    let cache_path = home.join("desktop").join("models.json");
    if cache_path.exists() {
        let data = std::fs::read_to_string(&cache_path).map_err(|e| napi::Error::from_reason(e.to_string()))?;
        return serde_json::from_str(&data).map_err(|e| napi::Error::from_reason(e.to_string()));
    }
    Ok(vec![])
}
