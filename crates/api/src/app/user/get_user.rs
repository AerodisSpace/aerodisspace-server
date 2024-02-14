use std::{error::Error, str::FromStr};

use charybdis::operations::Find;
use error_mapper::errors::AerodisSpaceError;
use futures::StreamExt;
use scylla::{CachingSession, IntoTypedRows};

use crate::{
    database::scylladb::scylladb::build_session,
    dto::user::user_dto::{RequestFindUserDTO, ResponseUserDTO},
    models::user::user::UserModel,
};

use super::user_roles::UserRoles;

pub async fn get_one_user(data: RequestFindUserDTO) -> Result<ResponseUserDTO, AerodisSpaceError> {
    if !data.email.is_empty() {
        let session = CachingSession::from(build_session().await?, 5);
        let user = UserModel::find_by_email(&session, data.email)
            .await
            .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
            .next()
            .await;
        match user {
            Some(user_found) => {
                let user = user_found.map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?;
                Ok(ResponseUserDTO {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    roles: user.roles.into_iter().map(|role| role.parse::<UserRoles>().unwrap()).collect(),
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
            }
            None => Err(AerodisSpaceError::NotFound("User Not Found".to_string()))?,
        }
    } else {
        return Err(AerodisSpaceError::BadRequest("No data to search".to_string()))?;
    }
}

pub async fn get_all_users() -> Result<Vec<ResponseUserDTO>, AerodisSpaceError> {
    let session: CachingSession = CachingSession::from(build_session().await?, 5);
    let query = "SELECT * FROM user;";
    let mut user_list: Vec<ResponseUserDTO> = Vec::new();
    if let Some(rows) = session
        .execute(query, ())
        .await
        .map_err(|_err| AerodisSpaceError::Internal(_err.to_string()))?
        .rows
    {
        for row in rows.into_typed::<UserModel>() {
            match row {
                Ok(user) => {
                    user_list.push(ResponseUserDTO {
                        id: user.id,
                        username: user.username,
                        roles: user.roles.into_iter().map(|role| role.parse::<UserRoles>().unwrap()).collect(),
                        email: user.email,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    });
                }
                Err(_err) => {
                    return Err(AerodisSpaceError::Internal(_err.to_string()))?;
                }
            }
        }
    }
    Ok(user_list)
}
