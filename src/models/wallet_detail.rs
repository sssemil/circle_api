use crate::models::blockchain::Blockchain;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::custody_type::CustodyType;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WalletDetail {
    pub id: Uuid,
    pub address: String,
    pub blockchain: Blockchain,
    pub create_date: DateTime<Utc>,
    pub custody_type: CustodyType,
    pub account_type: Option<String>,
    pub name: Option<String>,
    pub ref_id: Option<String>,
    pub state: WalletState,
    pub update_date: DateTime<Utc>,
    pub user_id: Option<String>,
    pub wallet_set_id: Uuid,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WalletState {
    Live,
    Frozen,
}
