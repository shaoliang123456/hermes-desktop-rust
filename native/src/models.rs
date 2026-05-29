use crate::wrap_err;
use crate::common::hermes_home;
use napi_derive::napi;

#[napi(object)]
#[derive(serde::Deserialize)]
pub struct Model { pub id: String, pub name: String, pub provider: String }

#[napi]
pub fn list_models_cached() -> napi::Result<Vec<Model>> {
    let cache_path = hermes_home()?.join("desktop").join("models.json");
    if cache_path.exists() {
        let data = wrap_err!(std::fs::read_to_string(&cache_path))?;
        return serde_json::from_str(&data).map_err(|e| napi::Error::from_reason(e.to_string()));
    }
    Ok(vec![])
}
