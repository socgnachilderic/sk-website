use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sk_website_domain::value_objects::{
    Email, Localization, PhoneNumber, UserBirthdate, UserId, UserJobs, UserNames,
};
use sqlx::prelude::FromRow;
use sqlx::types::Uuid;
use sqlx::{Pool, Postgres};

use sk_website_domain::entities::{User, UserBuilder};
use sk_website_domain::repositories::UserRepository;

pub struct SqlxUserRepository(pub Pool<Postgres>);

#[async_trait]
impl UserRepository for SqlxUserRepository {
    async fn save(&self, user: User) -> User {
        let name = user.name();
        let localization = user.localization();
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
                    city,
                    is_superuser
                )
                VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )
                RETURNING *
            "#,
            user.user_id().value(),
            name.first_name,
            name.last_name,
            user.email().to_string(),
            user.phone_number().to_string(),
            user.jobs().to_string(),
            user.birthdate().value(),
            localization.country,
            localization.city,
            user.is_superuser()
        )
        .fetch_one(&self.0)
        .await;

        query_result.unwrap().into()
    }

    async fn find_superuser(&self) -> Option<User> {
        sqlx::query_as!(UserModel, r#"SELECT * FROM users WHERE is_superuser=true"#)
            .fetch_one(&self.0)
            .await
            .ok()
            .map(|user_model| user_model.into())
    }

    async fn find_user_by_email(&self, email: &Email) -> Option<User> {
        sqlx::query_as!(
            UserModel,
            r#"SELECT * FROM users WHERE email=$1"#,
            email.to_string()
        )
        .fetch_one(&self.0)
        .await
        .ok()
        .map(|user_model| user_model.into())
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub jobs: String,
    pub birthdate: NaiveDate,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub country: String,
    pub city: String,
    pub is_superuser: bool,
}

impl From<UserModel> for User {
    fn from(val: UserModel) -> Self {
        let email = Email::new(&val.email.unwrap());
        let phone_number = val.phone_number.unwrap().parse::<PhoneNumber>().unwrap();

        UserBuilder::default()
            .with_user_id(&UserId::new(val.user_id))
            .with_name(&UserNames::new(&val.first_name, &val.last_name))
            .with_email(&email)
            .with_phone_number(&phone_number)
            .with_birthdate(&UserBirthdate::new(val.birthdate))
            .with_localizationn(&Localization::new(&val.country, &val.city))
            .with_jobs(&UserJobs::new(&val.jobs))
            .with_is_superuser(val.is_superuser)
            .build()
    }
}
