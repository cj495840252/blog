use dioxus::prelude::*;
use ui::{article_home::ArticleHomeLayout, model::ArticleCard};
#[component]
pub fn Rust() -> Element {
    let articles = vec![
        ArticleCard::new(
            "Rust基础",
            "rust 基础".to_string(),
            "跟着哔哩哔哩杨旭老师学rust".to_string(),
            "kafka_logo.jpg".to_string(),
        ),
        ArticleCard::new(
            "Rust基础",
            "数据待填充".to_string(),
            "description 2".to_string(),
            "not_set.jpeg".to_string(),
        ),
        ArticleCard::new(
            "Rust基础",
            "test2".to_string(),
            "description 3".to_string(),
            "not_set.jpeg".to_string(),
        ),
    ];
    rsx! {
        ArticleHomeLayout { articles }
    }
}
