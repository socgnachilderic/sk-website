use dioxus::prelude::*;

use crate::models::USER;
use crate::views::atoms::{Button, Level, Title};
use crate::views::molecules::SocialIcons;

#[component]
pub(crate) fn HomeAbout() -> Element {
    let infos = use_memo::<Vec<(String, String)>>(move || {
        let user = USER();

        vec![
            ("Birthdate".to_string(), user.birthdate),
            ("Email".to_string(), user.email),
            ("Phone".to_string(), user.phone_number),
            (
                "Location".to_string(),
                format!("{}, {}", user.country, user.city),
            ),
        ]
    });

    rsx! {
        div { id: "about", class: "grid grid-cols-3",

            AboutCard { title: "Who am I ?",
                h5 { class: "text-xl font-semibold",
                    "A Web Designer / Developer Located In Our Lovely Earth"
                }
                p { class: "text-gray-600",
                    "Lorem ipsum dolor sit amet, consectetur adipisicing elit.sit amet, Qui deserunt consequatur fugit repellendusillo voluptas?"
                }
                Button::Button { "Download My CV" }
            }
            AboutCard { title: "Personal Info",
                div { class: "space-y-10",
                    AboutPersonalInfo { infos: infos() }
                    SocialIcons { class: "!text-red-500" }
                }
            }
            AboutCard { title: "My Expertise",
                AboutExpertiseInfo {
                    ExpertiseItem {
                        title: "Software Development",
                        summary: "exercitat Repellendus, corrupt.",
                        class: "i-ion-cafe-outline"
                    }
                    ExpertiseItem {
                        title: "DevOps and Cloud",
                        summary: "Lorem ipsum dolor sit consectetur.",
                        class: "i-ion-ios-cloud-outline"
                    }
                    ExpertiseItem {
                        title: "Blockchain",
                        summary: "voluptate commodi illo voluptatib.",
                        class: "i-ion-social-bitcoin-outline"
                    }
                }
            }
        }
    }
}

#[component]
fn AboutCard(title: String, children: Element) -> Element {
    rsx! {
        div { class: "p-16 border border-gray-300 space-y-4",
            Title { {title} }
            {children}
        }
    }
}

#[component]
fn AboutPersonalInfo(infos: Vec<(String, String)>) -> Element {
    rsx! {
        ul { class: "space-y-4 text-gray-600",
            {infos.iter().map(|info| rsx! {
                li {
                    span { class: "font-semibold text-gray-700", "{info.0} : " },
                    "{info.1}"
                }
            })}
        }
    }
}

#[component]
fn AboutExpertiseInfo(children: Element) -> Element {
    rsx! {
        ul { {children} }
    }
}

#[component]
fn ExpertiseItem(title: String, summary: String, class: String) -> Element {
    rsx! {
        li { class: "flex gap-4",
            i { class: "{class} text-4xl text-red-500" }
            div { class: "flex-1",
                Title { level: Level::H5, class: "!font-normal", {title} }
                p { class: "text-gray-600", {summary} }
                hr { class: "my-4" }
            }
        }
    }
}
