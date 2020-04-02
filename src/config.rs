//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use crate::database::DatabaseConnection;
use config::{Config, Environment, File};
use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Clone, Deserialize, Debug)]
pub struct IConfig {
    pub auth_salt: String,
    pub database: DatabaseConnection,
    pub database_url: String,
    pub jwt_expiration: i64,
    pub jwt_key: String,
    pub redis_url: String,
    // pub rust_backtrace: u8,
    pub rust_log: String,
    pub logger_level: String,
    pub server: String,
    pub session_key: String,
    pub session_name: String,
    pub session_secure: bool,
    pub session_timeout: i64,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: IConfig = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> IConfig {
    dotenv().ok();
    let mut config = Config::new();

    config
        .merge(File::with_name("config/default"))
        .expect("merge default config file error");

    let env = env::var("RUN_MODE").unwrap_or("development".into());
    config
        .merge(File::with_name(&format!("config/{}", env)).required(false))
        .expect("merge env config file error");
    config
        .merge(File::with_name("config/local").required(false))
        .expect("merge local config file error");

    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    config
        .merge(Environment::new())
        .expect("merge env config error");

    match config.try_into() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.server, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.server, "".to_string());
    }
}
