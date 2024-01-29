use actix_web::HttpResponse;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Password doesn't match")]
    WrongPassword,
    #[error("Email is already taken")]
    DuplicateUserEmail,
    #[error("Search data not found")]
    NotFoundData,
    #[error("Resize file error")]
    ResizeImageError,
}
pub type ApiResult<T> = std::result::Result<T, CustomError>;
pub type ApiError = HttpResponse;

impl From<CustomError> for ApiError {
    fn from(err: CustomError) -> Self {
        let payload = json!({"message": err.to_string()});
        let status = match err {
            CustomError::WrongCredentials => HttpResponse::Unauthorized().json(payload),
            CustomError::ValidationError(_) => HttpResponse::BadRequest().json(payload),
            CustomError::NotFoundData => HttpResponse::NoContent().json(payload),
            _ => HttpResponse::InternalServerError().json(actix_web::web::Json(payload))
        };
        status
    }
}
