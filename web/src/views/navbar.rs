use crate::router::Route;
use dioxus::prelude::*;
/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        ui::WebNavbar{}
        Outlet::<Route> {}
    }
}
