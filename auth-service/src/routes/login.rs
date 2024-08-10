use crate::app_state::AppState;
use crate::domain::{AuthAPIError, Email, Password, UserStore, UserStoreError};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = match Email::parse(&request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    let password = match Password::parse(&request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::IncorrectCredentials),
    };

    let user_store = state.user_store.read().await;
    match user_store.validate_user(&email, &password).await {
        Ok(_) => (),
        Err(UserStoreError::UserNotFound) => return Err(AuthAPIError::IncorrectCredentials),
        Err(UserStoreError::InvalidCredentials) => return Err(AuthAPIError::IncorrectCredentials),
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    };

    Ok(StatusCode::OK.into_response())
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
