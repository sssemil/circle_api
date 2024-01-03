use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use log::{error, info};
use once_cell::sync::Lazy;
use web3::signing::keccak256;

use circle_api::api::CircleClient;

use circle_api::models::wallet_list::WalletListQueryParams;
use circle_api::models::wallet_set::WalletSetsQueryParams;

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
    let circle_client = CircleClient::new(
        CONFIG.circle_api_key.clone(),
        CONFIG.circle_entity_secret.clone(),
    )
    .await?;
    let wallet_set_name = "test_wallet_set";

    let list_wallet_set_response = circle_client
        .list_wallet_sets(WalletSetsQueryParams::new())
        .await?;
    let wallet_set = list_wallet_set_response
        .wallet_sets
        .iter()
        .filter(|x| x.name.is_some())
        .find(|x| x.name.as_ref().unwrap() == wallet_set_name)
        .ok_or(anyhow::Error::msg("Wallet set not found!"))?;

    let list_wallet_response = circle_client
        .list_wallets(WalletListQueryParams::default().wallet_set_id(wallet_set.id))
        .await?;

    let wallet = &list_wallet_response.wallets[0];
    info!("Wallet: {:?}", wallet);

    let message = "Dummy message for the world!";
    let formatted_message = format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
    let message_hash = keccak256(formatted_message.as_bytes());
    info!("Message hash: {:?}", message_hash);

    let signature = circle_client
        .sign_ethereum_message(wallet.id, message.to_string())
        .await?;
    info!("Signature: {:?}", signature);

    let signature_valid = circle_client
        .verify_ethereum_message(
            wallet.address.clone(),
            message.to_string(),
            signature.clone(),
        )
        .await?;
    info!("Signature valid: {:?}", signature_valid);

    Ok(())
}
