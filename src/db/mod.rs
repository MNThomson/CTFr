use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::Pool;

#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!(r#"feature "postgres" and feature "sqlite" cannot be enabled at the same time"#);

#[cfg(feature = "postgres")]
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};

#[cfg(feature = "postgres")]
pub type DB = Postgres;

#[cfg(feature = "postgres")]
pub type DbPool = PgPool;

#[cfg(feature = "sqlite")]
use sqlx::{sqlite::SqlitePoolOptions, Sqlite, SqlitePool};

#[cfg(feature = "sqlite")]
pub type DB = Sqlite;

#[cfg(feature = "sqlite")]
pub type DbPool = SqlitePool;

use tracing::info;

pub async fn init_dbpool() -> Result<DbPool> {
    #[cfg(feature = "postgres")]
    return PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://user:pass@localhost/database")
        .await
        .context("Could not connect to database (with URL)");

    #[cfg(feature = "sqlite")]
    return SqlitePoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .connect("sqlite://sqlite.db")
        .await
        .context("Could not connect to database (with URL)");
}

pub async fn setup_database(db: &Pool<DB>) {
    if cfg!(debug_assertions) {
        sqlx::query(include_str!("sql/1_setup.down.sql"))
            .execute(db)
            .await
            .unwrap();
    }

    sqlx::query(include_str!("sql/1_setup.up.sql"))
        .execute(db)
        .await
        .unwrap();

    if cfg!(debug_assertions) {
        sqlx::query(include_str!("sql/2_test_data.up.sql"))
            .execute(db)
            .await
            .unwrap();
    }

    info!("database sucessfully setup");
}
