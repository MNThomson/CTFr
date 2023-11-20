use super::AppState;
use axum::{extract::State, response::Html, routing::get, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}

async fn handler(State(state): State<AppState>) -> Html<&'static str> {
    Html(
        r#"
<link rel="shortcut icon" href="data:image/x-icon;," type="image/x-icon">
<h1>Hello, World!</h1>
    "#,
    )
}