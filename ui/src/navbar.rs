use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/css/navbar.css");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            class: "fixed",
            {children}
        }
    }
}
