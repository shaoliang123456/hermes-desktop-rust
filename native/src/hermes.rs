use napi_derive::napi;
use std::process::Command;

#[napi(object)]
pub struct HermesStatus {
    pub installed: bool,
    pub version: Option<String>,
    pub path: Option<String>,
}

#[napi]
pub fn check_hermes_installation() -> napi::Result<HermesStatus> {
    match Command::new("hermes").arg("--version").output() {
        Ok(out) if out.status.success() => {
            let version = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let path = Command::new("which").arg("hermes").output().ok()
                .and_then(|o| if o.status.success() { Some(String::from_utf8_lossy(&o.stdout).trim().to_string()) } else { None });
            Ok(HermesStatus { installed: true, version: Some(version), path })
        }
        _ => Ok(HermesStatus { installed: false, version: None, path: None }),
    }
}
