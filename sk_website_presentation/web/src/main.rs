use dioxus::prelude::*;
use log::LevelFilter;

use views::Route;

mod models;
mod views;

#[cfg(feature = "server")]
mod dependency_injection;

const _TAILWIND_URL: &str = manganis::mg!(file("./public/tailwind.css"));

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    // let cfg = server_only!(dioxus::fullstack::Config::new().incremental(
    //     IncrementalRendererConfig::default().invalidate_after(std::time::Duration::from_secs(120)),
    // ));

    // LaunchBuilder::fullstack().with_cfg(cfg).launch(app);
    launch(app);
}

pub fn app() -> Element {
    rsx! { Router::<Route> {} }
}
