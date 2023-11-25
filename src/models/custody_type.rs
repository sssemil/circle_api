use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustodyType {
    Developer,
    EndUser,
}
