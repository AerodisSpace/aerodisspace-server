use auth::{
    fields::{check_email_field, check_password_field},
    hash::{generate_hash, verify_hash},
};
use charybdis::{
    model::BaseModel,
    operations::{Find, Insert},
};
use error_mapper::errors::AerodisSpaceError;
use fake::{faker::internet::en::Username, Fake};
use futures::stream::StreamExt;
use rocket::form::validate::Len;
use scylla::CachingSession;
use std::{borrow::BorrowMut, collections::HashSet, error::Error};
use uuid::Uuid;

use crate::{
    app::user::user_roles::UserRoles,
    database::scylladb::scylladb::build_session,
    dto::user::{user_auth_dto::RequestRegisUserDTO, user_dto::ResponseUserDTO},
    models::user::user::UserModel,
};

pub async fn signup_user(data: RequestRegisUserDTO) -> Result<ResponseUserDTO, AerodisSpaceError> {
    let session: CachingSession = CachingSession::from(build_session().await?, 5);

    check_password_field(&data.password)?;
    check_email_field(&data.email)?;
    // Check if email is used
    let email_used = match UserModel::find_by_email(&session, data.email.clone())
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

    let mut username = data.username;
    if username.is_empty() {
        username = Username().fake();
    }
    let password_hash = generate_hash(&data.password)?;
    if !verify_hash(&data.password, &password_hash)? {
        Err(AerodisSpaceError::Internal(String::default()))?;
    }

    let now = chrono::Utc::now();
    let mut role = HashSet::new();
    role.insert(UserRoles::FreeUser.to_string());

    let user: UserModel = UserModel {
        id: Uuid::new_v4(),
        email: data.email,
        username: username,
        password: password_hash,
        roles: role,
        created_at: now,
        updated_at: now,
    };

    user.insert_if_not_exists(&session)
        .await
        .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;

    let user_found = UserModel::find_by_email(&session, user.email)
        .await
        .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
        .next()
        .await;
    match user_found {
        Some(user) => {
            let user = user.map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
            let response = ResponseUserDTO {
                id: user.id,
                email: user.email,
                username: user.username,
                roles: user.roles.into_iter().map(|role| role.parse::<UserRoles>().unwrap()).collect(),
                created_at: user.created_at,
                updated_at: user.updated_at,
            };
            Ok(response)
        }
        None => Err(AerodisSpaceError::Internal("User after created not found".to_string()))?,
    }
}
