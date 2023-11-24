use anyhow::Result;

use openssl::pkey::Public;
use openssl::rsa::Rsa;
use reqwest::Client;
use serde::Deserialize;

use crate::circle::error::CircleError;
use crate::circle::models::public_key::PublicKeyResponse;
use crate::circle::models::wallet_set::{WalletSetRequest, WalletSetResponse};
use crate::circle::utils::encrypt_entity_secret;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiResponse<T> {
    data: T,
}

pub struct CircleClient {
    base_url: String,
    api_key: String,
    circle_entity_secret: String,
    client: Client,
    public_key: Rsa<Public>,
}

impl CircleClient {
    pub async fn new(api_key: String, circle_entity_secret: String) -> Result<Self> {
        let base_url = "https://api.circle.com/v1/".to_string();
        let client = Client::new();

        let url = format!("{}w3s/config/entity/publicKey", base_url);
        let res = client
            .get(&url)
            .header("Content-Type", "application/json")
            .bearer_auth(&api_key)
            .send()
            .await?;

        let public_key = if res.status().is_success() {
            res.json::<ApiResponse<PublicKeyResponse>>()
                .await?
                .data
                .public_key
        } else {
            Err(CircleError::ResponseStatusCodeError(res.status()))?
        };

        let public_key = Rsa::public_key_from_pem_pkcs1(public_key.as_bytes())?;

        Ok(CircleClient {
            base_url,
            api_key,
            circle_entity_secret,
            client: Client::new(),
            public_key,
        })
    }

    pub async fn create_wallet_set(
        &self,
        idempotency_key: String,
        name: String,
    ) -> Result<WalletSetResponse> {
        let url = format!("{}w3s/developer/walletSets", self.base_url);
        let request = WalletSetRequest {
            idempotency_key,
            entity_secret_cipher_text: encrypt_entity_secret(
                self.public_key.as_ref(),
                &self.circle_entity_secret,
            )?,
            name,
        };
        let res = self
            .client
            .post(&url)
            .json(&request)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if res.status().is_success() {
            let wallet_set_response = res.json::<ApiResponse<WalletSetResponse>>().await?;
            Ok(wallet_set_response.data)
        } else {
            Err(CircleError::ResponseStatusCodeError(res.status()))?
        }
    }
}
