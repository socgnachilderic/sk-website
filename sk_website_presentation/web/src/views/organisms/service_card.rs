use dioxus::prelude::*;

use crate::views::atoms::{Level, Title};

#[component]
pub fn ServiceCard(title: String, description: String, class: String) -> Element {
    rsx! {
        div { class: "space-y-4 w-full max-w-sm py-6 px-6 bg-white border border-gray-200 relative",
            div { class: "absolute top-0 -translate-y-1/2 bg-white px-1", i { class: "{class} text-5xl text-red-500" } }
            Title { level: Level::H4, {title} }
            p { class: "text-gray-600", {description} }
        }
    }
}
