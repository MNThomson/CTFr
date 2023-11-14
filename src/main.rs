use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const BANNER: &str = r#"
 ██████╗████████╗███████╗
██╔════╝╚══██╔══╝██╔════╝
██║        ██║   █████╗  █████╗
██║        ██║   ██╔══╝ ██╔═══╝
╚██████╗   ██║   ██║    ██║
 ╚═════╝   ╚═╝   ╚═╝    ╚═╝
                 MNThomson
"#;

#[tokio::main]
async fn main() {
    println!("{}", BANNER);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ctfr=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .expect("Tracing to start");

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 4321));
    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(
        r#"
<link rel="shortcut icon" href="data:image/x-icon;," type="image/x-icon">
<h1>Hello, World!</h1>
    "#,
    )
}
