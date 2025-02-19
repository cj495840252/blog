use dioxus::prelude::*;
use ui::model::ArticleCard;
use ui::ArticleHomeLayout;

#[component]
pub fn BigData() -> Element {
    let articles = vec![
        ArticleCard::new(
            "Java",
            "Java".to_string(),
            "很久之前学习java的笔记，放这里凑个数，以后有文章了就删除。".to_string(),
            "Java-logo.png".to_string(),
        ),
        ArticleCard::new(
            "Flink",
            "Flink".to_string(),
            "很久之前学习flink的笔记，1.14版本3年前了".to_string(),
            "flink-Logo.jpg".to_string(),
        ),
        ArticleCard::new(
            "Kafka",
            "Kafka".to_string(),
            "很久之前学习kafka的笔记，放这里先凑个数".to_string(),
            "kafka_logo.jpg".to_string(),
        ),
    ];
    rsx! {
        ArticleHomeLayout { articles }
    }
}
