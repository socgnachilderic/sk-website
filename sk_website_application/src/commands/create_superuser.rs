use std::sync::Arc;

use async_trait::async_trait;

use sk_website_domain::entities::User;
use sk_website_domain::repositories::UserRepository;

use crate::commons::BaseHandler;
use crate::dto::UserDTO;

use super::create_user::CreateUser;

pub struct CreateSuperuserHandler {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl CreateSuperuserHandler {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl BaseHandler for CreateSuperuserHandler {
    type Action = CreateUser;
    type Response = Result<UserDTO, String>;

    async fn execute(&self, action: Self::Action) -> Self::Response {
        let is_superuser_already_exist = self.user_repository.find_superuser().await;

        if is_superuser_already_exist.is_some() {
            return Err(String::from("Superuser Already Exist."));
        }

        let email_already_exist = self.user_repository.find_user_by_email(&action.email).await;

        if email_already_exist.is_some() {
            return Err(String::from("User Already Exist"));
        }

        let mut user: User = action.into();
        user.upgrate_to_superuser();

        let user = self.user_repository.save(user).await;

        Ok(UserDTO::from(user))
    }
}
