use dioxus::prelude::*;

use crate::{Markdown, MenuSidebar};

#[component]
pub fn ArticleDetaileLayout() -> Element {
    let class = use_signal(|| "content".into());
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
                        class :  class, content: include_str!("../../README.md")
                    }
                }
            }
        }
    }
}
