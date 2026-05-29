use napi_derive::napi;
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;
use std::time::{Duration, Instant};

lazy_static::lazy_static! {
    static ref TUNNEL_STATE: Mutex<Option<TunnelState>> = Mutex::new(None);
}

struct TunnelState {
    process: Child,
    local_port: u32,
}

#[napi(object)]
pub struct SshConfigData {
    pub host: String,
    pub port: u32,
    pub username: String,
    pub key_path: String,
    pub remote_port: u32,
    pub local_port: u32,
}

#[napi(object)]
pub struct SshTunnelStatus {
    pub active: bool,
    pub local_port: Option<u32>,
    pub error: Option<String>,
}

#[napi(object)]
pub struct SshTestResult {
    pub success: bool,
    pub error: Option<String>,
}

fn expand_key_path(key_path: &str) -> PathBuf {
    if key_path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&key_path[2..]);
        }
    }
    PathBuf::from(key_path)
}

fn find_free_port(preferred: u32) -> napi::Result<u32> {
    if let Ok(_) = TcpStream::connect(format!("127.0.0.1:{preferred}")) {
        return Ok(preferred);
    }
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
    let port = listener.local_addr()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?
        .port();
    drop(listener);
    Ok(port as u32)
}

fn wait_for_port(port: u32, timeout_ms: u64) -> napi::Result<()> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    while Instant::now() < deadline {
        if TcpStream::connect(format!("127.0.0.1:{port}")).is_ok() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(300));
    }
    Err(napi::Error::from_reason(format!("SSH tunnel port {port} not ready after {timeout_ms}ms")))
}

fn check_health(port: u32, timeout_ms: u64) -> bool {
    let url = format!("http://127.0.0.1:{port}/health");
    let agent = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(timeout_ms))
        .build();
    let agent = match agent {
        Ok(a) => a,
        Err(_) => return false,
    };
    match agent.get(&url).send() {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

fn build_ssh_args(config: &SshConfigData, local_port: u32) -> Vec<String> {
    let key = expand_key_path(&config.key_path);
    let key_str = key.to_string_lossy().to_string();
    vec![
        "-N".to_string(),
        "-L".to_string(),
        format!("{}:127.0.0.1:{}", local_port, config.remote_port),
        "-p".to_string(),
        config.port.to_string(),
        "-i".to_string(),
        key_str,
        "-o".to_string(),
        "StrictHostKeyChecking=accept-new".to_string(),
        "-o".to_string(),
        "BatchMode=yes".to_string(),
        "-o".to_string(),
        "ExitOnForwardFailure=yes".to_string(),
        "-o".to_string(),
        "ServerAliveInterval=30".to_string(),
        "-o".to_string(),
        "ServerAliveCountMax=3".to_string(),
        format!("{}@{}", config.username, config.host),
    ]
}

#[napi]
pub fn ssh_tunnel_status() -> napi::Result<SshTunnelStatus> {
    let state = TUNNEL_STATE.lock().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    match state.as_ref() {
        Some(s) => {
            let active = check_health(s.local_port, 3000);
            Ok(SshTunnelStatus {
                active,
                local_port: Some(s.local_port),
                error: None,
            })
        }
        None => Ok(SshTunnelStatus {
            active: false,
            local_port: None,
            error: None,
        }),
    }
}

#[napi]
pub fn ssh_tunnel_start(config: SshConfigData) -> napi::Result<SshTunnelStatus> {
    ssh_tunnel_stop()?;

    let local_port = find_free_port(config.local_port)?;
    let args = build_ssh_args(&config, local_port);

    let child = Command::new("ssh")
        .args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| napi::Error::from_reason(format!("Failed to start SSH: {e}")))?;

    let mut state = TUNNEL_STATE.lock().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    *state = Some(TunnelState {
        process: child,
        local_port,
    });

    match wait_for_port(local_port, 12000) {
        Ok(_) => {
            let healthy = check_health(local_port, 20000);
            if !healthy {
                ssh_tunnel_stop()?;
                return Ok(SshTunnelStatus {
                    active: false,
                    local_port: None,
                    error: Some("SSH tunnel started but health check failed".to_string()),
                });
            }
            Ok(SshTunnelStatus {
                active: true,
                local_port: Some(local_port),
                error: None,
            })
        }
        Err(e) => {
            ssh_tunnel_stop()?;
            Ok(SshTunnelStatus {
                active: false,
                local_port: None,
                error: Some(e.reason),
            })
        }
    }
}

#[napi]
pub fn ssh_tunnel_stop() -> napi::Result<SshTunnelStatus> {
    let mut state = TUNNEL_STATE.lock().map_err(|e| napi::Error::from_reason(e.to_string()))?;
    if let Some(mut s) = state.take() {
        let _ = s.process.kill();
    }
    Ok(SshTunnelStatus {
        active: false,
        local_port: None,
        error: None,
    })
}

#[napi]
pub fn ssh_tunnel_test(config: SshConfigData) -> napi::Result<SshTestResult> {
    let local_port = find_free_port(config.local_port).unwrap_or(config.local_port);
    let args = build_ssh_args(&config, local_port);

    let mut child = match Command::new("ssh")
        .args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            return Ok(SshTestResult {
                success: false,
                error: Some(format!("Failed to start SSH: {e}")),
            });
        }
    };

    let result = match wait_for_port(local_port, 15000) {
        Ok(_) => {
            let healthy = check_health(local_port, 5000);
            SshTestResult {
                success: healthy,
                error: if healthy { None } else { Some("Port open but health check failed".to_string()) },
            }
        }
        Err(e) => SshTestResult {
            success: false,
            error: Some(e.reason),
        },
    };

    let _ = child.kill();
    let _ = child.wait();
    Ok(result)
}
