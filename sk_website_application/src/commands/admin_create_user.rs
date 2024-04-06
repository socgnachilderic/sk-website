use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::commons::BaseHandler;
use crate::dto::UserDTO;
use sk_website_domain::entities::User;
use sk_website_domain::repositories::UserRepository;

pub struct AdminCreateUserHandler {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl AdminCreateUserHandler {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl BaseHandler for AdminCreateUserHandler {
    type Action = AdminCreateUser;
    type Response = UserDTO;

    async fn execute(&self, action: Self::Action) -> Self::Response {
        let user = self.user_repository.save(action.into()).await;

        UserDTO::from(user)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AdminCreateUser {
    pub first_name: String,
    pub last_name: String,
    pub jobs: String,
    pub birthdate: NaiveDate,
    pub email: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
}

impl AdminCreateUser {
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

impl Into<User> for AdminCreateUser {
    fn into(self) -> User {
        let user = User::new()
            .with_first_name(&self.first_name)
            .with_last_name(&self.last_name)
            .with_email(&self.email)
            .with_phonenumber(&self.phone_number)
            .with_birthdate(&self.birthdate)
            .with_jobs(&self.jobs)
            .with_localization(&self.country, &self.city);

        user
    }
}
