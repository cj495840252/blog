use dioxus::prelude::*;
use tracing::{info, Level};
mod router;
mod views;

use router::frontend_router::FrontendRouter;

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::logger::init(Level::INFO).unwrap();
    info!("Starting the web app");
    // let config = dioxus::config::Config::default();
    // LaunchBuilder::new().with_cfg(config).launch(App);

    dioxus::launch(App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    // Connect to dioxus' logging infrastructure
    use router::api_router::get_api_router;
    use tower_http::services::{ServeDir, ServeFile};
    dioxus::logger::init(Level::INFO).unwrap();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    // let addr = dioxus::cli_config::fullstack_address_or_localhost();
    let addr = dioxus_cli_config::fullstack_address_or_localhost();
    info!("Starting the web app backend at: {}", addr);

    let serve_dir =
        ServeDir::new("articles").not_found_service(ServeFile::new("assets/NotFound.html"));

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new().unwrap(), App)
        .nest("/api", get_api_router())
        .nest_service("/articles", serve_dir.clone())
        // .fallback_service(serve_dir)
        .into_make_service();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    // And launch it!
    axum::serve(listener, router).await.unwrap();
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {rel:"icon", href: "/assets/favicon.ico" ,r#type: "image/x-icon"}
        Router::<FrontendRouter> {}
    }
}
