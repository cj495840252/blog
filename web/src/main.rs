use dioxus::prelude::*;
use tracing::{info, Level};
mod router;
mod views;

use router::Route;

fn main() {
    dioxus::logger::init(Level::INFO).unwrap();
    info!("App started: http://localhost:8080");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {rel:"icon", href: "/assets/favicon.ico" ,r#type: "image/x-icon"}
        Router::<Route> {}
    }
}
