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
use log::debug;
use serde::Deserialize;
use std::env;

#[derive(Clone, Deserialize, Debug)]
pub struct Database {
    pub driver: DatabaseConnection,
    pub url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct JWT {
    pub expiration: i64,
    pub key: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Session {
    pub name: String,
    pub key: String,
    pub secure: bool,
    pub timeout: i64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Server {
    pub url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Logger {
    pub level: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Redis {
    pub url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct IConfig {
    pub active_profile: String,
    pub auth_salt: String,
    pub database: Database,
    pub jwt: JWT,
    pub redis: Redis,
    pub logger: Logger,
    pub server: Server,
    pub session: Session,
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

    let active_profile = env::var("ACTIVE_PROFILE").unwrap_or("development".into());
    debug!("current active profile is: {}", active_profile);
    config
        .merge(File::with_name(&format!("config/{}", active_profile)).required(false))
        .expect("merge env config file error");
    config
        .merge(File::with_name("config/local").required(false))
        .expect("merge local config file error");

    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    config
        .merge(Environment::new().separator("_"))
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
        assert_ne!(config.server.url, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.server.url, "".to_string());
    }
}
