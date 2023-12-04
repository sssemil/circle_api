use uuid::Uuid;

pub mod custody_type;
pub mod public_key;
pub mod transaction;
pub mod wallet_balance;
pub mod wallet_create;
pub mod wallet_list;
pub mod wallet_set;

pub type RequestId = Uuid;
