use actix_web::error::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed SQL execution")]
    DbError(diesel::result::Error),
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),
}

impl ResponseError for AppError {}
