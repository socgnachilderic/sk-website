use serde::{Deserialize, Serialize};
use sk_website_domain::entities::User;
use sk_website_domain::value_objects::{
    Email, Localization, PhoneNumber, UserBirthdate, UserId, UserJobs, UserNames,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub user_id: UserId,
    pub name: UserNames,
    pub jobs: UserJobs,
    pub birthdate: UserBirthdate,
    pub email: Email,
    pub phone_number: PhoneNumber,
    pub localization: Localization,
    pub is_superuser: bool,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            user_id: value.user_id().to_owned(),
            name: value.name().to_owned(),
            email: value.email().to_owned(),
            phone_number: value.phone_number().to_owned(),
            birthdate: value.birthdate().to_owned(),
            localization: value.localization().to_owned(),
            jobs: value.jobs().to_owned(),
            is_superuser: value.is_superuser(),
        }
    }
}
