use uuid::Uuid;

pub mod custody_type;
pub mod public_key;
pub mod transaction;
pub mod wallet_balance;
pub mod wallet_create;
pub mod wallet_detail;
pub mod wallet_get;
pub mod wallet_list;
pub mod wallet_set;
pub mod wallet_update;

pub type RequestId = Uuid;
