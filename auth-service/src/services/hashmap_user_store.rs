use crate::domain::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(&user.email) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        }
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        // calls add_user with a user that doesn't exist
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "mreynolds@serenity.co".to_string(),
            password: "nothingintheverse".to_string(),
            requires_2fa: false,
        };

        // assert that the user was added
        assert_eq!(user_store.add_user(user.clone()), Ok(()));

        // assert re-adding the same user the failes with an error
        assert_eq!(
            user_store.add_user(user),
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "mreynolds@serenity.co".to_string(),
            password: "nothingintheverse".to_string(),
            requires_2fa: false,
        };
        // add the user to the store
        assert_eq!(user_store.add_user(user.clone()), Ok(()));
        // assert that the user can be retrieved
        assert_eq!(
            user_store
                .get_user(&user.email)
                .expect("failed to get user"),
            user
        );
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "mreynolds@serenity.co".to_string(),
            password: "nothingintheverse".to_string(),
            requires_2fa: false,
        };
        // add the user to the store
        assert_eq!(user_store.add_user(user.clone()), Ok(()));
        // assert that the user validates
        assert_eq!(
            user_store.validate_user(&user.email, &user.password),
            Ok(())
        );
    }
}
