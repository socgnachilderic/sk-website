use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub(crate) struct ContainerProps {
    #[props(extends = div)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
    pub id: Option<String>,
    pub class: Option<String>,
}

#[component]
pub(crate) fn Container(props: ContainerProps) -> Element {
    let ContainerProps {
        id,
        class,
        children,
        attributes,
    } = props;

    rsx! {
        section {
            id,
            class: "py-20",
            class: if let Some(class) = class { class },
            ..attributes,

            div { class: "relative container", {children} }
        }
    }
}
