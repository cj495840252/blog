use crate::{Markdown, MenuSidebar};
use dioxus::prelude::*;

#[component]
pub fn ArticleDetaileLayout(title: String) -> Element {
    let class = use_signal(|| "content".into());
    let mut content = use_signal(|| "".to_string());
    let _ = use_resource(move || {
        let value = title.clone();
        async move {
            let response = reqwest::get(&format!("http://127.0.0.1:8080/articles/{}.md", value))
                .await
                .unwrap();
            let text = response.text().await.unwrap();
            content.set(text);
        }
    });

    rsx! {
        div {
            class: "flex",
            div {
                class: "w-1/4 h-screen bg-gray-200 sticky top-0 p-4" ,
                MenuSidebar {  }
            }
            div {
                class: "w-3/4 bg-white p-4" ,
                div {
                    class: "container is-fluid",
                    Markdown {
                        class :  class, content
                    }
                }
            }
        }
    }
}
