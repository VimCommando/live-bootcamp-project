use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::{BannedTokenStore, EmailClient, TwoFACodeStore, UserStore};

// Using a type alias to improve readability!
pub type BannedTokenStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;
pub type EmailClientType = Arc<dyn EmailClient + Send + Sync>;
pub type TwoFACodeStoreType = Arc<RwLock<dyn TwoFACodeStore + Send + Sync>>;
pub type UserStoreType = Arc<RwLock<dyn UserStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState {
    pub banned_token_store: BannedTokenStoreType,
    pub email_client: EmailClientType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(
        banned_token_store: BannedTokenStoreType,
        email_client: EmailClientType,
        two_fa_code_store: TwoFACodeStoreType,
        user_store: UserStoreType,
    ) -> Self {
        Self {
            banned_token_store,
            email_client,
            two_fa_code_store,
            user_store,
        }
    }
}
