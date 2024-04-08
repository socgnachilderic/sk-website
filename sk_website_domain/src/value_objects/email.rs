use std::str::FromStr;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, PartialEq, Eq)]
pub struct Email {
    #[validate(email)]
    pub value: String,
}

impl Email {
    pub fn new(email: &str) -> Self {
        Self {
            value: email.trim().to_lowercase(),
        }
    }
}

impl FromStr for Email {
    type Err = ();

    fn from_str(email: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: email.trim().to_lowercase(),
        })
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
