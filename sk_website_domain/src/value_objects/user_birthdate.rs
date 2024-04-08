use std::str::FromStr;

use chrono::{NaiveDate, ParseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBirthdate(NaiveDate);

impl UserBirthdate {
    pub fn new(birthdate: NaiveDate) -> Self {
        Self(birthdate)
    }

    pub fn value(&self) -> &NaiveDate {
        &self.0
    }
}

impl ToString for UserBirthdate {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for UserBirthdate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = s.parse::<NaiveDate>()?;

        Ok(UserBirthdate::new(date))
    }
}
