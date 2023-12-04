use reqwest::Method;
use uuid::Uuid;

use crate::api::{encrypt_entity_secret, CircleClient};
use crate::error::Result;
use crate::models::wallet_balance::{WalletBalanceQueryParams, WalletBalanceResponse};
use crate::models::wallet_create::{WalletCreateRequest, WalletCreateResponse};
use crate::models::wallet_get::WalletGetResponse;
use crate::models::wallet_list::{WalletListQueryParams, WalletListResponse};

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

    pub async fn list_wallets(
        &self,
        query_params: WalletListQueryParams,
    ) -> Result<WalletListResponse> {
        let url = format!("{}w3s/wallets", self.base_url);
        self.send_request(Method::GET, url, Some(query_params))
            .await
    }

    pub async fn get_wallet(&self, wallet_id: Uuid) -> Result<WalletGetResponse> {
        let url = format!("{}w3s/wallets/{}", self.base_url, wallet_id);
        self.send_request(Method::GET, url, None::<()>).await
    }

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
