use dioxus::prelude::*;

use crate::{Articles, MenuSidebar};

#[component]
pub fn ArticleHomeLayout() -> Element {
    rsx! {
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
