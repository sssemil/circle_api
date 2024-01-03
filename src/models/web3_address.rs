use serde::{Deserialize, Serialize};
use web3::types::Address;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Web3Address(Address);

impl From<Web3Address> for Address {
    fn from(value: Web3Address) -> Self {
        value.0
    }
}
