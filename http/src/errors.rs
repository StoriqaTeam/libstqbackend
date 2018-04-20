use hyper;
use serde_json;

use failure::Error;

use validator::ValidationErrors;

#[derive(Debug, Fail)]
pub enum ControllerError {
    #[fail(display = "Not found")]
    NotFound,
    #[fail(display = "Parse error: {}", _0)]
    Parse(String),
    #[fail(display = "Bad request: {}", _0)]
    BadRequest(Error),
    #[fail(display = "Validation error: {}", _0)]
    Validate(ValidationErrors),
    #[fail(display = "Unprocessable entity: {}", _0)]
    UnprocessableEntity(Error),
    #[fail(display = "Internal server error: {}", _0)]
    InternalServerError(Error),
    #[fail(display = "Server is refusing to fullfil the reqeust: {}", _0)]
    Forbidden(Error),
}

impl From<serde_json::error::Error> for ControllerError {
    fn from(e: serde_json::error::Error) -> Self {
        ControllerError::UnprocessableEntity(e.into())
    }
}

impl ControllerError {
    /// Converts `Error` to HTTP Status Code
    pub fn code(&self) -> hyper::StatusCode {
        use hyper::StatusCode;

        match *self {
            ControllerError::NotFound => StatusCode::NotFound,
            ControllerError::Parse(_) | ControllerError::BadRequest(_) | ControllerError::Validate(_) => StatusCode::BadRequest,
            ControllerError::UnprocessableEntity(_) => StatusCode::UnprocessableEntity,
            ControllerError::InternalServerError(_) => StatusCode::InternalServerError,
            ControllerError::Forbidden(_) => StatusCode::Forbidden,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}
