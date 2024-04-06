use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};

use sk_website_domain::entities::User;
use sk_website_domain::repositories::UserRepository;

pub struct SqlxUserRepository(pub Pool<Postgres>);

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn save(&self, user: User) -> User {
        let query_result = sqlx::query_as!(
            UserModel,
            r#"
                INSERT INTO users (
                    user_id,
                    first_name,
                    last_name,
                    email,
                    phone_number,
                    jobs,
                    birthdate,
                    country,
                    city
                )
                VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )
                RETURNING *
            "#,
            user.user_id.clone(),
            user.first_name,
            user.last_name,
            user.email,
            user.phone_number,
            user.jobs,
            user.birthdate,
            user.country,
            user.city
        )
        .fetch_one(&self.0)
        .await;
        
        query_result.unwrap().into()
    }

    async fn find_current_user(&self) -> User {
        let query_result = sqlx::query_as!(
            UserModel,
            r#"SELECT * FROM users"#
        )
        .fetch_one(&self.0)
        .await;

        query_result.unwrap().into()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub jobs: String,
    pub birthdate: NaiveDate,
    pub email: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
}

impl Into<User> for UserModel {
    fn into(self) -> User {
        User {
            user_id: self.user_id,
            first_name: self.first_name,
            last_name: self.last_name,
            jobs: self.jobs,
            birthdate: self.birthdate,
            email: self.email,
            phone_number: self.phone_number,
            country: self.country,
            city: self.city,
        }
    }
}
