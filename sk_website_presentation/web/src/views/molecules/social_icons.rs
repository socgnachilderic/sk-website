use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{
    IoLogoFacebook, IoLogoGithub, IoLogoGitlab, IoLogoLinkedin, IoLogoTwitter,
};
use dioxus_free_icons::{Icon, IconShape};

#[component]
pub fn SocialIcons(class: Option<String>) -> Element {
    rsx! {
        ul {
            class: "flex max-w-[180px] justify-between text-white",
            class: if let Some(class) = class { class },
            SocialIcon {
                href: "https://www.google.com",
                icon: IoLogoFacebook,
            },
            SocialIcon {
                href: "https://www.google.com",
                icon: IoLogoTwitter,
            },
            SocialIcon {
                href: "https://www.google.com",
                icon: IoLogoLinkedin,
            },
            SocialIcon {
                href: "https://www.google.com",
                icon: IoLogoGithub,
            },
            SocialIcon {
                href: "https://www.google.com",
                icon: IoLogoGitlab,
            },
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct SocialIconProps<T: IconShape + Clone + PartialEq + 'static> {
    #[props(into)]
    href: String,
    icon: T,
    #[props(default = 18)]
    size: u16,
}

#[component]
fn SocialIcon<T: IconShape + Clone + PartialEq + 'static>(props: SocialIconProps<T>) -> Element {
    let size: u32 = props.size.into();

    rsx! {
        li {
            a {
                href: props.href,
                target: "_blank",
                Icon {
                    width: size,
                    height: size,
                    icon: props.icon
                }
            }
        }
    }
}
