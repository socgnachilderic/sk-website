use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::IoPrint;
use dioxus_free_icons::Icon;

use crate::models::USER;
use crate::views::atoms::Button;
use crate::views::molecules::SocialIcons;

const BACKGROUND: manganis::ImageAsset = manganis::mg!(image("public/img/header.jpg"));

#[component]
pub fn HomeHero() -> Element {
    rsx! {
        header {
            class: "relative min-h-[600px] h-[80vh] text-white bg-cover bg-center bg-no-repeat bg-fixed",
            background_image: "linear-gradient(to top, rgba(0, 0, 0, 0.7), rgba(0, 0, 0, 0.7)), url({BACKGROUND})",

            div {
                class: "relative h-full container",

                SocialIcons { class: "py-4" },
                ul {
                    class: "w-full absolute left-0 -translate-y-1/2 space-y-4",
                    top: "55%",

                    h4 { class: "text-4xl font-extralight", "Hello, I am" }
                    h1 { class: "text-6xl font-bold", "{USER().full_name()}" },
                    h6 {
                        class: "font-semibold text-lg",
                        letter_spacing: "5px",
                        "{USER().jobs}"
                    },
                    Button::Button {
                        Icon {
                            width: 30,
                            height: 30,
                            icon: IoPrint,
                        },
                       "Print Resume"
                    }
                }
            }
        }
    }
}
