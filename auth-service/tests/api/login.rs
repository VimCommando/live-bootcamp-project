use crate::helpers::{get_random_email, TestApp};
use auth_service::{
    domain::{Email, LoginAttemptId},
    routes::TwoFactorAuthResponse,
    utils::constants::JWT_COOKIE_NAME,
};
use serde_json::json;

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "Password123!",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

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

    let (login_attempt_id, _) = app
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
}

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;
    let body = json!({
        "email": get_random_email(),
    });

    let response = app.post_login(&body).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;
    let body = json!({
        "email": "IDontExist@gmail.com",
        "password": "InvalidPassword"
    });

    let response = app.post_login(&body).await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let user_json = json!({
        "email": "mreynolds@serenity.co",
        "password": "N0thingInTheverse!",
        "requires2FA": false
    });

    let response = app.post_signup(&user_json).await;

    assert_eq!(response.status().as_u16(), 201);

    let invalid_password = json!({
        "email": "mreynolds@serenity.co",
        "password": "Noth1ngInTheverse?",
    });

    let response = app.post_login(&invalid_password).await;

    assert_eq!(response.status().as_u16(), 401);
}
