mod http;
mod telemetry;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(not(debug_assertions)) {
        utils::print_banner();
    }

    telemetry::init_telemetry();

    http::serve().await?;

    Ok(())
}
