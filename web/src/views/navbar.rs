use crate::router::frontend_router::FrontendRouter;
use dioxus::prelude::*;
/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "z-10 sticky bg-white shadow-md",
            top:"0",
            ui::WebNavbar{}
        }
        Outlet::<FrontendRouter> {}
    }
}
