use std::fmt;  //formatting and printing output

use actix_web::{HttpResponse, ResponseError}; 
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String
}

//allows the stuct error response to be formatted as a string
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! (f, "{}", serde_json::to_string(&self).unwrap())
    }
}


#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

//defines various error messages that can be associated with HTTP errors
#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    HashingError,
    InvalidHashFormat,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied
}

// to string trait allows the error message to be converted to a string
impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned() //provides a string representation for each error variant
    }
}

// into trait allows the error message to be converted to a string
impl Into<String> for ErrorMessage {
    fn into(self) -> String {
        self.to_string()
    }
}

// display trait allows the error message to be displayed
impl ErrorMessage {
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            ErrorMessage::WrongCredentials => "Email or password is wromg".to_string(),
            ErrorMessage::EmailExist => "A user with this email already exists".to_string(),
            ErrorMessage::UserNoLongerExist => "User belonging to this token no longer exists".to_string(),
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            ErrorMessage::HashingError => "Error while hashing password".to_string(),
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            ErrorMessage::ExceededMaxPasswordLength => format!("Password must not be more than {} characters", max_length),
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide token".to_string(),
            ErrorMessage::PermissionDenied => "You are not allowed to perform this action".to_string()         

        }
    }
}

#[derive(Debug, Clone)]
pub struct HttpError {
    pub message: String,
    pub status: u16,
}

impl HttpError {
    pub fn new(message: impl Into<String> , status: u16) -> Self {
        HttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 500,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 400
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 409
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),
            status: 401
        }
    }

    // maps the error to an appropriate  Actix Web 'HttpResponse' based on the status code

    pub fn into_http_response(self) -> HttpResponse {
        match self.status {
            400 => HttpResponse::BadRequest().json(Response{
                status: "fail",
                message: self.message.into(),
            }),

            401 => HttpResponse::unauthorized().json(Response{
                status: "fail",
                message: self.message.into(),
            }),
            409 => HttpResponse::Conflict().json(Response {
                status: "fail",
                message: self.message.into(),
            }),
            500 => HttpResponse::InternalServerError().json(Response {
                status: "error",
                message: self.message.into(),
            }),
            _ => {
                eprintln! {
                    "Warning: Missing Pattern match. Converted status code {} to 500",
                    self.status
                    
                };

                HttpResponse::InternalServerError().json(Response {
                    status: "error",
                    message: self.message.into(),
                })
            }
        }
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write! (f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}

// error trait implementations for HttpError
impl std::error::Error for HttpError {}
impl ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let cloned = self.clone();
        cloned.into_http_response()
    }
}