use std::fs;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sk_website_application::commands::CreateUser;
use sk_website_domain::value_objects::{
    Email, Localization, PhoneNumber, UserBirthdate, UserJobs, UserNames,
};

pub fn get_configs() -> SkConfigFile {
    dotenv().ok();

    let website_config_path =
        std::env::var("WEBSITE_CONFIG_PATH").expect("WEBSITE_CONFIG_PATH must set");
    let yaml =
        fs::read_to_string(website_config_path).expect("Should have been able to read the file");

    serde_yaml::from_str::<SkConfigFile>(&yaml).unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkConfigFile {
    pub users: Vec<ConfigUser>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct ConfigUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub birthdate: String,
    pub country: String,
    pub city: String,
    pub jobs: String,
    pub is_superuser: Option<bool>,
}

impl Into<CreateUser> for ConfigUser {
    fn into(self) -> CreateUser {
        CreateUser {
            name: UserNames::new(&self.first_name, &self.last_name),
            email: Email::new(&self.email),
            phone_number: self.phone_number.parse::<PhoneNumber>().unwrap(),
            birthdate: self.birthdate.parse::<UserBirthdate>().unwrap(),
            localization: Localization::new(&self.country, &self.city),
            jobs: UserJobs::new(&self.jobs),
        }
    }
}
