use std::sync::Arc;

use sk_website_application::commands::{CreateSuperuserHandler, CreateUserHandler};
use sk_website_infrastructure::database::get_pool;
use sk_website_infrastructure::database::sqlx_user_repository::SqlxUserRepository;

pub(crate) struct InjectionContainer {
    pub create_superuser_handler: CreateSuperuserHandler,
    pub create_user_handler: CreateUserHandler,
}

impl InjectionContainer {
    pub async fn new() -> Self {
        let pool = get_pool().await;
        let user_repository = Arc::new(SqlxUserRepository(pool));

        Self {
            create_superuser_handler: CreateSuperuserHandler::new(user_repository.clone()),
            create_user_handler: CreateUserHandler::new(user_repository),
        }
    }
}
