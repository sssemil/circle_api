use crate::models::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TransactionGetRequest {}

#[derive(Deserialize, Debug)]
pub struct TransactionGetResponse {
    pub transaction: Transaction,
}
