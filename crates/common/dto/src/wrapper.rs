use envs::DEBUG_MODE;
use error_mapper::errors::AerodisSpaceError;
use rocket::{http::Status, response::status::Custom, serde::json::Json};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct BaseResponseDTO<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub status: ResponseStatusDTO,
}

impl<T> BaseResponseDTO<T> {
    pub fn new(data: Option<T>, message: Option<String>, status: ResponseStatusDTO) -> Self {
        match status {
            ResponseStatusDTO::ERROR => logger::tracing::error!("Response Error: {:?}", message),
            _ => {}
        }

        Self { data, message, status }
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
pub enum ResponseStatusDTO {
    SUCCESS,
    ERROR,
    WARNING,
}
impl ToString for ResponseStatusDTO {
    fn to_string(&self) -> String {
        match self {
            ResponseStatusDTO::SUCCESS => "SUCCESS".to_string(),
            ResponseStatusDTO::ERROR => "ERROR".to_string(),
            ResponseStatusDTO::WARNING => "WARNING".to_string(),
        }
    }
}

pub fn create_response<T>(data: T, message: Option<String>) -> Json<BaseResponseDTO<T>> {
    let response = BaseResponseDTO::new(Some(data), message, ResponseStatusDTO::SUCCESS);
    Json(response)
}

pub fn create_error_response<T>(err: AerodisSpaceError) -> Custom<Json<BaseResponseDTO<T>>> {
    let message;
    let status: Status;
    match err {
        AerodisSpaceError::Internal(msg) => {
            message = if *DEBUG_MODE { Some(msg) } else { None };
            status = Status::InternalServerError;
        }
        AerodisSpaceError::BadRequest(msg) => {
            message = Some(msg);
            status = Status::BadRequest;
        }
        AerodisSpaceError::NotFound(msg) => {
            message = Some(msg);
            status = Status::NotFound;
        }
        AerodisSpaceError::InvalidField(msg) => {
            message = Some(msg);
            status = Status::BadRequest;
        }
        AerodisSpaceError::AlreadyExists(msg) => {
            message = Some(msg);
            status = Status::BadRequest;
        }
        AerodisSpaceError::Auth(msg) => {
            message = Some(msg);
            status = Status::Unauthorized;
        }
    }
    let error_response = BaseResponseDTO::<T>::new(None, message, ResponseStatusDTO::ERROR);

    Custom(status, Json(error_response))
}
