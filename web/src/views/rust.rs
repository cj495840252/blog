use dioxus::prelude::*;
use ui::layout::ArticleHomeLayout;

// const BLOG_CSS: Asset = asset!("/assets/about.css");

#[component]
pub fn Rust() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS}
        ArticleHomeLayout {  }
    }
}
