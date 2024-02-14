use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{app::user::user_roles::UserRoles, envs::TOKEN_EXPIRATION};

// TOKEN GENERATION FOR USER ----------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJWTTokenClaims {
    pub sub: String,        // user email
    pub roles: Vec<String>, // UserRole
    pub exp: usize,         // Unix timestamp (seconds since 1970) duration
}

impl UserJWTTokenClaims {
    pub fn new(sub: String, roles: &Vec<UserRoles>) -> Self {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("TIME ERROR");
        let exp = since_epoch.as_secs() as usize + *TOKEN_EXPIRATION;
        UserJWTTokenClaims {
            sub,
            roles: roles.into_iter().map(|role| role.to_string()).collect(),
            exp,
        }
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("TIME ERROR");
        since_epoch.as_secs() as usize > self.exp
    }

    pub fn refresh(&mut self) {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("TIME ERROR");
        self.exp = since_epoch.as_secs() as usize + *TOKEN_EXPIRATION;
    }

    pub fn get_roles(&self) -> Vec<UserRoles> {
        self.roles.iter().map(|role| UserRoles::from_str(role).unwrap()).collect()
    }

}
