use crate::wrap_err;
use crate::common::hermes_home;
use napi_derive::napi;

#[napi(object)]
pub struct Profile { pub name: String, pub is_active: bool }

#[napi]
pub fn list_profiles(active: Option<String>) -> napi::Result<Vec<Profile>> {
    let home = hermes_home()?;
    let active_name = active.unwrap_or_else(|| "default".to_string());
    let mut profiles = vec![Profile { name: "default".to_string(), is_active: active_name == "default" }];
    let profiles_dir = home.join("profiles");
    if profiles_dir.exists() {
        for entry in wrap_err!(std::fs::read_dir(&profiles_dir))? {
            let entry = wrap_err!(entry)?;
            if wrap_err!(entry.file_type())?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                profiles.push(Profile { is_active: name == active_name, name });
            }
        }
    }
    Ok(profiles)
}
