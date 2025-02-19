use dioxus::prelude::*;
use ui::{article_home::ArticleHomeLayout, model::ArticleCard};
#[component]
pub fn Rust() -> Element {
    let articles = vec![ArticleCard::new(
        "Rust基础",
        "rust 基础".to_string(),
        "跟着哔哩哔哩杨旭老师学rust".to_string(),
        "rust-log.webp".to_string(),
    )];
    rsx! {
        ArticleHomeLayout { articles }
    }
}
