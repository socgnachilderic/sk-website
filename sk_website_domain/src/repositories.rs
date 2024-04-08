use async_trait::async_trait;

use crate::entities::User;
use crate::value_objects::Email;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> User;

    async fn find_superuser(&self) -> Option<User>;
    async fn find_user_by_email(&self, email: &Email) -> Option<User>;
}
