#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
// #[macro_use]
// extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use crate::config::CONFIG;
use crate::server::server;
use log::{info, LevelFilter};
use log4rs;
use log4rs::config::Config;

mod auth;
// mod cache;
mod config;
mod database;
mod errors;
mod extractors;
pub mod handlers;
mod helpers;
mod middleware;
mod models;
mod routes;
mod schema;
mod server;
mod state;
mod tests;
mod validate;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // init log config
    let mut config: Config =
        log4rs::load_config_file("config/log4rs.yaml", Default::default()).unwrap();
    config
        .root_mut()
        .set_level(level_filter(&CONFIG.logger_level));
    log4rs::init_config(config).unwrap();
    info!("init log config file...");
    server().await
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
