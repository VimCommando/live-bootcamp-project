use crate::{
    app_state::AppState,
    domain::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;

pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError> {
    let auth_cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = auth_cookie.value().to_owned();

    let _claim = validate_token(&token, state.banned_token_store.clone())
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    let jar = jar.remove(JWT_COOKIE_NAME);
    state
        .banned_token_store
        .write()
        .await
        .add_token(token)
        .await
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    Ok((jar, StatusCode::OK))
}
