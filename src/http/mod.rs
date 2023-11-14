use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;
use tracing::debug;

mod index;

pub async fn serve() -> anyhow::Result<()> {
    let app = api_router();

    let addr = SocketAddr::from(([127, 0, 0, 1], 4321));
    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    Router::new().merge(index::router())
    // .merge(articles::router())
}
