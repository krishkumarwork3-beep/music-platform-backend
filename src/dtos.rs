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

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    pub id: String,
    pub username: String,
    pub email: String,

    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,
}

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            username: user.username.to_owned(),
            email: user.email.to_owned(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
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
pub struct UserLoginResponseDto {
    pub status: String,
    pub user: FilterUserDto,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required."),
        length(min = 6, message = "new password must be at least 6 characters")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "New password confirm is required."),
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required."),
        length(min = 6, message = "Old password must be at least 6 characters")
    )]
    pub old_password: String,
}

#[derive(Serialize)]
pub struct UploadResponse {
   pub track_id: uuid::Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncompleteTrackInfo {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub thumbnail_name: Option<String>,
    pub file_name: Option<String>,
    pub track_id: Option<uuid::Uuid>,
    pub total_chunks: i32,
    pub uploaded_chunks: i32,
    pub current_chunk: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncompleteTrackInfoResponse {
    pub incomplete_track_info: Vec<IncompleteTrackInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterTrackDto {
    pub id: uuid::Uuid,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration_minutes: f64,
    pub duration_seconds: f64,
    pub duration_played: f64,
    pub file_name: Option<String>,
    pub thumbnail_name: Option<String>,
    pub is_favorite: Option<bool>,
    pub played_at: Option<chrono::NaiveDateTime>,
    pub is_created_by_user: Option<bool>,
}

impl FilterTrackDto {
    pub fn filter_track(track: &TrackDto) -> Self {
        FilterTrackDto {
            id: track.id.clone(),
            title: track.title.clone(),
            artist: track.artist.clone(),
            duration_minutes: convert_duration_to_minutes(&track.duration),
            duration_seconds: convert_duration_to_seconds(&track.duration),
            duration_played: convert_duration_to_seconds(&track.duration_played),
            file_name: track.file_name.clone(),
            thumbnail_name: track.thumbnail_name.clone(),
            is_favorite: track.is_favorite.clone(),
            played_at: track.played_at.clone(),
            is_created_by_user: track.is_created_by_user.clone()
        }
    }
}