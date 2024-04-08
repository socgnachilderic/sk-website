use sk_website_application::commands::CreateUser;
use sk_website_application::commons::BaseHandler;
use sk_website_infrastructure::parser::get_configs;

use crate::dependency_injection::InjectionContainer;

mod dependency_injection;

#[tokio::main]
async fn main() {
    let injection_container = InjectionContainer::new().await;
    let sk_configs = get_configs();

    for user_config in sk_configs.users {
        let is_superuser = user_config.is_superuser.unwrap_or_default();
        let command: CreateUser = user_config.into();

        let user = if is_superuser {
            injection_container
                .create_superuser_handler
                .execute(command)
                .await
                .unwrap()
        } else {
            injection_container
                .create_user_handler
                .execute(command)
                .await
                .unwrap()
        };

        dbg!(user);
    }
}
