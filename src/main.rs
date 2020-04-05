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

use crate::server::server;

mod auth;
mod logger;
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
mod websocket;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();
    server().await
}
