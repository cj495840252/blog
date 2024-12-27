use dioxus::prelude::*;
use ui::article_home::ArticleHomeLayout;
#[component]
pub fn Rust() -> Element {
    rsx! {
        ArticleHomeLayout {  }
    }
}
