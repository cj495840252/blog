// use crate::Route;
use dioxus::prelude::*;
use ui::article_detail::ArticleDetaileLayout;

// const BLOG_CSS: Asset = asset!("/assets/blog.css");

#[component]
pub fn Blog(title: String) -> Element {
    rsx! {
        div {
            id: "blog",
            ArticleDetaileLayout { title }
        }
    }
}
