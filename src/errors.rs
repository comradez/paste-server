use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Redis connection pool error.")]
    R2d2Error(#[from] r2d2::Error),
    #[error("Redis database error.")]
    RedisError(#[from] redis::RedisError),
    #[error("Not a valid UTF-8 string.")]
    FromUTF8Error(#[from] std::string::FromUtf8Error),
}

impl actix_web::ResponseError for MyError {}
