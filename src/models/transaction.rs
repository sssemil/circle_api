use crate::models::blockchain::Blockchain;
use crate::models::custody_type::CustodyType;
use crate::models::operation::Operation;
use crate::models::transaction_state::TransactionState;
use crate::models::transaction_transfer_create::FeeLevel;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::web3_address::Web3Address;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: Uuid,
    pub abi_function_signature: Option<String>,
    pub abi_parameters: Option<Vec<String>>,
    pub amounts: Option<Vec<String>>,
    pub amount_in_usd: Option<String>,
    pub block_hash: Option<String>,
    pub block_height: Option<i64>,
    pub blockchain: Blockchain,
    pub contract_address: Option<Web3Address>,
    pub create_date: DateTime<Utc>,
    pub custody_type: Option<CustodyType>,
    pub destination_address: Option<Web3Address>,
    pub error_reason: Option<String>,
    pub estimated_fee: Option<EstimatedFee>,
    pub first_confirm_date: Option<DateTime<Utc>>,
    pub network_fee: Option<String>,
    pub network_fee_in_usd: Option<String>,
    pub nfts: Option<Vec<String>>,
    pub operation: Option<Operation>,
    pub ref_id: Option<String>,
    pub source_address: Option<Web3Address>,
    pub state: TransactionState,
    pub token_id: Option<String>,
    pub transaction_type: String,
    pub tx_hash: Option<String>,
    pub update_date: DateTime<Utc>,
    pub user_id: Option<String>,
    pub wallet_id: Option<String>,
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
