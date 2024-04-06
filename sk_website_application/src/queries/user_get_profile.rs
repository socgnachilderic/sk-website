use std::sync::Arc;

use async_trait::async_trait;
use sk_website_domain::repositories::UserRepository;

use crate::{commons::BaseHandler, dto::UserDTO};

pub struct UserGetProfileHandler {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
}

impl UserGetProfileHandler {
    pub fn new(user_repository: Arc<dyn UserRepository + Send + Sync>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl BaseHandler for UserGetProfileHandler {
    type Action = ();
    type Response = UserDTO;

    async fn execute(&self, _: Self::Action) -> Self::Response {
        let user = self.user_repository.find_current_user().await;

        UserDTO::from(user)
    }
}
