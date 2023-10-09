use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models::users;

#[derive(Debug,Validate, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min=1, message = "Name is required"))]
    pub name: String,
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Invalid email format")
    )]
    pub email: String,

    #[validate{
        length(min = 1, message = "Password is required"),
        length(min = 8, message = "Password must be at least 8 characters long")
    }]
    pub password: String,

    #[validate(
        length(min=1 , message = "Please confirm your password"),
        must_match(other = "password", message = "Passwords do not match")
    )]
    pub confirm_password: String,



}

#[derive(Debug,Validate, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Invalid email format")
    )]
    
    pub email: String,
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 8, message = "Password must be at least 8 characters long")
    )]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min =1, message = "Page must be greater than 0"))]
    pub page: Option<i64>,
    #[validate(
        range(min = 1, max = 50, message = "Limit must be between 1 and 50")
    )]
    pub limit: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterUserDto{
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_string(),
            email: user.email.to_string(),
            role: user.role.to_string(),
            photo: user.photo.to_string(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponseDto {
    pub status: String,
    pub users: Vec<FilterUserDto>,
    pub results: usize,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}
