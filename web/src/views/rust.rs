use dioxus::prelude::*;

// const BLOG_CSS: Asset = asset!("/assets/about.css");

#[component]
pub fn Rust() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS}
        p { "随便写点占个位" }
    }
}
