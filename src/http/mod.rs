use std::{net::SocketAddr, time::Duration};

use anyhow::{Context, Result};
use axum::{
    extract::MatchedPath,
    http::{Request, Version},
    response::Response,
    Router,
};
use sqlx::PgPool;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{debug, info, info_span, Span};

mod index;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

pub async fn serve(db: PgPool) -> Result<()> {
    let app = api_router().with_state(AppState { db }).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                return info_span!(
                    "http_request",
                    "http.request.method" = ?request.method(),
                    "http.request.route" = request.extensions().get::<MatchedPath>().map(MatchedPath::as_str),
                    "http.request.target" = request.uri().to_string(),
                    "http.flavor" = match request.version() {
                        Version::HTTP_09 => "0.9".to_string(),
                        Version::HTTP_10 => "1.0".to_string(),
                        Version::HTTP_11 => "1.1".to_string(),
                        Version::HTTP_2 => "2.0".to_string(),
                        Version::HTTP_3 => "3.0".to_string(),
                        _ => "Unknown".to_string(),
                    },
                    "http.request.content_length" = tracing::field::Empty, // TODO: Add content length value
                    "http.response.status_code" = tracing::field::Empty,
                    "error" = tracing::field::Empty,
                )
            })
            .on_request(|_request: &Request<_>, _span: &Span| {})
            .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                _span.record("http.response.status_code", _response.status().as_u16());
                debug!("Request served");
            })
            .on_failure(
                |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                    _span.record("error", _error.to_string());
                    debug!("Request errored");
                },
            ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4321));
    info!("listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router<AppState> {
    return Router::new().merge(index::router());
}
