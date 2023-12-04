use serde::Serialize;
use uuid::Uuid;

use crate::error::CircleError;
use crate::error::Result;

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    page_before: Option<Uuid>,
    page_after: Option<Uuid>,
    page_size: Option<u8>,
}

impl Pagination {
    pub fn page_before(mut self, value: Uuid) -> Result<Self> {
        if self.page_after.is_some() {
            Err(CircleError::ValueError)?
        }
        self.page_before = Some(value);
        Ok(self)
    }

    pub fn page_after(mut self, value: Uuid) -> Result<Self> {
        if self.page_before.is_some() {
            Err(CircleError::ValueError)?;
        }
        self.page_after = Some(value);
        Ok(self)
    }

    pub fn page_size(mut self, value: u8) -> Self {
        self.page_size = Some(value);
        self
    }
}
