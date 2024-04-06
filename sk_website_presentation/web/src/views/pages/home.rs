use dioxus::prelude::*;

use crate::models::{User, USER};
use crate::views::organisms::{HomeAbout, HomeHero, HomeNavbar, HomeResume};

#[component]
pub(crate) fn Home() -> Element {
    let user_resource = use_server_future(get_user_server_data)?;
    
    if let Some(Ok(user)) = user_resource.value().read().as_ref() {
        *USER.write() = user.clone();

        rsx! {
            HomeHero { },
            HomeNavbar { },
            HomeAbout { },
            HomeResume { },
        }
    } else {
        rsx! { div { "Loading dogs..." } }
    }

}

#[server(GetUserServerData)]
async fn get_user_server_data() -> Result<User, ServerFnError> {
    use crate::dependency_injection::InjectionContainer;
    use sk_website_application::commons::BaseHandler;
    
    let injection_container = InjectionContainer::new().await;
    let user = injection_container.user_get_current_profile.execute(()).await;

    Ok(User {
        user_id: user.user_id,
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
        jobs: user.jobs,
        birthdate: user.birthdate.to_string(),
        phone_number: user.phone_number.to_string(),
        country: user.country.to_string(),
        city: user.city.to_string(),
    })
}
