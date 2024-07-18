use tg_api::bot;
use tracing_subscriber::EnvFilter;

#[tokio::main]
pub async fn main() -> Result<(), bot::TgError> {
    tracing_subscriber::fmt()
        .without_time() // For dev enb
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    log::info!("Starting bot...");

    let bot = bot::TgBot::new();
    let _ = bot.init().await;

    Ok(())
}
