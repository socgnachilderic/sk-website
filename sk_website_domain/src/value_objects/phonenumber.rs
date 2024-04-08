use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PhoneNumber {
    dial_code: u16,
    number: String,
}

impl PhoneNumber {
    pub fn new(dial_code: u16, number: &str) -> Self {
        Self {
            dial_code,
            number: number.trim().replace(' ', ""),
        }
    }
}

impl ToString for PhoneNumber {
    fn to_string(&self) -> String {
        format!("{} {}", self.dial_code, self.number)
    }
}

impl FromStr for PhoneNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split_whitespace().collect::<Vec<_>>();

        if values.len() != 2 {
            return Err("phone number is invalid".to_string());
        }

        let dial_code = values[0].parse::<u16>().unwrap();
        let number = values[1].to_string();

        Ok(Self { dial_code, number })
    }
}
