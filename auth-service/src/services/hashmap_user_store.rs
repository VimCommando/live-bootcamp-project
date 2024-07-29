use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.get(&user.email) {
            Some(_) => Err(UserStoreError::UserAlreadyExists),
            None => {
                self.users.insert(user.email.clone(), user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(&email) {
            Some(user) => {
                if &user.password == password {
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
    async fn add_user() {
        // calls add_user with a user that doesn't exist
        let mut user_store = HashmapUserStore::default();
        let user = User::new(
            Email::parse("mreynolds@serenity.co").unwrap(),
            Password::parse("N0thingInTheverse!").unwrap(),
            false,
        );

        // assert that the user was added
        assert_eq!(user_store.add_user(user.clone()).await, Ok(()));

        // assert re-adding the same user the failes with an error
        assert_eq!(
            user_store.add_user(user).await,
            Err(UserStoreError::UserAlreadyExists)
        );
    }

    #[tokio::test]
    async fn get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new(
            Email::parse("mreynolds@serenity.co").unwrap(),
            Password::parse("N0thingInTheverse!").unwrap(),
            false,
        );

        // add the user to the store
        assert_eq!(user_store.add_user(user.clone()).await, Ok(()));
        // assert that the user can be retrieved
        assert_eq!(
            user_store
                .get_user(&user.email)
                .await
                .expect("failed to get user"),
            user
        );
    }

    #[tokio::test]
    async fn validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new(
            Email::parse("mreynolds@serenity.co").unwrap(),
            Password::parse("N0thingInTheverse!").unwrap(),
            false,
        );

        // add the user to the store
        assert_eq!(user_store.add_user(user.clone()).await, Ok(()));
        // assert that the user validates
        assert_eq!(
            user_store.validate_user(&user.email, &user.password).await,
            Ok(())
        );
    }
}
