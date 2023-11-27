use anyhow::Result;
use reqwest::Method;

use crate::api::{encrypt_entity_secret, CircleClient};
use uuid::Uuid;

use crate::models::wallet_balance::{WalletBalanceQueryParams, WalletBalanceResponse};
use crate::models::wallet_create::{WalletCreateRequest, WalletCreateResponse};

impl CircleClient {
    pub async fn create_wallet(
        &self,
        idempotency_key: Uuid,
        wallet_set_id: Uuid,
        blockchains: Vec<String>,
        count: u32,
    ) -> Result<WalletCreateResponse> {
        let url = format!("{}w3s/developer/wallets", self.base_url);
        let request = WalletCreateRequest {
            idempotency_key,
            entity_secret_cipher_text: encrypt_entity_secret(
                &self.public_key,
                &self.circle_entity_secret,
            )?,
            wallet_set_id,
            blockchains,
            count,
        };
        self.send_request(Method::POST, url, Some(request)).await
    }

    // TODO: list wallets
    // TODO: get a wallet
    // TODO: updates a wallet

    pub async fn get_wallet_balance(
        &self,
        wallet_id: Uuid,
        query_params: WalletBalanceQueryParams,
    ) -> Result<WalletBalanceResponse> {
        let url = format!("{}w3s/wallets/{}/balances", self.base_url, wallet_id);
        self.send_request(Method::GET, url, Some(query_params))
            .await
    }

    // TODO: get nfts for a wallet
}
