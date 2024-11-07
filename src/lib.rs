pub mod auth;
pub mod config;
pub mod db;
pub mod urls;
pub mod utils;

pub trait Validate {
    fn validate(&self) -> bool;
}
