use gethostname::gethostname;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use rustc_version;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_telemetry() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| return "ctfr=debug,axum::rejection=trace".into()),
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
                            KeyValue::new("service.name", "ctfr"),
                            KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
                            KeyValue::new("process.runtime.name", "rustc"),
                            KeyValue::new(
                                "process.runtime.version",
                                rustc_version::version().unwrap().to_string(),
                            ),
                            KeyValue::new("process.command", std::env::args().next().unwrap()),
                            KeyValue::new(
                                "process.command_line",
                                std::env::args().collect::<Vec<_>>().join(" "),
                            ),
                            KeyValue::new(
                                "process.executable.name",
                                std::env::current_exe()
                                    .unwrap()
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy()
                                    .into_owned(),
                            ),
                            KeyValue::new(
                                "process.executable.path",
                                std::env::current_exe()
                                    .unwrap()
                                    .display()
                                    .to_string(),
                            ),
                            KeyValue::new("process.pid", std::process::id() as i64),
                            KeyValue::new("host.arch", std::env::consts::ARCH),
                            KeyValue::new("host.name", gethostname().into_string().unwrap()),
                        ])))
                    .install_batch(runtime::Tokio)
                    .unwrap(),
            ),
        )
        .try_init()
        .expect("Tracing to start");
}
