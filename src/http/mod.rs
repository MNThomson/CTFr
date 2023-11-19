use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use sqlx::PgPool;
use tracing::debug;

mod index;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let app = api_router().with_state(AppState { db });

    let addr = SocketAddr::from(([127, 0, 0, 1], 4321));
    debug!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router<AppState> {
    Router::new().merge(index::router())
}
