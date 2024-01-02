use std::time::Duration;

use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use futures::future::join_all;
use log::{error, info};
use once_cell::sync::Lazy;
use tokio::time::sleep;
use uuid::Uuid;

use circle_api::api::CircleClient;
use circle_api::models::transaction_state::TransactionState;
use circle_api::models::transaction_transfer_create::{
    FeeLevel, TransactionTransferCreateRequestBuilder,
};
use circle_api::models::wallet_balance::{WalletBalanceQueryParams, WalletBalanceResponse};
use circle_api::models::wallet_detail::WalletDetail;
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

async fn get_balances(
    circle_client: &CircleClient,
    wallets: &[WalletDetail],
) -> Result<Vec<WalletBalanceResponse>> {
    let balance_futures = wallets
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

    Ok(balances)
}

fn print_balances(balances: &[WalletBalanceResponse]) {
    for (i, balance) in balances.iter().enumerate() {
        info!("Balance #{}", i);
        for b in &balance.token_balances {
            info!(
                "Balance #{}: {}; {}",
                i,
                b.token.symbol.clone().unwrap_or_default(),
                b.amount
            );
        }
    }
}

async fn run() -> Result<(), anyhow::Error> {
    info!("Starting payments-service");
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

    let wallets = &list_wallet_response.wallets[0..2];
    for (i, wallet) in wallets.iter().enumerate() {
        info!("Wallet #{}: {:?}", i, wallet);
    }

    let balances = get_balances(&circle_client, wallets).await?;
    print_balances(&balances);

    let native_token = &balances[0]
        .token_balances
        .iter()
        .find(|x| x.token.is_native)
        .expect("Native token not found!")
        .token;
    let native_token_info = circle_client.get_token_details(native_token.id).await?;
    info!("Native token info: {:?}", native_token_info.token);

    info!("Sending a bit of eth from wallet 0 to wallet 1...");

    let idempotency_key = Uuid::new_v4();
    let transaction_request_builder = TransactionTransferCreateRequestBuilder::new(
        wallets[1].address.clone(),
        native_token.id,
        wallets[0].id,
        0.0001,
    )
    .fee_level(FeeLevel::Low);
    let tx_request = circle_client
        .create_transfer_transaction(idempotency_key, transaction_request_builder)
        .await?;

    info!("Transaction created: {:?}", tx_request);

    loop {
        let tx = circle_client.get_transaction(tx_request.id, None).await?;
        info!("Transaction: {:?}", tx.transaction.state);

        // if matches!(tx.transaction.state, TransactionState::Queued) {
        //     let tx_cancel = circle_client.cancel_transaction(tx_request.id).await?;
        //     info!("Transaction cancelled: {:?}", tx_cancel);
        // }

        if matches!(
            tx.transaction.state,
            TransactionState::Confirmed | TransactionState::Failed
        ) {
            info!("Transaction finished! {:?}", tx);
            break;
        }
        sleep(Duration::from_millis(200)).await;
    }

    let balances = get_balances(&circle_client, wallets).await?;
    print_balances(&balances);

    let tx_list = circle_client.list_transactions(Default::default()).await?;
    for tx in tx_list.transactions {
        info!("Transaction: {:?}", tx);
    }

    Ok(())
}
