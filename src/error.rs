use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::api::ApiError;
use crate::models::RequestId;

pub type Result<T> = std::result::Result<T, CircleError>;

#[derive(Debug)]
pub enum CircleError {
    ApiError(RequestId, ApiError),
    ValueError,
    MissingRequestId,
    MissingField(&'static str),
    RequestIdIsNotAValidString(reqwest::header::ToStrError),
    RequestIdIsNotAValidUuid(uuid::Error),
    UnknownRequestError(reqwest::Error),
    FromHexError(hex::FromHexError),
    RsaError(rsa::errors::Error),
    SerdeQsError(serde_qs::Error),
    SerdeJsonError(serde_json::Error),
    Web3SigningRecoveryError(web3::signing::RecoveryError),
}

impl Display for CircleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CircleError({:?})", self)
    }
}

impl Error for CircleError {}

impl From<reqwest::header::ToStrError> for CircleError {
    fn from(err: reqwest::header::ToStrError) -> Self {
        CircleError::RequestIdIsNotAValidString(err)
    }
}

impl From<uuid::Error> for CircleError {
    fn from(err: uuid::Error) -> Self {
        CircleError::RequestIdIsNotAValidUuid(err)
    }
}

impl From<reqwest::Error> for CircleError {
    fn from(err: reqwest::Error) -> Self {
        CircleError::UnknownRequestError(err)
    }
}

impl From<hex::FromHexError> for CircleError {
    fn from(err: hex::FromHexError) -> Self {
        CircleError::FromHexError(err)
    }
}

impl From<rsa::errors::Error> for CircleError {
    fn from(err: rsa::errors::Error) -> Self {
        CircleError::RsaError(err)
    }
}

impl From<serde_qs::Error> for CircleError {
    fn from(err: serde_qs::Error) -> Self {
        CircleError::SerdeQsError(err)
    }
}

impl From<serde_json::Error> for CircleError {
    fn from(err: serde_json::Error) -> Self {
        CircleError::SerdeJsonError(err)
    }
}

impl From<web3::signing::RecoveryError> for CircleError {
    fn from(err: web3::signing::RecoveryError) -> Self {
        CircleError::Web3SigningRecoveryError(err)
    }
}
