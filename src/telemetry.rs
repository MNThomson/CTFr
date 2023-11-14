use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ctfr=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .expect("Tracing to start");
}
