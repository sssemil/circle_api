use crate::error::{CircleError, Result};
use crate::models::web3_signature::Web3Signature;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SignMessageRequest {
    wallet_id: Uuid,
    encoded_by_hex: Option<bool>,
    message: String,
    memo: Option<String>,
    entity_secret_ciphertext: String,
}

#[derive(Debug, Default)]
pub struct SignMessageRequestBuilder {
    wallet_id: Option<Uuid>,
    encoded_by_hex: Option<bool>,
    message: Option<String>,
    memo: Option<String>,
    entity_secret_ciphertext: Option<String>,
}

impl SignMessageRequestBuilder {
    pub fn wallet_id(mut self, wallet_id: Uuid) -> Self {
        self.wallet_id = Some(wallet_id);
        self
    }

    pub fn encoded_by_hex(mut self, encoded_by_hex: bool) -> Self {
        self.encoded_by_hex = Some(encoded_by_hex);
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    pub fn memo(mut self, memo: String) -> Self {
        self.memo = Some(memo);
        self
    }

    pub fn entity_secret_ciphertext(mut self, entity_secret_ciphertext: String) -> Self {
        self.entity_secret_ciphertext = Some(entity_secret_ciphertext);
        self
    }

    pub fn build(self) -> Result<SignMessageRequest> {
        Ok(SignMessageRequest {
            wallet_id: self
                .wallet_id
                .ok_or(CircleError::MissingField("wallet_id"))?,
            encoded_by_hex: self.encoded_by_hex,
            message: self.message.ok_or(CircleError::MissingField("message"))?,
            memo: self.memo,
            entity_secret_ciphertext: self
                .entity_secret_ciphertext
                .ok_or(CircleError::MissingField("entity_secret_ciphertext"))?,
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct SignMessageResponse {
    pub signature: Web3Signature,
}
