use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sk_website_domain::entities::{User, UserBuilder};
use sk_website_domain::repositories::UserRepository;
use sk_website_domain::value_objects::{
    Email, Localization, PhoneNumber, UserBirthdate, UserJobs, UserNames,
};

use crate::commons::BaseHandler;
use crate::dto::UserDTO;

pub struct CreateUserHandler {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl CreateUserHandler {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl BaseHandler for CreateUserHandler {
    type Action = CreateUser;
    type Response = Result<UserDTO, String>;

    async fn execute(&self, action: Self::Action) -> Self::Response {
        match self.user_repository.find_user_by_email(&action.email).await {
            Some(_) => Err(String::from("User Already Exist.")),
            None => {
                let user = self.user_repository.save(action.into()).await;
                Ok(UserDTO::from(user))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CreateUser {
    pub name: UserNames,
    pub jobs: UserJobs,
    pub birthdate: UserBirthdate,
    pub email: Email,
    pub phone_number: PhoneNumber,
    pub localization: Localization,
}

impl From<CreateUser> for User {
    fn from(val: CreateUser) -> Self {
        UserBuilder::default()
            .with_name(&val.name)
            .with_email(&val.email)
            .with_phone_number(&val.phone_number)
            .with_birthdate(&val.birthdate)
            .with_localizationn(&val.localization)
            .with_jobs(&val.jobs)
            .build()
    }
}
