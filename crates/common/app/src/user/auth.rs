use std::{
    collections::HashSet,
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use database::scylla::CachingSession;
use dto::user::auth::{UserLoginRequestDTO, UserLoginResponseDTO, UserSignupRequestDTO, UserSignupResponseDTO};
use envs::TOKEN_EXPIRATION;
use error_mapper::errors::AerodisSpaceError;
use serde::{Deserialize, Serialize};

use super::roles::UserRoles;

/// Auth trait that defines the methods that a user should implement
#[async_trait::async_trait]
pub trait Auth {
    async fn login(data: UserLoginRequestDTO, session: &CachingSession) -> Result<UserLoginResponseDTO, AerodisSpaceError>;
    async fn signup(data: UserSignupRequestDTO, session: &CachingSession) -> Result<UserSignupResponseDTO, AerodisSpaceError>;
}

// JWT TOKEN CLAIMS
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJWTTokenClaims {
    pub sub: String,            // user email
    pub roles: HashSet<String>, // UserRole
    pub exp: usize,             // Unix timestamp (seconds since 1970) duration
}

impl UserJWTTokenClaims {
    pub fn new(sub: String, roles: HashSet<UserRoles>) -> Self {
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

    pub fn get_roles(&self) -> Result<HashSet<UserRoles>, AerodisSpaceError> {
        let user_roles: HashSet<UserRoles> = self
            .roles
            .clone()
            .into_iter()
            .filter_map(|role_str| UserRoles::from_str(&role_str).ok())
            .collect();
        Ok(user_roles)
    }
}
