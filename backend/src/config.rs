use lazy_static::lazy_static;
use std::env;

pub const TOKEN_EXPIRATION_TIME: usize = 60 * 60 * 24 * 15; // 15 days
pub const MAX_POOL_SIZE: u32 = 5; // Database connection pool size

lazy_static! {
    pub static ref SECRET_KEY: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
}
