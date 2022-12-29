use std::net::IpAddr;
use std::{thread, time, fs, str};

use ping::ping;
mod wifi;
// use wifi;

use anyhow::{Result, anyhow};

use base64::{Engine as _, engine::general_purpose};

use log::{error, info, warn, debug};
use function_name::named;

mod logger;
use logger::macro_log_output::*;

mod argparse;
// use argparse::Args;

#[named]
fn main() -> Result<()> {

  logger::init();

  let args = argparse::get_args()?;
  let password = get_wifi_password(&args.password, &args.secret)?;

  loop {

    let ping_result = ping(
      args.connection_ip.parse::<IpAddr>().unwrap(),
      None,
      None,
      None,
      None,
      None
    );
  
    log_out!("info", format!("ping result {:?}", &ping_result));

    match ping_result {
      Err(_) => {

        log_out!("info", format!("Failed ping to {:?}", &args.connection_ip));
        log_out!("info", format!("Trying to connect to access point {} on iface {}", &args.ssid, &args.iface));

        let conn = wifi::connect(&args.ssid, &password, &args.iface);

        match conn {
          Ok(_) => {},
          Err(err) => {
            log_out!("error", format!("{:?}", err))
          },
        }      
      },
      Ok(())=> {}
    }

    if args.once {
      break;
    }
    else {
      let sleep_mills = time::Duration::from_millis(1000*args.exec_interval);
      thread::sleep(sleep_mills);
    }
  }

  Ok(())
}

fn get_wifi_password(
  password: &Option<String>,
  secret: &Option<String>,
) -> Result<String> {

  let mut password_org: String;

  match password {
    Some(v) => {
      password_org = v.to_string();
      trim_newline(&mut password_org);
    },
    None => {
      match secret {
        Some(filename) => {
          password_org = fs::read_to_string(filename)?;
          trim_newline(&mut password_org);
        },
        None => {
          return Err(anyhow!("Please specify either password or secret as an argument."));
        }
      }
    }
  }

  let base64_decode = general_purpose::STANDARD
    .decode(&password_org)?;

  let mut password_ret: String;

  match str::from_utf8(&base64_decode) {
    Ok(s) => {
      password_ret = s.to_string();
      trim_newline(&mut password_ret);
    }
    Err(_) => {
      password_ret = password_org;
    }
  }

  Ok(password_ret)
}

fn trim_newline(s: &mut String) {
  if s.ends_with('\n') {
    s.pop();
  }
}

#[test]
fn it_works_get_wifi_password_from_string() {
  assert_eq!(
    get_wifi_password(&Some("abcd".to_string()), &None).unwrap(),
    "abcd".to_string()
  );
  assert_eq!(
    get_wifi_password(&Some("abcd\n".to_string()), &None).unwrap(),
    "abcd".to_string()
  );
  assert_eq!(
    get_wifi_password(&Some("YWJjZA==".to_string()), &None).unwrap(),
    "abcd".to_string()
  );
  assert_eq!(
    get_wifi_password(&Some("YWJjZAo=".to_string()), &None).unwrap(),
    "abcd".to_string()
  );
}

#[test]
fn it_works_get_wifi_password_from_file() {
  assert_eq!(
    get_wifi_password(&None, &Some("./test/abcd.txt".to_string())).unwrap(),
    "abcd".to_string()
  );
  assert_eq!(
    get_wifi_password(&None, &Some("./test/abcd_nl.txt".to_string())).unwrap(),
    "abcd".to_string()
  );
  assert_eq!(
    get_wifi_password(&None, &Some("./test/base64.txt".to_string())).unwrap(),
    "abcd".to_string()
  );
  assert_eq!(
    get_wifi_password(&None, &Some("./test/base64_nl.txt".to_string())).unwrap(),
    "abcd".to_string()
  );
}
