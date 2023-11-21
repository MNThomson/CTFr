use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::info;

pub async fn init_dbpool() -> Result<Pool<Postgres>> {
    return PgPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://user:pass@localhost/database")
        .await
        .context("Could not connect to database (with URL)");
}

pub async fn setup_database(db: &Pool<Postgres>) {
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
