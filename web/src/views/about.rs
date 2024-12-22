//! 这个用来放置本blog项目的关于页面

use dioxus::prelude::*;

// const BLOG_CSS: Asset = asset!("/assets/about.css");

#[component]
pub fn About() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS}
        p { "随便写点占个位" }
    }
}
