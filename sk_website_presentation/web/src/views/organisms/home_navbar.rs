use dioxus::prelude::*;

use crate::views::Route;

#[component]
pub(crate) fn HomeNavbar() -> Element {
    const AVATAR: manganis::ImageAsset = manganis::mg!(image("./public/img/avatar.jpg"));

    rsx! {
        nav {
            class: "bg-white shadow sticky top-0 z-[1020] flex items-center px-4 py-3 flex-wrap text-gray-700",

            div {
                class: "container flex items-center",

                ul {
                    class: "mr-auto flex [&>*:first-child>a]:pl-0",

                    NavLink::A { href: "#home", "Home", active_class: NavLink::active_class() },
                    NavLink::A { href: "#about", "About" },
                    NavLink::A { href: "#resume", "Resume" },
                }

                div {
                    img {
                        class: "absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 border-white w-44 h-44 rounded-full",

                        border_width: "10px",
                        box_shadow: "0 5px 1px rgba(128, 128, 128, 0.13)",
                        src: AVATAR
                    }
                }

                ul {
                    class: "ml-auto flex [&>*:last-child>a]:pr-0",

                    NavLink::Link { to: Route::Blog { id: 1 }, "Portfolio" }
                    NavLink::Link { to: Route::Blog { id: 1 }, "Blog" }
                    NavLink::Link { to: Route::Blog { id: 1 }, "Contact" }
                }
            }
        }
    }
}

struct NavLink;

#[derive(Props, PartialEq, Clone)]
struct NavLinkAProps {
    #[props(extends = a)]
    atrributes: Vec<Attribute>,
    children: Element,
    active_class: Option<String>,
}

impl NavLink {
    #[component]
    fn Link(props: LinkProps) -> Element {
        rsx! {
            li { Link { ..props, class: Self::class(), active_class: Self::active_class() } }
        }
    }

    #[component]
    fn A(props: NavLinkAProps) -> Element {
        rsx! {
            li { a {
                ..props.atrributes,
                class: Self::class().unwrap_or_default(),
                class: props.active_class.unwrap_or_default(),
                {props.children}
            } }
        }
    }

    fn class() -> Option<String> {
        Some(String::from("py-3 px-6 block text-lg font-bold"))
    }

    fn active_class() -> Option<String> {
        Some(String::from("text-red-500"))
    }
}
