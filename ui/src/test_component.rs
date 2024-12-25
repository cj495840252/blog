use dioxus::prelude::*;

use crate::{MenuSidebar, TAILWIND_CSS};

#[component]
pub fn Test() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        MenuSidebar{}
    }
}
