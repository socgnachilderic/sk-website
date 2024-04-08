use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::value_objects::{
    Email, Localization, PhoneNumber, UserBirthdate, UserId, UserJobs, UserNames,
};

#[derive(Debug, Serialize, Deserialize, Default, Validate, Eq)]
pub struct User {
    user_id: UserId,
    name: UserNames,
    jobs: UserJobs,
    birthdate: UserBirthdate,
    email: Email,
    phone_number: PhoneNumber,
    localization: Localization,
    is_superuser: bool,
}

impl User {
    pub fn upgrate_to_superuser(&mut self) {
        self.is_superuser = true
    }

    pub fn validate(&self) -> Result<(), String> {
        let fields: Vec<Box<dyn Validate>> = vec![
            Box::new(&self.name),
            Box::new(&self.jobs),
            Box::new(&self.email),
            Box::new(&self.localization),
        ];

        let errors = fields
            .iter()
            .filter_map(|field| field.validate().err())
            .fold(ValidationErrors::new(), |acc, val| {
                let mut field_errors = acc.field_errors();
                field_errors.extend(val.field_errors());
                acc
            });

        if !errors.is_empty() {
            return Err(errors.to_string());
        }

        Ok(())
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

impl User {
    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn name(&self) -> &UserNames {
        &self.name
    }

    pub fn jobs(&self) -> &UserJobs {
        &self.jobs
    }

    pub fn birthdate(&self) -> &UserBirthdate {
        &self.birthdate
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn phone_number(&self) -> &PhoneNumber {
        &self.phone_number
    }

    pub fn localization(&self) -> &Localization {
        &self.localization
    }

    pub fn is_superuser(&self) -> bool {
        self.is_superuser
    }
}

#[derive(Default)]
pub struct UserBuilder {
    user_id: Option<UserId>,
    name: UserNames,
    jobs: UserJobs,
    email: Email,
    phone_number: PhoneNumber,
    birthdate: UserBirthdate,
    localization: Localization,
    is_superuser: Option<bool>,
}

impl UserBuilder {
    pub fn with_user_id(mut self, user_id: &UserId) -> Self {
        self.user_id = Some(user_id.to_owned());
        self
    }

    pub fn with_name(mut self, name: &UserNames) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn with_email(mut self, email: &Email) -> Self {
        self.email = email.to_owned();
        self
    }

    pub fn with_phone_number(mut self, phone_number: &PhoneNumber) -> Self {
        self.phone_number = phone_number.to_owned();
        self
    }

    pub fn with_birthdate(mut self, birthdate: &UserBirthdate) -> Self {
        self.birthdate = birthdate.to_owned();
        self
    }

    pub fn with_localizationn(mut self, localization: &Localization) -> Self {
        self.localization = localization.to_owned();
        self
    }

    pub fn with_jobs(mut self, jobs: &UserJobs) -> Self {
        self.jobs = jobs.to_owned();
        self
    }

    pub fn with_is_superuser(mut self, is_superuser: bool) -> Self {
        self.is_superuser = Some(is_superuser);
        self
    }

    pub fn build(self) -> User {
        User {
            name: self.name,
            email: self.email,
            phone_number: self.phone_number,
            jobs: self.jobs,
            birthdate: self.birthdate,
            localization: self.localization,
            user_id: self.user_id.unwrap_or_default(),
            is_superuser: self.is_superuser.unwrap_or_default(),
        }
    }
}
