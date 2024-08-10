use crate::{domain::AuthAPIError, utils::auth::validate_token};
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn verify_token(
    Json(request): Json<VerifyTokenRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    match validate_token(&request.token).await {
        Ok(_) => Ok(StatusCode::OK.into_response()),
        Err(_) => Err(AuthAPIError::InvalidToken),
    }
}

#[derive(Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}
