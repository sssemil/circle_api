use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Blockchain {
    EthGoerli,
    Eth,
    AvaxFuji,
    Avax,
    MaticMumbai,
    Matic,
}
