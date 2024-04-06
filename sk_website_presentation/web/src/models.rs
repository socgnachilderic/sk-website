use serde::{Deserialize, Serialize};
use dioxus::prelude::*;

pub static USER: GlobalSignal<User> = Signal::global(|| User::default());

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
        format!("{} {}", self.last_name, self.first_name)
    }
}
