use convert_case::{Case, Casing};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

pub static USER: GlobalSignal<User> = Signal::global(User::default);

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub jobs: String,
    pub birthdate: String,
    pub email: String,
    pub phone_number: String,
    pub country: String,
    pub city: String,
}

impl User {
    pub fn full_name(&self) -> String {
        format!(
            "{} {}",
            self.last_name.to_uppercase(),
            self.first_name.to_case(Case::Title)
        )
    }
}
