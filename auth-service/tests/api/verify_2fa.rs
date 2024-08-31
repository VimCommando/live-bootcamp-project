use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{Email, LoginAttemptId, TwoFACode},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
};
use serde_json::json;

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let mut app = TestApp::new().await;

    let random_email = Email::parse(&get_random_email()).expect("Random email was not parseable");

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&random_email)
        .await
        .unwrap();

    assert_eq!(
        LoginAttemptId::parse(&json_body.login_attempt_id).unwrap(),
        login_attempt_id
    );

    let verify_2fa_body = json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;
    let login_attempt_id = LoginAttemptId::default();
    let body = json!({
        "email": get_random_email(),
        "loginAttemptId": login_attempt_id,
        "2FACode": "1234", // invalid 2FA code
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 400);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;
    let email = get_random_email();
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let body = json!({
        "email": email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let mut app = TestApp::new().await;

    let random_email = Email::parse(&get_random_email()).expect("Random email was not parseable");

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(json_body.message, "2FA required".to_owned());

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&random_email)
        .await
        .unwrap();

    assert_eq!(
        LoginAttemptId::parse(&json_body.login_attempt_id).unwrap(),
        login_attempt_id
    );

    let verify_2fa_body = json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": two_fa_code
    });

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_verify_2fa(&verify_2fa_body).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;
    let login_attempt_id = LoginAttemptId::default();
    let body = json!({
        "email": get_random_email(),
        "loginAttemptId": login_attempt_id,
        // missing 2FACode
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status().as_u16(), 422);
    app.clean_up().await;
}
