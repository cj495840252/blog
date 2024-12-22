use dioxus::prelude::*;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
mod router;
mod views;

use router::Route;

fn main() {
    let layer = Layer::default().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
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
