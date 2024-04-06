use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sk_website_domain::entities::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub jobs: String,
    pub birthdate: NaiveDate,
    pub email: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id.to_string(),
            first_name: value.first_name,
            last_name: value.last_name,
            jobs: value.jobs,
            birthdate: value.birthdate,
            email: value.email,
            phone_number: value.phone_number,
            country: value.country,
            city: value.city,
        }
    }
}
