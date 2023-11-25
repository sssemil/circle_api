use crate::models::custody_type::CustodyType;
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
#[serde(rename_all = "camelCase")]
pub struct WalletDetail {
    pub id: Uuid,
    pub address: String,
    pub blockchain: String,
    pub create_date: DateTime<Utc>,
    pub custody_type: CustodyType,
    pub account_type: Option<String>,
    pub name: Option<String>,
    pub ref_id: Option<String>,
    pub state: String,
    pub update_date: DateTime<Utc>,
    pub user_id: Option<String>,
    pub wallet_set_id: Uuid,
}

#[derive(Deserialize, Debug)]
pub struct WalletCreateResponse {
    pub wallets: Vec<WalletDetail>,
}
