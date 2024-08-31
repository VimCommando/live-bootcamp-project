use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}

lazy_static! {
    pub static ref JWT_SECRET: String = set_env(env::JWT_SECRET_ENV_VAR, None);
    pub static ref DATABASE_URL: String = set_env(env::DATABASE_URL_ENV_VAR, None);
    pub static ref REDIS_HOST_NAME: String =
        set_env(env::REDIS_HOST_NAME_ENV_VAR, Some("127.0.0.1"));
}

fn set_env(name: &str, default: Option<&str>) -> String {
    dotenv().ok();
    match std_env::var(name) {
        Ok(value) if value.is_empty() => default
            .expect(&format!("{name} must not be empty."))
            .to_string(),
        Ok(value) => value,
        Err(_) if default.is_none() => panic!("{name} has no default."),
        Err(_) => default
            .expect(&format!("{name} default failed!"))
            .to_string(),
    }
}

pub const JWT_COOKIE_NAME: &str = "jwt";
