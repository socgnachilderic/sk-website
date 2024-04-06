use std::sync::Arc;

use sk_website_application::commands::admin_create_user::AdminCreateUserHandler;
use sk_website_infrastructure::database::{get_pool, sqlx_user_repository::SqlxUserRepository};

pub(crate) struct InjectionContainer {
    pub admin_create_user_handler: AdminCreateUserHandler,
}

impl InjectionContainer {
    pub async fn new() -> Self {
        let pool = get_pool().await;
        let user_repository = Arc::new(SqlxUserRepository(pool));

        Self {
            admin_create_user_handler: AdminCreateUserHandler::new(user_repository),
        }
    }
}
