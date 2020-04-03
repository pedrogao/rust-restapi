//! Database-related functions
use crate::config::{IConfig, CONFIG};
use actix_web::web;
use diesel::{
    r2d2::{ConnectionManager, PoolError},
    sqlite::SqliteConnection,
    Connection,
};

#[serde(untagged)]
#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase")]
pub enum DatabaseConnection {
    Cockroach,
    Mysql,
    Postgres,
    Sqlite,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type SqlitePool = Pool<SqliteConnection>;

#[cfg(feature = "sqlite")]
pub type PoolType = SqlitePool;

#[derive(Clone)]
pub enum InferPool {
    Sqlite(SqlitePool),
}

impl InferPool {
    pub fn init_pool(config: IConfig) -> Result<Self, r2d2::Error> {
        // match config.database {
        //     DatabaseConnection::Sqlite => {
        //         init_pool::<SqliteConnection>(config).map(InferPool::Sqlite)
        //     }
        // }
        init_pool::<SqliteConnection>(config).map(InferPool::Sqlite).map_err(Into::into)
    }
}

pub fn init_pool<T>(config: IConfig) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    Pool::builder().build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    let pool = InferPool::init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    match pool {
        InferPool::Sqlite(sqlite_pool) => cfg.data(sqlite_pool),
    };
}
