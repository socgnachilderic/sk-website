use async_trait::async_trait;

use crate::entities::User;

#[async_trait]
pub trait UserRepository {
    async fn save(&self, user: User) -> User;
    
    async fn find_current_user(&self) -> User;
}
