use dioxus::prelude::*;
use tracing::{info, Level};
mod router;
mod views;

use router::Route;

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::init(Level::INFO).unwrap();
    info!("Starting the web app");
    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // Connect to dioxus' logging infrastructure

    use axum::routing::get;
    dioxus::logger::init(Level::INFO).unwrap();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();
    info!("Starting the web app backend at: {}", socket_addr);

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new().unwrap(), App)
        .route(
            "/api",
            get(|| async {
                info!("access /api");
                "Hello, World!"
            }),
        )
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    // And launch it!
    axum::serve(listener, router).await.unwrap();
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {rel:"icon", href: "/assets/favicon.ico" ,r#type: "image/x-icon"}
        Router::<Route> {}
    }
}
