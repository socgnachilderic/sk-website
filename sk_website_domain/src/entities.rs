use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Default, Validate)]
pub struct User {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub last_name: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub jobs: String,
    pub birthdate: NaiveDate,
    #[validate(email)]
    pub email: String,
    pub phone_number: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub country: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub city: String,
}

impl User {
    pub fn new() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            ..Default::default()
        }
    }

    pub fn with_first_name(mut self, first_name: &str) -> Self {
        self.first_name = String::from(first_name);
        self
    }

    pub fn with_last_name(mut self, last_name: &str) -> Self {
        self.last_name = String::from(last_name);
        self
    }

    pub fn with_jobs(mut self, jobs: &str) -> Self {
        self.jobs = String::from(jobs);
        self
    }

    pub fn with_birthdate(mut self, birthdate: &NaiveDate) -> Self {
        self.birthdate = birthdate.to_owned();
        self
    }

    pub fn with_email(mut self, email: &str) -> Self {
        self.email = String::from(email);
        self
    }

    pub fn with_phonenumber(mut self, phone_number: &str) -> Self {
        self.phone_number = String::from(phone_number);
        self
    }

    pub fn with_localization(mut self, country: &str, city: &str) -> Self {
        self.country = String::from(country);
        self.city = String::from(city);
        self
    }
}
