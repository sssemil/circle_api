use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionState {
    Initiated,
    PendingRiskScreening,
    Denied,
    Queued,
    Sent,
    Confirmed,
    Complete,
    Failed,
    Cancelled,
}
