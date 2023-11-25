use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use once_cell::sync::Lazy;

use circle_api::api::CircleClient;

pub fn get_env(env: &'static str) -> String {
    std::env::var(env).unwrap_or_else(|_| panic!("Cannot get the {} env variable", env))
}

pub struct Config {
    pub circle_api_key: String,
    pub circle_entity_secret: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().expect("Failed to read .env file");

    Config {
        circle_api_key: get_env("CIRCLE_API_KEY"),
        circle_entity_secret: get_env("CIRCLE_ENTITY_SECRET"),
    }
});

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
        .await?
        .wallet_set;
    info!("Wallet set response: {:?}", wallet_set_response);
    let idempotency_key = uuid::Uuid::new_v4();
    let create_wallet_response = circle_client
        .create_wallet(
            idempotency_key.to_string(),
            wallet_set_response.id,
            vec!["MATIC-MUMBAI".to_string()],
            2,
        )
        .await?;
    for (i, wallet) in create_wallet_response.wallets.iter().enumerate() {
        info!("Wallet #{}: {:?}", i, wallet);
    }
    Ok(())
}
