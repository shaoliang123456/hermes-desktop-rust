use napi_derive::napi;

#[napi(object)]
pub struct Profile { pub name: String, pub is_active: bool }

#[napi]
pub fn list_profiles(active: Option<String>) -> napi::Result<Vec<Profile>> {
    let home = std::env::var("HERMES_HOME").ok().map(std::path::PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".hermes"));
    let active_name = active.unwrap_or_else(|| "default".to_string());
    let mut profiles = vec![Profile { name: "default".to_string(), is_active: active_name == "default" }];
    let profiles_dir = home.join("profiles");
    if profiles_dir.exists() {
        for entry in std::fs::read_dir(&profiles_dir).map_err(|e| napi::Error::from_reason(e.to_string()))? {
            let entry = entry.map_err(|e| napi::Error::from_reason(e.to_string()))?;
            if entry.file_type().map_err(|e| napi::Error::from_reason(e.to_string()))?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                profiles.push(Profile { is_active: name == active_name, name });
            }
        }
    }
    Ok(profiles)
}
