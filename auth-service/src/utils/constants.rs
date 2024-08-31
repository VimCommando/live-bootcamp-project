use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

lazy_static! {
    pub static ref JWT_SECRET: String = set_env(env::JWT_SECRET_ENV_VAR);
    pub static ref DATABASE_URL: String = set_env(env::DATABASE_URL_ENV_VAR);
}

fn set_env(name: &str) -> String {
    dotenv().ok();
    match std_env::var(name) {
        Err(_) => panic!("{name} must be set."),
        Ok(value) if value.is_empty() => panic!("{name} must not be empty."),
        Ok(value) => value,
    }
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
}

pub const JWT_COOKIE_NAME: &str = "jwt";
