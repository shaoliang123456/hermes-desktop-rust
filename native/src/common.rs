use std::path::PathBuf;

#[macro_export]
macro_rules! wrap_err {
    ($result:expr) => {
        $result.map_err(|e| napi::Error::from_reason(e.to_string()))
    };
}

pub fn hermes_home() -> napi::Result<PathBuf> {
    std::env::var("HERMES_HOME")
        .ok()
        .map(|p| PathBuf::from(p.trim()))
        .filter(|p| !p.as_os_str().is_empty())
        .map(Ok)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .map(|h| h.join(".hermes"))
                .ok_or_else(|| napi::Error::from_reason("Cannot determine home directory"))
        })
}

pub fn desktop_dir(profile: Option<&str>) -> napi::Result<PathBuf> {
    let home = hermes_home()?;
    match profile {
        Some(p) if p != "default" => Ok(home.join("profiles").join(p).join("desktop")),
        _ => Ok(home.join("desktop")),
    }
}
