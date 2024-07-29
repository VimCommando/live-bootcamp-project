use super::UserStoreError;

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Password, UserStoreError> {
        match password {
            password if password.chars().count() < 8 || password.chars().count() > 256 => {
                Err(UserStoreError::InvalidCredentials)
            }
            password if !password.contains(|c: char| c.is_uppercase()) => {
                Err(UserStoreError::InvalidCredentials)
            }
            password if !password.contains(|c: char| c.is_lowercase()) => {
                Err(UserStoreError::InvalidCredentials)
            }
            password if !password.contains(|c: char| c.is_numeric()) => {
                Err(UserStoreError::InvalidCredentials)
            }
            password => Ok(Password(password.to_string())),
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_number_returns_err() {
        assert_eq!(
            Password::parse("Password!"),
            Err(UserStoreError::InvalidCredentials)
        );
    }

    #[test]
    fn missing_uppercase_returns_err() {
        assert_eq!(
            Password::parse("password1!"),
            Err(UserStoreError::InvalidCredentials)
        );
    }

    #[test]
    fn short_password_returns_err() {
        assert_eq!(
            Password::parse("Pass1!"),
            Err(UserStoreError::InvalidCredentials)
        );
    }

    #[test]
    fn long_password_returns_err() {
        assert_eq!(
            Password::parse("Passwoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooord1!"),
            Err(UserStoreError::InvalidCredentials)
        );
    }

    #[test]
    fn valid_password_returns_ok() {
        assert_eq!(
            Password::parse("Password1!"),
            Ok(Password("Password1!".to_string()))
        );
    }
}
