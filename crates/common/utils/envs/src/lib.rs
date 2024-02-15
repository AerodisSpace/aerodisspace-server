pub use once_cell;
use once_cell::sync::Lazy;
use std::env::var;

pub static DEBUG_MODE: Lazy<bool> = Lazy::new(|| var("DEBUG_MODE").unwrap_or("false".to_string()) == "true");

pub static SALT_SECRET: Lazy<String> = Lazy::new(|| var("SALT_SECRET").expect("NEED SET A SALT_SECRET"));

pub static TOKEN_EXPIRATION: Lazy<usize> = Lazy::new(|| var("TOKEN_EXPIRATION").unwrap_or("3600".to_string()).parse().unwrap());
