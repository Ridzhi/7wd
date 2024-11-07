use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use time::PrimitiveDateTime;
use thiserror::Error;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Error, PartialEq)]
pub enum ErrorKind {
    #[error("user not found")]
    UserNotFound,
    #[error("email already in use")]
    EmailAlreadyInUse,
    #[error("nickname already in use")]
    NicknameAlreadyInUse,
    #[error("invalid credentials")]
    InvalidCredentials,
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // здесь можно обозначить свою структуру
        // и проконтролить что мы возвращаем на клиент как есть а что нет

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0)
        ).into_response()
    }
}

impl <E> From<E> for AppError
where
    E: Into<anyhow::Error>
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UtcDateTime(pub PrimitiveDateTime);

impl Default for UtcDateTime {
    fn default() -> Self {
        Self(now_utc())
    }
}

impl Into<PrimitiveDateTime> for UtcDateTime {
    fn into(self) -> PrimitiveDateTime {
        self.0
    }
}

pub fn now_utc() -> PrimitiveDateTime {
    let now = time::OffsetDateTime::now_utc();

    PrimitiveDateTime::new(now.date(), now.time())
}