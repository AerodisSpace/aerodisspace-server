use rocket::{catch, catchers, response::status::Custom, routes, serde::json::Json};

use crate::dto::base_dto::{create_error_response, BaseResponseDTO};

#[catch(404)]
fn not_found() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::NotFound("Not Found".to_string()));
    Ok(response)
}

#[catch(401)]
fn unauthorized() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::Auth("Unauthorized".to_string()));
    Ok(response)
}




pub fn catcher_router() -> Vec<rocket::catcher::Catcher> {
    catchers![not_found, unauthorized]
}
