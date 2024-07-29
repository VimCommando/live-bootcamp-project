use super::UserStoreError;
use validator::ValidateEmail;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Email, UserStoreError> {
        match ValidateEmail::validate_email(&email) {
            true => Ok(Email(email.to_string())),
            false => Err(UserStoreError::InvalidEmail),
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_invalid_email_returns_err() {
        assert_eq!(
            Email::parse("thisisnotanemail").unwrap_err(),
            UserStoreError::InvalidEmail
        )
    }

    #[test]
    fn parse_valid_email_returns_ok() {
        assert_eq!(
            Email::parse("name@example.com").unwrap(),
            Email("name@example.com".to_string())
        )
    }
}
