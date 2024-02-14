use rocket::{catch, catchers, response::status::Custom, routes, serde::json::Json};

use crate::dto::wrapper::{create_error_response, BaseResponseDTO};

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

#[catch(400)]
fn bad_request() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::BadRequest("Bad Request".to_string()));
    Ok(response)
}

#[catch(500)]
fn internal_server_error() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::Internal(
        "Internal Server Error".to_string(),
    ));
    Ok(response)
}

#[catch(403)]
fn forbidden() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::Auth("Forbidden".to_string()));
    Ok(response)
}

#[catch(405)]
fn method_not_allowed() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::BadRequest(
        "Method Not Allowed".to_string(),
    ));
    Ok(response)
}

#[catch(422)]
fn unprocessable_entity() -> Result<Custom<Json<BaseResponseDTO<()>>>, Custom<Json<BaseResponseDTO<()>>>> {
    let response = create_error_response(error_mapper::errors::AerodisSpaceError::BadRequest(
        "Unprocessable Entity".to_string(),
    ));
    Ok(response)
}

pub fn catcher_router() -> Vec<rocket::catcher::Catcher> {
    catchers![
        not_found,
        unauthorized,
        bad_request,
        internal_server_error,
        forbidden,
        method_not_allowed,
        unprocessable_entity
    ]
}
