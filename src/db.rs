use once_cell::sync::Lazy;
use postgres::{Client, NoTls};
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use std::env;

pub type PgPool = Pool<PostgresConnectionManager<NoTls>>;
pub type PgPooledConn = PooledConnection<PostgresConnectionManager<NoTls>>;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DbInitError {
    #[error("DATABASE_URL must be set, e.g. postgres://user:pass@localhost/dbname")]
    MissingUrl,
    #[error("Invalid DATABASE_URL")]
    BadUrl,
    #[error("Failed to build Postgres pool: {0}")]
    PoolInit(String),
}

static POOL: Lazy<Result<PgPool, DbInitError>> = Lazy::new(|| {
    let url = env::var("DATABASE_URL").map_err(|_| DbInitError::MissingUrl)?;
    let config = url.parse().map_err(|_| DbInitError::BadUrl)?;
    let manager = PostgresConnectionManager::new(config, NoTls);
    r2d2::Pool::builder()
        .max_size(16)
        .build(manager)
        .map_err(|e| DbInitError::PoolInit(e.to_string()))
});

pub fn pool() -> Result<&'static PgPool, DbInitError> {
    match &*POOL {
        Ok(p) => Ok(p),
        Err(e) => Err(e.clone()),
    }
}

pub fn direct_client() -> Result<Client, DbInitError> {
    let url = env::var("DATABASE_URL").map_err(|_| DbInitError::MissingUrl)?;
    Client::connect(&url, NoTls).map_err(|_| DbInitError::BadUrl)
}