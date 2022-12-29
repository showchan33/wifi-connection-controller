use std::net::Ipv4Addr;
use clap::{Parser, ArgGroup};
use anyhow::Result;

/// Check if the network is connected, and if it is not, try to connect to Wi-Fi.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(group(
  ArgGroup::new("secrets")
      .required(true)
      .args(["password", "secret"]),
))]
pub struct Args {
  /// IP address for checking network connetcion
  #[arg(short, long, default_value_t = String::from("192.168.1.1"))]
  pub connection_ip: String,

  /// network interface for connecting Wi-Fi
  #[arg(short, long)]
  pub iface: String,

  /// SSID of Wi-Fi connection
  #[arg(short, long)]
  pub ssid: String,

  /// password of Wi-Fi access point (Supports plain text, Base64)
  #[arg(short, long)]
  pub password: Option<String>,

  /// File name that stored the password (Supports plain text, Base64)
  #[arg(long)]
  pub secret: Option<String>,

  /// execution interval (sec)
  #[arg(short, long, default_value_t = 10)]
  pub exec_interval: u64,

  /// set for execution only once.
  #[arg(short, long, default_value_t = false)]
  pub once: bool,

}

/// Retrieve a value from the command-line arguments.
pub fn get_args() -> Result<Args> {
  let args = Args::parse();

  let _connection_ip_format_check = args.connection_ip.parse::<Ipv4Addr>()?;

  Ok(args)
}
