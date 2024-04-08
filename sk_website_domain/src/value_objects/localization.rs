use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, Clone, PartialEq, Eq)]
pub struct Localization {
    #[validate(length(min = 1, message = "must not be empty"))]
    pub country: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub city: String,
}

impl Localization {
    pub fn new(country: &str, city: &str) -> Self {
        Self {
            country: country.trim().to_string(),
            city: city.trim().to_string(),
        }
    }
}

impl Default for Localization {
    fn default() -> Self {
        Self {
            country: String::from("Cameroun"),
            city: String::from("Yaoundé"),
        }
    }
}
