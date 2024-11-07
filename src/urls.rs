use rocket::serde::{Deserialize, Serialize};
use sqlx::types::{chrono::NaiveDateTime, Uuid};
use validators::{is_valid_description, is_valid_long_url, is_valid_title};

use crate::Validate;

pub mod handlers;
mod repo;
mod validators;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Url {
    id: Uuid,
    creator: Uuid,
    title: String,
    description: Option<String>,
    long_url: String,
    short_url: String,
    times_visited: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateBody {
    title: String,
    description: Option<String>,
    long_url: String,
}

impl Validate for CreateBody {
    fn validate(&self) -> bool {
        is_valid_title(&self.title)
            && is_valid_long_url(&self.long_url)
            && self
                .description
                .as_ref()
                .map_or(true, |d| is_valid_description(d))
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PatchBody {
    title: Option<String>,
    description: Option<String>,
}

impl Validate for PatchBody {
    fn validate(&self) -> bool {
        self.title.as_ref().map_or(true, |t| is_valid_title(t))
            && self
                .description
                .as_ref()
                .map_or(true, |d| is_valid_description(d))
    }
}
