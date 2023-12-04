use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TimeRange {
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
}

impl TimeRange {
    pub fn from(mut self, value: DateTime<Utc>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<Utc>) -> Self {
        self.to = Some(value);
        self
    }
}
