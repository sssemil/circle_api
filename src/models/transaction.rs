use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub idempotency_key: Uuid,
    pub amounts: Vec<String>,
    pub destination_address: String,
    pub entity_secret_cipher_text: String,
    pub fee_level: Option<FeeLevel>,
    pub gas_limit: Option<u64>,
    pub gas_price: Option<f64>,
    pub max_fee: Option<f64>,
    pub priority_fee: Option<f64>,
    pub nft_token_ids: Option<Vec<String>>,
    pub ref_id: Option<String>,
    pub token_id: Uuid,
    pub wallet_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FeeLevel {
    Low,
    Medium,
    High,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    pub id: String,
    pub state: TransactionState,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionState {
    Initiated,
    PendingRiskScreening,
    Denied,
    Queued,
    Sent,
    Confirmed,
    Complete,
    Failed,
    Cancelled,
}
