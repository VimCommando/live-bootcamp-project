use crate::helpers::{get_random_email, TestApp};
use auth_service::{routes::SignupResponse, ErrorResponse};
use serde_json::json;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let random_email = get_random_email(); // Call helper method to generate email

    let test_cases = [
        json!({
            "password": "N0thingInTheverse!",
            "requires2FA": true
        }),
        json!({
            "email": random_email,
            "requires2FA": true
        }),
        json!({
            "email": random_email,
            "password": "N0thingInTheverse!"
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await; // call `post_signup`
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let mut app = TestApp::new().await;
    let user_json = json!({
        "email": "mreynolds@serenity.co",
        "password": "N0thingInTheverse!",
        "requires2FA": false
    });

    let response = app.post_signup(&user_json).await;

    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignupResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // The signup route should return a 400 HTTP status code if an invalid input is sent.
    // The input is considered invalid if:
    // - The email is empty or does not contain '@'
    // - The password is less than 8 characters
    let mut app = TestApp::new().await;
    let invalid_signups = vec![
        // No '@' in email
        json!({
            "email": "mreynolds_serenity.co",
            "password": "N0thingInTheverse!",
            "requires2FA": false
        }),
        // Password is less than 8 characters
        json!({
            "email": "mreynolds@serenity.co",
            "password": "pass",
            "requires2FA": false
        }),
        // Empty email
        json!({
            "email": "",
            "password": "N0thingInTheverse!",
            "requires2FA": false
        }),
    ];

    for invalid_signup in invalid_signups.iter() {
        let response = app.post_signup(&invalid_signup).await;
        assert_eq!(response.status().as_u16(), 400);
        let expected_response = ErrorResponse {
            error: "Invalid credentials".to_string(),
        };

        // Assert that we are getting the correct error message
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to UserBody"),
            expected_response
        );
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let mut app = TestApp::new().await;
    let user_json = json!({
        "email": "mreynolds@serenity.co",
        "password": "N0thingInTheverse!",
        "requires2FA": false
    });

    // Create user, ignoring the response
    let response = app.post_signup(&user_json).await;
    // It shoulld succeed
    assert_eq!(response.status().as_u16(), 201);

    // Attempt creating user again
    let response = app.post_signup(&user_json).await;
    // It should fail
    assert_eq!(response.status().as_u16(), 409);

    let expected_response = ErrorResponse {
        error: "User already exists".to_string(),
    };

    // Assert that we are getting the correct error message
    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
    app.clean_up().await;
}
