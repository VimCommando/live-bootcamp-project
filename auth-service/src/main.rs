use auth_service::{
    app_state::AppState,
    services::{HashmapTwoFACodeStore, HashmapUserStore, HashsetBannedTokenStore, MockEmailClient},
    utils::constants::prod,
    Application,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let mock_email_client = Arc::new(MockEmailClient::default());
    let app_state = AppState::new(
        banned_token_store,
        mock_email_client,
        two_fa_code_store,
        user_store,
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
