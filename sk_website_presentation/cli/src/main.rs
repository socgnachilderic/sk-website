use chrono::NaiveDate;
use sk_website_application::commands::admin_create_user::AdminCreateUser;
use sk_website_application::commons::BaseHandler;

use crate::dependency_injection::InjectionContainer;

mod dependency_injection;

#[tokio::main]
async fn main() {
    let injection_container = InjectionContainer::new().await;
    let command = AdminCreateUser::default()
        .with_first_name("Childéric")
        .with_last_name("SOCGNA")
        .with_email("socgnachilderic@proton.me")
        .with_phonenumber("+237 693 80 49 24")
        .with_jobs("Fullstack Developer")
        .with_localization("Cameroon", "Yaoundé")
        .with_birthdate(&NaiveDate::from_ymd_opt(2000, 01, 25).unwrap());

    let response = injection_container
        .admin_create_user_handler
        .execute(command)
        .await;

    dbg!(response);
}
