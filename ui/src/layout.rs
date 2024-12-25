use dioxus::prelude::*;

use crate::{Articles, Markdown, MenuSidebar, TAILWIND_CSS};

#[component]
pub fn ArticleHomeLayout() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex",
            div {
                class: "w-1/4 h-screen bg-gray-200 sticky top-0 p-4" ,
                MenuSidebar {  }
            }
            div {
                class: "w-3/4 bg-white p-4" ,
                style { height: "100rem" }
                Articles{}
            }
        }
    }
}

#[component]
pub fn ArticleDetaileLayout() -> Element {
    let class = use_signal(|| "content".into());
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
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
