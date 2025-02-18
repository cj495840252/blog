use dioxus::prelude::*;
use ui::model::ArticleCard;
use ui::ArticleHomeLayout;

#[component]
pub fn BigData() -> Element {
    let articles = vec![
        ArticleCard::new(
            "Java",
            "Java".to_string(),
            "description 1".to_string(),
            "Java-logo.png".to_string(),
        ),
        ArticleCard::new(
            "Flink",
            "Flink".to_string(),
            "description 2".to_string(),
            "flink-logo.jpg".to_string(),
        ),
        ArticleCard::new(
            "Kafka",
            "Kafka".to_string(),
            "description 3".to_string(),
            "Kafka_logo.jpg".to_string(),
        ),
    ];
    rsx! {
        ArticleHomeLayout { articles }
    }
}
