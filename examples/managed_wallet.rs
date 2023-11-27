use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use futures::future::join_all;
use log::{error, info};
use once_cell::sync::Lazy;

use circle_api::api::CircleClient;
use circle_api::models::wallet_balance::WalletBalanceQueryParams;
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
    info!("Starting payments-service");
    let circle_client = CircleClient::new(
        CONFIG.circle_api_key.clone(),
        CONFIG.circle_entity_secret.clone(),
    )
    .await?;

    let wallet_set_name = "test_wallet_set";
    let idempotency_key = uuid::Uuid::new_v4();
    let wallet_set_response = circle_client
        .create_wallet_set(idempotency_key, wallet_set_name.to_string())
        .await?
        .wallet_set;
    info!("Wallet set response: {:?}", wallet_set_response);

    let wallet_set_name = "test_updated_wallet_set";
    let update_wallet_set_response = circle_client
        .update_wallet_set(wallet_set_response.id, wallet_set_name.to_string())
        .await?
        .wallet_set;
    info!(
        "Updated wallet set response: {:?}",
        update_wallet_set_response
    );

    let get_wallet_set_response = circle_client.get_wallet_set(wallet_set_response.id).await?;
    info!("Get wallet set response: {:?}", get_wallet_set_response);

    let wallet_sets_response = circle_client
        .list_wallet_sets(WalletSetsQueryParams::default())
        .await?;
    for wallet_set in &wallet_sets_response.wallet_sets {
        info!("Wallet set: {:?}", wallet_set);
    }

    let wallet_set = &wallet_sets_response.wallet_sets[0];

    let idempotency_key = uuid::Uuid::new_v4();
    let create_wallet_response = circle_client
        .create_wallet(
            idempotency_key,
            wallet_set.id,
            vec!["MATIC-MUMBAI".to_string()],
            2,
        )
        .await?;
    for (i, wallet) in create_wallet_response.wallets.iter().enumerate() {
        info!("Wallet #{}: {:?}", i, wallet);
    }

    let balance_futures = create_wallet_response
        .wallets
        .iter()
        .map(|w| {
            circle_client
                .get_wallet_balance(w.id, WalletBalanceQueryParams::default().include_all(true))
        })
        .collect::<Vec<_>>();

    let balances = join_all(balance_futures)
        .await
        .into_iter()
        .map(|r| r.map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()?;
    for balance in balances {
        info!("Balance: {:?}", balance);
    }

    Ok(())
}
