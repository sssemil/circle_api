use crate::models::auth::Auth;
use crate::models::transaction_state::TransactionState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTransferCreateRequest {
    #[serde(flatten)]
    auth: Auth,
    amounts: Vec<String>,
    destination_address: String,
    fee_level: Option<FeeLevel>,
    gas_limit: Option<u64>,
    gas_price: Option<f64>,
    max_fee: Option<f64>,
    priority_fee: Option<f64>,
    nft_token_ids: Option<Vec<String>>,
    ref_id: Option<String>,
    token_id: Uuid,
    wallet_id: Uuid,
}

pub struct TransactionTransferCreateRequestBuilder {
    amounts: Vec<i64>,
    destination_address: String,
    fee_level: Option<FeeLevel>,
    gas_limit: Option<u64>,
    gas_price: Option<f64>,
    max_fee: Option<f64>,
    priority_fee: Option<f64>,
    nft_token_ids: Option<Vec<String>>,
    ref_id: Option<String>,
    token_id: Uuid,
    wallet_id: Uuid,
}

impl TransactionTransferCreateRequestBuilder {
    pub fn new(destination_address: String, token_id: Uuid, wallet_id: Uuid, amount: i64) -> Self {
        TransactionTransferCreateRequestBuilder {
            amounts: vec![amount],
            destination_address,
            fee_level: None,
            gas_limit: None,
            gas_price: None,
            max_fee: None,
            priority_fee: None,
            nft_token_ids: None,
            ref_id: None,
            token_id,
            wallet_id,
        }
    }

    pub fn amounts(mut self, amounts: Vec<i64>) -> Self {
        self.amounts = amounts;
        self
    }

    pub fn fee_level(mut self, fee_level: FeeLevel) -> Self {
        self.fee_level = Some(fee_level);
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    pub fn gas_price(mut self, gas_price: f64) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    pub fn max_fee(mut self, max_fee: f64) -> Self {
        self.max_fee = Some(max_fee);
        self
    }

    pub fn priority_fee(mut self, priority_fee: f64) -> Self {
        self.priority_fee = Some(priority_fee);
        self
    }

    pub fn nft_token_ids(mut self, nft_token_ids: Vec<String>) -> Self {
        self.nft_token_ids = Some(nft_token_ids);
        self
    }

    pub fn ref_id<S: Into<String>>(mut self, ref_id: S) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }

    pub fn build(self, auth: Auth) -> TransactionTransferCreateRequest {
        TransactionTransferCreateRequest {
            auth,
            amounts: self.amounts.iter().map(|x| x.to_string()).collect(),
            destination_address: self.destination_address,
            fee_level: self.fee_level,
            gas_limit: self.gas_limit,
            gas_price: self.gas_price,
            max_fee: self.max_fee,
            priority_fee: self.priority_fee,
            nft_token_ids: self.nft_token_ids,
            ref_id: self.ref_id,
            token_id: self.token_id,
            wallet_id: self.wallet_id,
        }
    }
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
pub struct TransactionTransferCreateResponse {
    pub id: Uuid,
    pub state: TransactionState,
}
