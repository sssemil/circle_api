use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::pagination::Pagination;
use crate::models::time_range::TimeRange;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WalletObjectsQueryParams {
    include_all: Option<bool>,
    name: Option<String>,
    token_address: Option<String>,
    standard: Option<String>,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    #[serde(flatten)]
    time_range: TimeRange,
    #[serde(flatten)]
    pagination: Pagination,
}

impl WalletObjectsQueryParams {
    pub fn include_all(mut self, value: bool) -> Self {
        self.include_all = Some(value);
        self
    }

    pub fn name<S: Into<String>>(mut self, value: S) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn token_address<S: Into<String>>(mut self, value: S) -> Self {
        self.token_address = Some(value.into());
        self
    }

    pub fn standard<S: Into<String>>(mut self, value: S) -> Self {
        self.standard = Some(value.into());
        self
    }
}
