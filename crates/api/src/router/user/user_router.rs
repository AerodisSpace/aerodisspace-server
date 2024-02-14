use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::Json;
use rocket::{get, post, routes, Route};

use crate::app::user::auth::login_user::login_user;
use crate::app::user::auth::signup_user::signup_user;
use crate::app::user::get_user::{get_all_users, get_one_user};
use crate::dto::user::user_auth_dto::{RequestLoginUserDTO, RequestRegisUserDTO, ResponseLoginUserDTO};
use crate::dto::user::user_dto::{RequestFindUserDTO, ResponseUserDTO};
use crate::dto::wrapper::{create_error_response, create_response, BaseResponseDTO, ResponseStatusDTO};
use crate::router::guards::jwt_token_guard::JWTTokenGuard;

#[post("/signup", data = "<data>")]
async fn signup_user_route(
    data: Json<RequestRegisUserDTO>,
) -> Result<Json<BaseResponseDTO<ResponseUserDTO>>, Custom<Json<BaseResponseDTO<()>>>> {
    match signup_user(data.into_inner()).await {
        Ok(resp) => Ok(create_response(resp, None)),
        Err(_err) => Err(create_error_response(_err)),
    }
}

#[post("/login", data = "<data>")]
async fn login_user_route(
    data: Json<RequestLoginUserDTO>,
) -> Result<Json<BaseResponseDTO<ResponseLoginUserDTO>>, Custom<Json<BaseResponseDTO<()>>>> {
    match login_user(data.into_inner()).await {
        Ok(resp) => Ok(create_response(resp, None)),
        Err(_err) => Err(create_error_response(_err)),
    }
}

#[post("/search", data = "<data>")]
async fn get_one_user_route(
    data: Json<RequestFindUserDTO>,
) -> Result<Json<BaseResponseDTO<ResponseUserDTO>>, Custom<Json<BaseResponseDTO<()>>>> {
    match get_one_user(data.into_inner()).await {
        Ok(resp) => Ok(create_response(resp, None)),
        Err(_err) => Err(create_error_response(_err)),
    }
}

#[get("/all")]
async fn get_all_users_route() -> Result<Json<BaseResponseDTO<Vec<ResponseUserDTO>>>, Custom<Json<BaseResponseDTO<()>>>> {
    match get_all_users().await {
        Ok(resp) => Ok(create_response(resp, None)),
        Err(_err) => Err(create_error_response(_err)),
    }
}

#[get("/test-token")]
async fn test_token(_token: JWTTokenGuard) -> String {
    String::from("Token is valid")
}

pub fn user_router() -> Vec<Route> {
    routes![
        get_one_user_route,
        signup_user_route,
        get_all_users_route,
        login_user_route,
        test_token
    ]
}
