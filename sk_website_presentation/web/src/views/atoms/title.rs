use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub(crate) enum Level {
    H3,
    H4,
    H5,
}

#[derive(Props, PartialEq, Clone)]
pub(crate) struct TitleProps {
    #[props(default = Level::H3)]
    pub level: Level,
    pub children: Element,
}

#[component]
pub(crate) fn Title(props: TitleProps) -> Element {
    let TitleProps { level, children } = props;
    let line = rsx! { span { class: "block w-7 h-0.5 bg-red-500 !mt-1 !mb-12" } };

    match level {
        Level::H3 => rsx! {
            h3 { class: "font-normal text-3xl", {children} },
            {line}
        },
        Level::H4 => rsx! {
            h4 { class: "font-normal text-3xl", {children} },
            {line}
        },
        Level::H5 => rsx! {
            h5 { class: "text-xl font-semibold", {children} },
        },
    }
}
