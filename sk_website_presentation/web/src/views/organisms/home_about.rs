use dioxus::prelude::*;

use crate::models::USER;
use crate::views::atoms::{Button, Title};
use crate::views::molecules::SocialIcons;

#[component]
pub(crate) fn HomeAbout() -> Element {
    let infos = use_memo::<Vec<(String, String)>>(move || {
        let user = USER();

        vec![
            ("Birthdate".to_string(), user.birthdate),
            ("Email".to_string(), user.email),
            ("Phone".to_string(), user.phone_number),
            ("Location".to_string(), format!("{}, {}", user.country, user.city)),
        ]
    });

    rsx! {
        div {
            id: "about",
            class: "grid grid-cols-3",

            AboutCard {
                title: "Who am I ?",
                h5 { class: "text-xl font-semibold", "A Web Designer / Developer Located In Our Lovely Earth" }
                p { "Lorem ipsum dolor sit amet, consectetur adipisicing elit.sit amet, Qui deserunt consequatur fugit repellendusillo voluptas?" }
                Button::Button { "Download My CV" }
            },
            AboutCard {
                title: "Personal Info",
                AboutPersonalInfo { infos: infos() }
                SocialIcons { class: "text-red-500" },
            },
            AboutCard {
                title: "My Expertise",
                ul {
                    li {
                        span { "Birthdate :" },
                        "09/13/1996"
                    }
                }
            },
        }
    }
}

#[component]
fn AboutCard(title: String, children: Element) -> Element {
    rsx! {
        div {
            class: "p-16 border border-gray-300 space-y-4",

            Title { {title} }
            {children}
        }
    }
}

#[component]
fn AboutPersonalInfo(infos: Vec<(String, String)>) -> Element {    
    rsx! {
        ul {
            class: "space-y-4",

            {infos.iter().map(|info| rsx! {
                li {
                    span { class: "font-semibold", "{info.0} : " },
                    "{info.1}"
                }
            })}
        }
    }
}
