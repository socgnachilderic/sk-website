use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, PartialEq, Eq)]
pub struct UserNames {
    #[validate(length(min = 1, message = "must not be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub last_name: String,
}

impl UserNames {
    pub fn new(first_name: &str, last_name: &str) -> Self {
        Self {
            first_name: first_name.trim().to_lowercase(),
            last_name: last_name.trim().to_lowercase(),
        }
    }
}
