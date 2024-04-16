use dioxus::prelude::*;

use crate::views::atoms::{Level, Title};

#[component]
pub(crate) fn HomeResume() -> Element {
    rsx! {
        section { id: "resume", class: "py-20",

            div { class: "container",

                h2 { class: "text-4xl font-semibold mb-12",

                    span { class: "text-red-500", "My" }
                    " Resume"
                }

                div { class: "grid grid-cols-3 gap-12",
                    ResumeCard { title: "Expertise",
                        ResumeTimeline {
                            during: "2022 - Present",
                            job: "UX Developer",
                            description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nostrum recusandae, cupiditate ullam dolor ratione repellendus.aliquid repudiandae saepe!."
                        }
                        ResumeTimeline {
                            during: "2021 - 2022",
                            job: "Front-end Developer",
                            description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nostrum recusandae, cupiditate ullam dolor ratione repellendus.aliquid repudiandae saepe!."
                        }
                        ResumeTimeline {
                            during: "2020 - 2021",
                            job: "UX Designer",
                            description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nostrum recusandae, cupiditate ullam dolor ratione repellendus.aliquid repudiandae saepe!."
                        }
                    }
                    ResumeCard { title: "Education",
                        ResumeTimeline {
                            during: "2022 - Present",
                            job: "UX Developer",
                            description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nostrum recusandae, cupiditate ullam dolor ratione repellendus.aliquid repudiandae saepe!."
                        }
                        ResumeTimeline {
                            during: "2021 - 2022",
                            job: "Front-end Developer",
                            description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nostrum recusandae, cupiditate ullam dolor ratione repellendus.aliquid repudiandae saepe!."
                        }
                        ResumeTimeline {
                            during: "2020 - 2021",
                            job: "UX Designer",
                            description: "Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nostrum recusandae, cupiditate ullam dolor ratione repellendus.aliquid repudiandae saepe!."
                        }
                    }
                    ResumeCard { title: "Skills", "HTML5 & CSS3" }
                }
            }
        }
    }
}

#[component]
fn ResumeCard(title: String, children: Element) -> Element {
    rsx! {
        div { class: "border border-gray-300 space-y-4 p-4 text-gray-600",
            Title { {title} }
            {children}
        }
    }
}

#[component]
fn ResumeTimeline(during: String, job: String, description: String) -> Element {
    rsx! {
        Title { level: Level::H4, class: "text-red-500", {during} }
        h6 { {job} }
        p { {description} }
    }
}
