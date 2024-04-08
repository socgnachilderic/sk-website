use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn value(&self) -> &Uuid {
        &self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let user_id = Uuid::from_str(s.trim())?;

        Ok(Self(user_id))
    }
}
