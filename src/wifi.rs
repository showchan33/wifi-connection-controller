use std::process::Command;
use anyhow::{Result, anyhow};

use log::{error, info, warn, debug};
use function_name::named;
use super::logger::macro_log_output::*;

/// Attempts to connect to a wireless network with a given SSID and password.
#[named]
pub fn connect(ssid: &str, password: &str, interface: &str) -> Result<()> {

  let output = Command::new("nmcli")
  .args(&[
    "device", "wifi",
    "connect", ssid,
    "password", password,
    "ifname", interface,
  ])
  .output()?;

  match output.status.code() {
    Some(code) => {
      match code {
        0 => {
          let stdout_string = std::str::from_utf8(&output.stdout).unwrap();
          log_out!("info", format!("{:?}", stdout_string));
        },
        _ => {
          let stderr_string = std::str::from_utf8(&output.stderr).unwrap();
          log_out!("error", format!("code = {} : {:?}", code, stderr_string));
          return Err(anyhow!(""));
        }
      }
    },
    None => {
      let stderr_string = std::str::from_utf8(&output.stderr).unwrap();
      log_out!("error", format!("{:?}", stderr_string));
      return Err(anyhow!(""));
    }
  }

  Ok(())
}
