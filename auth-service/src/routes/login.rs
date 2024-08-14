use crate::{
    app_state::AppState,
    domain::{AuthAPIError, Email, LoginAttemptId, Password, TwoFACode, UserStoreError},
    utils::auth::generate_auth_cookie,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let email = Email::parse(&request.email).map_err(|_| AuthAPIError::InvalidCredentials)?;
    let password =
        Password::parse(&request.password).map_err(|_| AuthAPIError::InvalidCredentials)?;

    let user_store = state.user_store.read().await;

    user_store
        .validate_user(&email, &password)
        .await
        .map_err(|e| match e {
            UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
            UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
            _ => AuthAPIError::UnexpectedError,
        })?;

    let user = user_store.get_user(&email).await.map_err(|e| match e {
        UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
        UserStoreError::InvalidCredentials => AuthAPIError::IncorrectCredentials,
        _ => AuthAPIError::UnexpectedError,
    })?;

    let auth_cookie =
        generate_auth_cookie(&user.email).map_err(|_| AuthAPIError::UnexpectedError)?;

    let jar = jar.add(auth_cookie);

    // Handle request based on user's 2FA configuration
    match user.requires_2fa {
        true => handle_2fa(&user.email, &state, jar).await,
        false => handle_no_2fa(&user.email, jar).await,
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

async fn handle_2fa(
    email: &Email,
    state: &AppState,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    state
        .two_fa_code_store
        .write()
        .await
        .add_code(email.clone(), login_attempt_id.clone(), two_fa_code.clone())
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    // Finally, we need to return the login attempt ID to the client
    let auth_response = TwoFactorAuthResponse {
        message: "2FA required".to_string(),
        login_attempt_id: login_attempt_id.to_string(),
    };

    let response = (
        StatusCode::PARTIAL_CONTENT,
        Json(LoginResponse::TwoFactorAuth(auth_response)),
    );
    Ok((jar, response))
}

async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> Result<(CookieJar, (StatusCode, Json<LoginResponse>)), AuthAPIError> {
    let response = (StatusCode::OK, Json(LoginResponse::RegularAuth));
    Ok((jar, response))
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
