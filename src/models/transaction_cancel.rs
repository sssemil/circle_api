use crate::models::auth::Auth;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct TransactionCancelRequest {
    #[serde(flatten)]
    pub auth: Auth,
}

#[derive(Deserialize, Debug)]
pub struct TransactionCancelResponse {
    pub id: Uuid,
}
