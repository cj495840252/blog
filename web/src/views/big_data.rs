use dioxus::prelude::*;
use ui::ArticleHomeLayout;

// const BLOG_CSS: Asset = asset!("/assets/about.css");

#[component]
pub fn BigData() -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS}

        ArticleHomeLayout {  }
        // div {
        //     class: "flex",
        //     height: "100rem",
        //     div {
        //         class: "w-1/4 h-screen bg-gray-200 sticky top-0 p-4" ,
        //         MenuSidebar {  }
        //     }
        //     div {
        //         class: "w-3/4 bg-white p-4" ,

        //     }
        // }
    }
}
