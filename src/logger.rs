use crate::config::CONFIG;
use log::{debug, LevelFilter};
use log4rs;
use log4rs::config::Config;

pub fn init_logger() {
  // init log config
  let mut config: Config =
    log4rs::load_config_file("config/log4rs.yaml", Default::default()).unwrap();
  config
    .root_mut()
    .set_level(level_filter(&CONFIG.logger.level));
  log4rs::init_config(config).unwrap();
  debug!("init log config file...");
}

fn level_filter(level: &String) -> LevelFilter {
  match level.as_ref() {
    "debug" => LevelFilter::Debug,
    "info" => LevelFilter::Info,
    "trace" => LevelFilter::Trace,
    "error" => LevelFilter::Error,
    "warn" => LevelFilter::Warn,
    "off" => LevelFilter::Off,
    _ => LevelFilter::Off,
  }
}
