use std::sync::Arc;

use sk_website_application::queries::user_get_profile::UserGetProfileHandler;
use sk_website_infrastructure::database::get_pool;
use sk_website_infrastructure::database::sqlx_user_repository::SqlxUserRepository;

pub(crate) struct InjectionContainer {
    pub user_get_current_profile: UserGetProfileHandler,
}

impl InjectionContainer {
    pub async fn new() -> Self {
        let pool = get_pool().await;
        let user_repository = Arc::new(SqlxUserRepository(pool));

        Self {
            user_get_current_profile: UserGetProfileHandler::new(user_repository),
        }
    }
}
