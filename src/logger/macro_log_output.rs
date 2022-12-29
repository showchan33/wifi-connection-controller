/// Macro for log output
macro_rules! log_out {
  ($level:expr, $msg:expr) => {
    match $level {
    "debug"|"DEBUG" => debug!("#[{}] {}", function_name!(), $msg),
    "info"|"INFO" => info!("#[{}] {}", function_name!(), $msg),
    "warn"|"WARN" => warn!("#[{}] {}", function_name!(), $msg),
    "error"|"ERROR" => error!("#[{}] {}", function_name!(), $msg),
    _ => error!("#[{}] {}", function_name!(), format!("The log level \"{}\" cannot be specified.", $level)),
    }
  };
}

pub(crate) use log_out;
