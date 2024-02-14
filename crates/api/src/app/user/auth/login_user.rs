use std::time::{SystemTime, UNIX_EPOCH};

use auth::{
    hash::{generate_hash, verify_hash},
    jwt_token::jwt::{generate_jwt_token, jsonwebtoken::TokenData, verify_jwt_token},
};
use error_mapper::errors::AerodisSpaceError;
use futures::StreamExt;
use scylla::CachingSession;
use serde::{Deserialize, Serialize};

use crate::{
    app::user::user_roles::UserRoles,
    database::scylladb::scylladb::build_session,
    dto::user::user_auth_dto::{RequestLoginUserDTO, ResponseLoginUserDTO},
    envs::{SALT_SECRET, TOKEN_EXPIRATION},
    models::user::user::UserModel,
    utils::jwt_token_claims::UserJWTTokenClaims,
};

pub async fn login_user(data: RequestLoginUserDTO) -> Result<ResponseLoginUserDTO, AerodisSpaceError> {
    if data.email.is_empty() {
        return Err(AerodisSpaceError::BadRequest("Email is required".to_string()));
    }

    let session: CachingSession = CachingSession::from(build_session().await?, 1);
    let user = UserModel::find_by_email(&session, data.email)
        .await
        .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
        .next()
        .await;
    match user {
        Some(user_found) => {
            let user = user_found.map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
            let compare_hash_password = verify_hash(&data.password, &user.password)?;
            if compare_hash_password {
                let roles: Vec<UserRoles> = user.roles.into_iter().map(|role| role.parse::<UserRoles>().unwrap()).collect();
                let claim = UserJWTTokenClaims::new(user.email.clone(), &roles);
                let secret = SALT_SECRET.clone();
                let token = generate_jwt_token(&secret, claim)?;
                Ok(ResponseLoginUserDTO {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    roles,
                    token,
                })
            } else {
                return Err(AerodisSpaceError::BadRequest("Password is incorrect".to_string()));
            }
        }
        None => {
            return Err(AerodisSpaceError::NotFound(
                "User Not Found. Check your email and password".to_string(),
            ));
        }
    }
}
