use std::fmt::{self, Display, Formatter};

use rand::Rng;
use serde::Serialize;
use uuid::Uuid;

use super::{Email, Password, User};

#[derive(Debug, PartialEq, Serialize)]
pub enum UserStoreError {
    InvalidCredentials,
    InvalidEmail,
    UnexpectedError,
    UserAlreadyExists,
    UserNotFound,
}

#[async_trait::async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;
    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[async_trait::async_trait]
pub trait BannedTokenStore {
    async fn add_token(&mut self, token: String) -> Result<(), BannedTokenStoreError>;
    async fn contains_token(&self, token: &str) -> Result<bool, BannedTokenStoreError>;
}

#[derive(Debug)]
pub enum BannedTokenStoreError {
    UnexpectedError,
}

#[async_trait::async_trait]
pub trait TwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError>;
    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError>;
    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn parse(id: &str) -> Result<Self, String> {
        match Uuid::parse_str(&id) {
            Ok(_) => Ok(LoginAttemptId(id.to_string())),
            Err(_) => Err("Invalid UUID".to_string()),
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        LoginAttemptId(Uuid::new_v4().to_string())
    }
}

impl Display for LoginAttemptId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: &str) -> Result<Self, String> {
        match code.parse::<u32>() {
            Ok(_) if code.len() == 6 => Ok(TwoFACode(code.to_string())),
            _ => Err("Invalid 2FA code".to_string()),
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        let code = rand::thread_rng().gen_range(100000..999999).to_string();
        TwoFACode(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
