use crate::models::blockchain::Blockchain;
use crate::models::custody_type::CustodyType;
use crate::models::operation::Operation;
use crate::models::transaction_state::TransactionState;
use crate::models::transaction_transfer_create::FeeLevel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    id: Uuid,
    abi_function_signature: Option<String>,
    abi_parameters: Option<Vec<String>>,
    amounts: Option<Vec<String>>,
    amount_in_usd: Option<String>,
    block_hash: Option<String>,
    block_height: Option<i64>,
    blockchain: Blockchain,
    contract_address: Option<String>,
    create_date: DateTime<Utc>,
    custody_type: Option<CustodyType>,
    destination_address: Option<String>,
    error_reason: Option<String>,
    estimated_fee: Option<EstimatedFee>,
    first_confirm_date: Option<DateTime<Utc>>,
    network_fee: Option<String>,
    network_fee_in_usd: Option<String>,
    nfts: Option<Vec<String>>,
    operation: Option<Operation>,
    ref_id: Option<String>,
    source_address: Option<String>,
    state: TransactionState,
    token_id: Option<String>,
    transaction_type: String,
    tx_hash: Option<String>,
    update_date: DateTime<Utc>,
    user_id: Option<String>,
    wallet_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EstimatedFee {
    base_fee: Option<String>,
    gas_limit: Option<String>,
    gas_price: Option<String>,
    max_fee: Option<String>,
    priority_fee: Option<String>,
    fee_level: Option<FeeLevel>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum TxType {
    Inbound,
    Outbound,
}
