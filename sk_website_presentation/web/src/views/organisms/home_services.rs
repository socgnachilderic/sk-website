use dioxus::prelude::*;

use crate::views::organisms::{Container, ServiceCard};

#[component]
pub(crate) fn HomeServices() -> Element {
    rsx! {
        Container { id: "services",
            h2 { class: "text-4xl font-semibold mb-16",

                span { class: "text-red-500", "My" }
                " Services"
            }

            div { class: "grid grid-cols-3 gap-y-14",

                ServiceCard {
                    class: "i-ion-ios-briefcase-outline",
                    title: "Ullam",
                    description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ullam commodi provident, dolores reiciendis enim pariatur error optio, tempora ex, nihil nesciunt! In praesentium sunt commodi, unde ipsam ex veritatis laboriosam dolor asperiores suscipit blanditiis, dignissimos quos nesciunt nulla aperiam officia."
                }
                ServiceCard {
                    class: "i-ion-ios-briefcase-outline",
                    title: "Asperiores",
                    description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ullam commodi provident, dolores reiciendis enim pariatur error optio, tempora ex, nihil nesciunt! In praesentium sunt commodi, unde ipsam ex veritatis laboriosam dolor asperiores suscipit blanditiis, dignissimos quos nesciunt nulla aperiam officia."
                }
                ServiceCard {
                    class: "i-ion-ios-briefcase-outline",
                    title: "Tempora",
                    description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ullam commodi provident, dolores reiciendis enim pariatur error optio, tempora ex, nihil nesciunt! In praesentium sunt commodi, unde ipsam ex veritatis laboriosam dolor asperiores suscipit blanditiis, dignissimos quos nesciunt nulla aperiam officia."
                }
                ServiceCard {
                    class: "i-ion-ios-briefcase-outline",
                    title: "Provident",
                    description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ullam commodi provident, dolores reiciendis enim pariatur error optio, tempora ex, nihil nesciunt! In praesentium sunt commodi, unde ipsam ex veritatis laboriosam dolor asperiores suscipit blanditiis, dignissimos quos nesciunt nulla aperiam officia."
                }
                ServiceCard {
                    class: "i-ion-ios-briefcase-outline",
                    title: "Consectetur",
                    description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ullam commodi provident, dolores reiciendis enim pariatur error optio, tempora ex, nihil nesciunt! In praesentium sunt commodi, unde ipsam ex veritatis laboriosam dolor asperiores suscipit blanditiis, dignissimos quos nesciunt nulla aperiam officia."
                }
                ServiceCard {
                    class: "i-ion-ios-briefcase-outline",
                    title: "Veritatis",
                    description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Ullam commodi provident, dolores reiciendis enim pariatur error optio, tempora ex, nihil nesciunt! In praesentium sunt commodi, unde ipsam ex veritatis laboriosam dolor asperiores suscipit blanditiis, dignissimos quos nesciunt nulla aperiam officia."
                }
            }
        }
    }
}
