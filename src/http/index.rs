use axum::{response::Html, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Html<&'static str> {
    Html(
        r#"
<link rel="shortcut icon" href="data:image/x-icon;," type="image/x-icon">
<h1>Hello, World!</h1>
    "#,
    )
}
