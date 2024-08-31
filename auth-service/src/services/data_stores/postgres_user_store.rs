use std::error::Error;

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::{postgres::PgDatabaseError, PgPool, Row};

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    Email, Password, User,
};

pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore for PostgresUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let hashed_password = compute_password_hash(user.password.as_ref())
            .map_err(|_| UserStoreError::UnexpectedError)?;

        sqlx::query("INSERT INTO users (email, password_hash, requires_2fa) VALUES ($1, $2, $3)")
            .bind(user.email.as_ref())
            .bind(hashed_password)
            .bind(user.requires_2fa)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                print!("Error: {:?}", &e);
                match e.into_database_error().unwrap().is_unique_violation() {
                    true => UserStoreError::UserAlreadyExists,
                    false => UserStoreError::UnexpectedError,
                }
            })?;

        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        let row = sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email.as_ref())
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => UserStoreError::UserNotFound,
                _ => UserStoreError::UnexpectedError,
            })?;

        let user = User::new(
            Email::parse(row.get("email")).map_err(|_| UserStoreError::UserNotFound)?,
            Password::parse(row.get("password_hash"))
                .map_err(|_| UserStoreError::UnexpectedError)?,
            row.get("requires_2fa"),
        );
        Ok(user)
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        let user = self
            .get_user(email)
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        verify_password_hash(user.password.as_ref(), password.as_ref())
            .map_err(|_| UserStoreError::InvalidCredentials)?;

        Ok(())
    }
}

// Helper function to verify if a given password matches an expected hash
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
fn verify_password_hash(
    expected_password_hash: &str,
    password_candidate: &str,
) -> Result<(), Box<dyn Error>> {
    let expected_password_hash: PasswordHash<'_> = PasswordHash::new(expected_password_hash)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .map_err(|e| e.into())
}

// Helper function to hash passwords before persisting them in the database.
// TODO: Hashing is a CPU-intensive operation. To avoid blocking
// other async tasks, update this function to perform hashing on a
// separate thread pool using tokio::task::spawn_blocking. Note that you
// will need to update the input parameters to be String types instead of &str
fn compute_password_hash(password: &str) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();

    Ok(password_hash)
}

// #[cfg(test)]
// mod tests {
//     use crate::{get_postgres_pool, utils::constants::DATABASE_URL};

//     use super::*;

//     #[tokio::test]
//     async fn add_user() {
//         let pool = get_postgres_pool(&DATABASE_URL).await.unwrap();
//         let mut user_store = PostgresUserStore::new(pool);
//         // calls add_user with a user that doesn't exist
//         let user = User::new(
//             Email::parse("mreynolds@serenity.co").unwrap(),
//             Password::parse("N0thingInTheverse!").unwrap(),
//             false,
//         );

//         // assert that the user was added
//         assert_eq!(user_store.add_user(user.clone()).await, Ok(()));

//         // assert re-adding the same user the failes with an error
//         assert_eq!(
//             user_store.add_user(user).await,
//             Err(UserStoreError::UserAlreadyExists)
//         );
//     }

//     #[tokio::test]
//     async fn get_user() {
//         let pool = get_postgres_pool(&DATABASE_URL).await.unwrap();
//         let mut user_store = PostgresUserStore::new(pool);
//         let user = User::new(
//             Email::parse("mreynolds@serenity.co").unwrap(),
//             Password::parse("N0thingInTheverse!").unwrap(),
//             false,
//         );

//         // add the user to the store
//         assert_eq!(user_store.add_user(user.clone()).await, Ok(()));
//         // assert that the user can be retrieved
//         assert_eq!(
//             user_store
//                 .get_user(&user.email)
//                 .await
//                 .expect("failed to get user"),
//             user
//         );
//     }

//     #[tokio::test]
//     async fn validate_user() {
//         let pool = get_postgres_pool(&DATABASE_URL).await.unwrap();
//         let mut user_store = PostgresUserStore::new(pool);
//         let user = User::new(
//             Email::parse("mreynolds@serenity.co").unwrap(),
//             Password::parse("N0thingInTheverse!").unwrap(),
//             false,
//         );

//         // add the user to the store
//         assert_eq!(user_store.add_user(user.clone()).await, Ok(()));
//         // assert that the user validates
//         assert_eq!(
//             user_store.validate_user(&user.email, &user.password).await,
//             Ok(())
//         );
//     }
// }
