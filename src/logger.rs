pub mod macro_log_output;

use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

pub fn init(){
  let stdout = ConsoleAppender::builder()
    // .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S%.3f)},{l},{t},{L},{m}{n}")))
    .encoder(Box::new(PatternEncoder::new("{l},{t},{L},{m}{n}")))
    .build();

  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .build(Root::builder().appender("stdout").build(LevelFilter::Info))
    .unwrap();

  let _handle = log4rs::init_config(config).unwrap();
}
