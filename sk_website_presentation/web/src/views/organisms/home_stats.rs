use dioxus::prelude::*;

use crate::views::organisms::Container;

#[component]
pub fn HomeStats() -> Element {
    rsx! {
        Container { class: "bg-gray-800",

            ul { class: "flex justify-between",
                StatsItem { title: "500", summary: "Hours Worked", class: "i-ion-alarm-outline" }
                StatsItem { title: "50K", summary: "Project Finished", class: "i-ion-layers-outline" }
                StatsItem { title: "200K", summary: "Happy Clients", class: "i-ion-happy-outline" }
                StatsItem { title: "2k", summary: "Coffee Drinked", class: "i-ion-heart-broken" }
            }
        }
    }
}

#[component]
fn StatsItem(title: String, summary: String, class: String) -> Element {
    rsx! {
        li { class: "flex text-gray-300 gap-4 items-center",
            i { class: "text-7xl text-white {class}" }
            div { class: "h-full w-[1px] bg-white" }
            div { class: "py-4",
                h3 { class: "text-4xl font-bold text-red-500", {title} }
                p { {summary} }
            }
        }
    }
}
