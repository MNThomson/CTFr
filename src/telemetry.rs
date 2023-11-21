use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| return "ctfr=debug,axum::rejection=trace,sqlx=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_opentelemetry::layer().with_tracer(
                opentelemetry_otlp::new_pipeline()
                    .tracing()
                    .with_exporter(
                        opentelemetry_otlp::new_exporter()
                            .tonic()
                            .with_endpoint("http://0.0.0.0:4317"),
                    )
                    .with_trace_config(sdktrace::config().with_resource(Resource::new(vec![
                        KeyValue::new("ctfr.version", env!("CARGO_PKG_VERSION")),
                    ])))
                    .install_batch(runtime::Tokio)
                    .unwrap(),
            ),
        )
        .try_init()
        .expect("Tracing to start");
}
