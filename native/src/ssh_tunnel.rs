use napi_derive::napi;

#[napi(object)]
pub struct SshTunnelStatus { pub active: bool, pub local_port: Option<u32>, pub error: Option<String> }

#[napi]
pub fn ssh_tunnel_status() -> napi::Result<SshTunnelStatus> {
    Ok(SshTunnelStatus { active: false, local_port: None, error: None })
}
