use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::{validate_email, Validate, ValidationError};
use regex::Regex;

use crate::models::{Duration, User};

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 3, message = "Username must be at least 3 characters long"))]
    #[validate(custom = "validate_username")]
    pub username: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,

    #[
        validate(
            length(min = 1, message = "Confirm Password is required"),
            must_match(other = "password", message="passwords do not match")
        )
    ]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

    if !re.is_match(username) {
        return Err(ValidationError::new(
            "Username can only contain letters, numbers, and underscores",
        ));
    }

    Ok(())
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(custom = "validate_identifier")]
    pub identifier: String,

    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}

fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
    if identifier.contains('@') {
        if !validate_email(identifier) {
            return Err(ValidationError::new("Invalid email format"));
        }
    } else if identifier.len() < 3 {
        return Err(ValidationError::new(
            "Username must be at least 3 characters long",
        ));
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Validate)]
pub struct RequestQueryDto {
    #[validate(range(min = 1))]
    pub page: Option<usize>,

    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,
}