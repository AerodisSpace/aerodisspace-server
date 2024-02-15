use std::collections::HashSet;

use crate::{repository::Repository, user::auth::UserJWTTokenClaims};
use auth::{
    fields::{check_email_field, check_password_field},
    hash::{generate_hash, verify_hash},
    jwt_token::jwt::generate_jwt_token,
};
use database::scylla::{transport::session, CachingSession, IntoTypedRows};
use database::scylladb::connection::build_session;
use dto::user::{
    auth::{UserLoginRequestDTO, UserLoginResponseDTO, UserSignupRequestDTO, UserSignupResponseDTO},
    free::UserGetResponseDTO,
};
use envs::SALT_SECRET;
use error_mapper::errors::AerodisSpaceError;
use fake::{faker::internet::en::Username, Fake};
use futures::StreamExt;
use models::charybdis::{
    model::BaseModel,
    operations::{Delete, Find, Insert, Update},
};
use models::user::UserModel;

use super::{auth::Auth, roles::UserRoles};
/// User struct that wraps the logic for users
pub struct User;
#[async_trait::async_trait]
impl Repository<UserModel> for User {
    // In this case the field is the email
    async fn get_one(field: String, session: &CachingSession) -> Result<UserModel, error_mapper::errors::AerodisSpaceError> {
        if field.is_empty() {
            return Err(AerodisSpaceError::InvalidField("No data to search".to_string()));
        }
        let user = UserModel::find_by_email(session, field)
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
            .next()
            .await;
        match user {
            Some(user_found) => {
                let user = user_found.map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
                Ok(user)
            }
            None => Err(AerodisSpaceError::NotFound("User Not Found".to_string())),
        }
    }

    async fn get_all(field: Option<String>, session: &CachingSession) -> Result<Vec<UserModel>, error_mapper::errors::AerodisSpaceError> {
        let mut user_list: Vec<UserModel> = Vec::new();
        // UserModel::FIND_BY_PARTITION_KEY_QUERY

        // TODO: FIX THIS FIND QUERY
        let mut users = UserModel::find(session, "email NOT NULL", &[])
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
        while let Some(user) = users.next().await {
            match user {
                Ok(user) => user_list.push(user),
                Err(_err) => return Err(AerodisSpaceError::Internal(_err.to_string())),
            }
        }
        Ok(user_list)
    }

    async fn create(data: &mut UserModel, session: &CachingSession) -> Result<UserModel, error_mapper::errors::AerodisSpaceError> {
        // Note: In this case data has a password not hashed, this come from dto
        check_password_field(&data.password)?;
        check_email_field(&data.email)?;

        let email_used = match UserModel::find_by_email(session, data.email.clone())
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
            .next()
            .await
        {
            Some(user_found) => {
                let user = user_found.map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
                if user.email == data.email {
                    Err(AerodisSpaceError::AlreadyExists("Email already used".to_string()))?;
                }
            }
            None => (),
        };

        let hashed_password = generate_hash(&data.password)?;
        let now = chrono::Utc::now();

        data.password = hashed_password;
        if data.username.is_empty() {
            data.username = Username().fake();
        }
        data.id = uuid::Uuid::new_v4();
        data.created_at = now;
        data.updated_at = now;

        data.insert_if_not_exists(&session)
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;

        let user_found = UserModel::find_by_email(session, data.email.clone())
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
            .next()
            .await;
        match user_found {
            Some(user) => {
                let user = user.map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
                Ok(user)
            }
            None => Err(AerodisSpaceError::Internal("User not created".to_string())),
        }
    }

    async fn update(data: UserModel, session: &CachingSession) -> Result<UserModel, error_mapper::errors::AerodisSpaceError> {
        // Note: In this case data has a password not hashed, this come from dto
        let mut user_found = Self::get_one(data.email.clone(), session).await?;
        let new_password = generate_hash(&data.password)?;

        let now = chrono::Utc::now();
        user_found.username = data.username;
        user_found.email = data.email;
        user_found.password = new_password;
        user_found.roles = data.roles;
        user_found.updated_at = now;

        user_found
            .update(&session)
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
        Ok(user_found)
    }

    async fn delete(field: String, session: &CachingSession) -> Result<(), error_mapper::errors::AerodisSpaceError> {
        let user = Self::get_one(field.clone(), session).await?;
        user.delete(&session)
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl Auth for User {
    async fn login(data: UserLoginRequestDTO, session: &CachingSession) -> Result<UserLoginResponseDTO, AerodisSpaceError> {
        if data.email.is_empty() || data.password.is_empty() {
            return Err(AerodisSpaceError::BadRequest("Email and Password is required".to_string()));
        }
        let user = Self::get_one(data.email.clone(), session).await?;
        if verify_hash(&data.password, &user.password)? {
            let roles: HashSet<UserRoles> = user
                .roles
                .clone()
                .into_iter()
                .map(|role| role.parse::<UserRoles>().unwrap())
                .collect();
            let claim = UserJWTTokenClaims::new(user.email.clone(), roles);
            let token = generate_jwt_token(&SALT_SECRET, claim)?;
            Ok(UserLoginResponseDTO {
                id: user.id,
                username: user.username,
                email: user.email,
                roles: user.roles,
                created_at: user.created_at,
                updated_at: user.updated_at,
                token,
            })
        } else {
            return Err(AerodisSpaceError::BadRequest("Password is incorrect".to_string()));
        }
    }

    async fn signup(data: UserSignupRequestDTO, session: &CachingSession) -> Result<UserSignupResponseDTO, AerodisSpaceError> {
        let mut roles = HashSet::new();
        roles.insert(UserRoles::FreeUser.to_string());
        let user = Self::create(
            &mut UserModel {
                username: data.username,
                email: data.email,
                password: data.password,
                roles,
                ..Default::default()
            },
            session,
        )
        .await;
        match user {
            Ok(user) => Ok(UserSignupResponseDTO {
                id: user.id,
                username: user.username,
                email: user.email,
                roles: user.roles,
                created_at: user.created_at,
                updated_at: user.updated_at,
            }),
            Err(err) => Err(err),
        }
    }
}
