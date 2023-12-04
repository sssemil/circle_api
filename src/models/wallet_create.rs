use crate::models::custody_type::CustodyType;
use crate::models::wallet_detail::WalletDetail;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletCreateRequest {
    pub idempotency_key: Uuid,
    pub entity_secret_cipher_text: String,
    pub wallet_set_id: Uuid,
    pub blockchains: Vec<String>,
    pub count: u32,
}

#[derive(Deserialize, Debug)]
pub struct WalletCreateResponse {
    pub wallets: Vec<WalletDetail>,
}
