use crate::{
    domain::{AuthAPIError, Email, LoginAttemptId, TwoFACode, UserStoreError},
    utils::auth::generate_auth_cookie,
    AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

pub async fn verify_2fa(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<Verify2FARequest>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let mut two_fa_code_store = state.two_fa_code_store.write().await;
    let user_store = state.user_store.read().await;

    if let (Ok(email), Ok(login_attempt_id), Ok(two_fa_code)) = (
        Email::parse(&request.email),
        LoginAttemptId::parse(&request.login_attempt_id),
        TwoFACode::parse(&request.two_fa_code),
    ) {
        let code_tuple = two_fa_code_store.get_code(&email).await;
        let login_tuple = (login_attempt_id, two_fa_code);
        let user = user_store.get_user(&email).await.map_err(|e| match e {
            UserStoreError::UserNotFound => AuthAPIError::IncorrectCredentials,
            _ => AuthAPIError::UnexpectedError,
        })?;

        match code_tuple {
            Ok(code_tuple) if code_tuple == login_tuple => {
                two_fa_code_store
                    .remove_code(&email)
                    .await
                    .map_err(|_| AuthAPIError::UnexpectedError)?;

                let auth_cookie =
                    generate_auth_cookie(&user.email).map_err(|_| AuthAPIError::UnexpectedError)?;

                Ok((jar.add(auth_cookie), StatusCode::OK.into_response()))
            }
            Ok(_) | Err(_) => Err(AuthAPIError::IncorrectCredentials),
        }
    } else {
        Err(AuthAPIError::InvalidCredentials)
    }
}

#[derive(Debug, Deserialize)]
pub struct Verify2FARequest {
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
