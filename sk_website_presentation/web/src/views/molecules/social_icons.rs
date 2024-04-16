use dioxus::prelude::*;

#[component]
pub fn SocialIcons(class: Option<String>) -> Element {
    rsx! {
        ul {
            class: "flex max-w-[180px] justify-between text-white text-[18px]",
            class: if let Some(class) = class { class },
            SocialIcon {
                href: "https://www.google.com",
                class: "i-ion-social-facebook",
            },
            SocialIcon {
                href: "https://twitter.com/c_childeric",
                class: "i-ion-social-twitter-outline",
            },
            SocialIcon {
                href: "https://www.linkedin.com/in/childeric-socgna-kouyem-2690b3159",
                class: "i-ion-social-linkedin",
            },
            SocialIcon {
                href: "https://github.com/socgnachilderic",
                class: "i-ion-social-github",
            },
            SocialIcon {
                href: "https://gitlab.com/socgnachilderic",
                class: "i-ion-logo-gitlab",
            },
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct SocialIconProps {
    #[props(into)]
    href: String,
    #[props(into)]
    class: String,
}

#[component]
fn SocialIcon(props: SocialIconProps) -> Element {
    rsx! {
        li {
            a {
                href: props.href,
                target: "_blank",
                i { class: " {props.class}" }
            }
        }
    }
}
