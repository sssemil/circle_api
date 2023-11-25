use anyhow::Result;
use env_logger::Env;
use log::{error, info};

use crate::circle::api::CircleClient;
use crate::static_config::CONFIG;

mod circle;
pub mod static_config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    match run().await {
        Ok(_) => {}
        Err(err) => {
            error!("Error: {:?}", err);
            Err(err)?
        }
    }
    Ok(())
}

async fn run() -> Result<(), anyhow::Error> {
    info!("Starting payments-service");
    let circle_client = CircleClient::new(
        CONFIG.circle_api_key.clone(),
        CONFIG.circle_entity_secret.clone(),
    )
        .await?;
    let idempotency_key = uuid::Uuid::new_v4();
    let wallet_set_response = circle_client
        .create_wallet_set(idempotency_key.to_string(), "test_wallet_set".to_string())
        .await?;
    info!("Wallet set response: {:?}", wallet_set_response);
    Ok(())
}
