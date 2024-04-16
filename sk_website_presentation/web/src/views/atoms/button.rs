use dioxus::prelude::*;
// use dioxus::prelude::{Link as InnerLink, LinkProps as InnerLinkProps};

#[derive(Props, PartialEq, Clone)]
pub(crate) struct ButtonProps {
    #[props(extends = button)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

pub(crate) struct Button;

impl Button {
    #[component]
    pub(crate) fn Button(props: ButtonProps) -> Element {
        rsx! {
            button { ..props.attributes, class: Self::class(), {props.children} }
        }
    }

    // #[component]
    // pub(crate) fn Link(props: InnerLinkProps) -> Element {
    //     rsx! {
    //         InnerLink {
    //             ..props,
    //             class: Self::class(),
    //         }
    //     }
    // }

    fn class() -> Option<String> {
        Some(String::from("bg-red-500 hover:bg-red-700 inline-flex items-center justify-center whitespace-nowrap text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 text-lg rounded-full text-white h-10 px-4 py-2"))
    }
}
