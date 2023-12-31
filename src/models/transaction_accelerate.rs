use crate::models::auth::Auth;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct TransactionAccelerateRequest {
    #[serde(flatten)]
    pub auth: Auth,
}

#[derive(Deserialize, Debug)]
pub struct TransactionAccelerateResponse {
    pub id: Uuid,
}
