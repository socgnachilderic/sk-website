use dioxus::prelude::*;

#[component]
pub(crate) fn HomeResume() -> Element {
    rsx! {
        section {
            id: "resume",
            class: "py-20",

            div {
                class: "container",

                h2 {
                    class: "text-4xl font-semibold mb-12",

                    span { class: "text-red-500", "My" },
                    " Resume"
                },

                div {
                    class: "grid grid-cols-3 gap-4",


                }
            }
        }
    }
}
