use serde::{Deserialize, Serialize};

use crate::models::blockchain::Blockchain;
use crate::models::custody_type::CustodyType;
use crate::models::operation::Operation;
use crate::models::pagination::Pagination;
use crate::models::time_range::TimeRange;
use crate::models::transaction::Transaction;
use crate::models::transaction_state::TransactionState;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListQueryParams {
    blockchain: Option<Blockchain>,
    custody_type: Option<CustodyType>,
    destination_address: Option<String>,
    include_all: Option<bool>,
    operation: Option<Operation>,
    state: Option<TransactionState>,
    tx_hash: Option<String>,
    tx_type: Option<String>,
    wallet_ids: Option<String>,
    #[serde(flatten)]
    time_range: TimeRange,
    #[serde(flatten)]
    pagination: Pagination,
}

impl TransactionListQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn blockchain(mut self, value: Blockchain) -> Self {
        self.blockchain = Some(value);
        self
    }

    pub fn custody_type(mut self, value: CustodyType) -> Self {
        self.custody_type = Some(value);
        self
    }

    pub fn destination_address(mut self, value: String) -> Self {
        self.destination_address = Some(value);
        self
    }

    pub fn include_all(mut self, value: bool) -> Self {
        self.include_all = Some(value);
        self
    }

    pub fn operation(mut self, value: Operation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn state(mut self, value: TransactionState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn tx_hash(mut self, value: String) -> Self {
        self.tx_hash = Some(value);
        self
    }

    pub fn tx_type(mut self, value: String) -> Self {
        self.tx_type = Some(value);
        self
    }

    pub fn wallet_ids(mut self, value: String) -> Self {
        self.wallet_ids = Some(value);
        self
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionListResponse {
    pub transactions: Vec<Transaction>,
}
