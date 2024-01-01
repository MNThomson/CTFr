use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::http::AppState;

pub fn router() -> Router<AppState> {
    return Router::new().route("/static/*file", get(static_handler));
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("static/") {
        path = path.replace("static/", "");
    }

    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "src/http/static/"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = from_path(path).first_or_octet_stream();
                return ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response();
            }
            None => return (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}
