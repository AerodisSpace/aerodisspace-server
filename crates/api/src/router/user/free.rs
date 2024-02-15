use app::repository::Repository;
use app::user::{auth::Auth, user::User};

use dto::user::auth::{UserLoginRequestDTO, UserLoginResponseDTO, UserSignupRequestDTO, UserSignupResponseDTO};
use dto::user::free::{UserGetOneRequestDTO, UserGetResponseDTO};
use error_mapper::errors::AerodisSpaceError;
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};

use crate::router::guards::user_jwt_token::JWTTokenGuard;
use crate::server::shared::SCYLLADB_SESSION;
use dto::wrapper::{create_error_response, create_response, BaseResponseDTO, ResponseStatusDTO};

#[post("/signup", data = "<data>")]
pub async fn user_route_signup(
    data: Json<UserSignupRequestDTO>,
) -> Result<Json<BaseResponseDTO<UserSignupResponseDTO>>, Custom<Json<BaseResponseDTO<()>>>> {
    let session = match SCYLLADB_SESSION.get() {
        Some(session) => session,
        None => {
            return Err(create_error_response(AerodisSpaceError::Internal(
                "Error getting scylladb session".to_string(),
            )))
        }
    };
    match User::signup(data.into_inner(), session).await {
        Ok(response) => return Ok(create_response(response, None)),
        Err(_err) => return Err(create_error_response(_err)),
    };
}

#[post("/login", data = "<data>")]
pub async fn user_route_login(
    data: Json<UserLoginRequestDTO>,
) -> Result<Json<BaseResponseDTO<UserLoginResponseDTO>>, Custom<Json<BaseResponseDTO<()>>>> {
    let session = match SCYLLADB_SESSION.get() {
        Some(session) => session,
        None => {
            return Err(create_error_response(AerodisSpaceError::Internal(
                "Error getting scylladb session".to_string(),
            )))
        }
    };
    match User::login(data.into_inner(), session).await {
        Ok(response) => return Ok(create_response(response, None)),
        Err(_err) => return Err(create_error_response(_err)),
    };
}

#[post("/search", data = "<data>")]
pub async fn user_route_search(
    data: Json<UserGetOneRequestDTO>,
) -> Result<Json<BaseResponseDTO<Vec<UserGetResponseDTO>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let session = match SCYLLADB_SESSION.get() {
        Some(session) => session,
        None => {
            return Err(create_error_response(AerodisSpaceError::Internal(
                "Error getting scylladb session".to_string(),
            )))
        }
    };
    let data = data.into_inner();
    if data.email.is_none() {
        match User::get_all(None, session).await {
            Ok(users) => {
                let response = users
                    .into_iter()
                    .map(|user| UserGetResponseDTO {
                        id: user.id,
                        username: user.username,
                        roles: user.roles.into_iter().map(|role| role.to_string()).collect(),
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    })
                    .collect();
                return Ok(create_response(response, None));
            }
            Err(_err) => return Err(create_error_response(_err)),
        }
    } else {
        match User::get_one(data.email.unwrap(), session).await {
            Ok(user) => {
                let response = vec![UserGetResponseDTO {
                    id: user.id,
                    username: user.username,
                    roles: user.roles.into_iter().map(|role| role.to_string()).collect(),
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                }];
                return Ok(create_response(response, None));
            }
            Err(_err) => return Err(create_error_response(_err)),
        };
    }
}

#[get("/test-token")]
pub async fn test_token(_token: JWTTokenGuard) -> String {
    String::from("Token is valid")
}
