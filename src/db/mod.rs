use std::time::Duration;

use anyhow::Context;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::debug;

pub async fn init_dbpool() -> anyhow::Result<Pool<Sqlite>> {
    return Ok(SqlitePoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        // .connect("sqlite::memory:")
        .connect("sqlite://sqlite.db")
        .await
        .context("Could not connect to database (with URL)")?);
}

pub async fn setup_database(db: &Pool<Sqlite>) {
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

    debug!("database sucessfully setup");
}
