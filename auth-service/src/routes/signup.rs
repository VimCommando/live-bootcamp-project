use crate::app_state::AppState;
use crate::domain::{AuthAPIError, User};
use crate::services::UserStoreError;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = match request.email {
        email if email.is_empty() => return Err(AuthAPIError::InvalidCredentials),
        email if !email.contains('@') => return Err(AuthAPIError::InvalidCredentials),
        email => email,
    };

    let password = match request.password {
        password if password.len() < 8 => return Err(AuthAPIError::InvalidCredentials),
        password => password,
    };

    let user = User {
        email,
        password,
        requires_2fa: request.requires_2fa,
    };

    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user) {
        Ok(_) => (),
        Err(UserStoreError::UserAlreadyExists) => return Err(AuthAPIError::UserAlreadyExists),
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    }

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    Ok((StatusCode::CREATED, response))
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}
