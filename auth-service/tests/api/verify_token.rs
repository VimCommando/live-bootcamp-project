use crate::helpers::{get_random_email, TestApp};
use auth_service::utils::constants::JWT_COOKIE_NAME;
use serde_json::json;

#[tokio::test]
async fn should_return_200_valid_token() {
    let mut app = TestApp::new().await;
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

    let body = json!({
        "token": auth_cookie.value(),
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status(), 200);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    let body = json!({
        "token": "invalid_token",
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let mut app = TestApp::new().await;
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

    let body = json!({
        "token": auth_cookie.value(),
    });

    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status(), 200);

    app.post_logout().await;

    let response = app.post_verify_token(&body).await;
    assert_eq!(response.status(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let body = json!({
        "invalid_key": "invalid_value",
    });

    let response = app.post_verify_token(&body).await;

    assert_eq!(response.status(), 422);
    app.clean_up().await;
}
