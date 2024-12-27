// use crate::Route;
use dioxus::prelude::*;
use ui::article_detail::ArticleDetaileLayout;

// const BLOG_CSS: Asset = asset!("/assets/blog.css");

#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        // document::Link { rel: "stylesheet", href: BLOG_CSS}
        div {
            id: "blog",
            ArticleDetaileLayout {  }
            // Navigation links
            // Link {
            //     to: Route::Blog { id: id - 1 },
            //     "Previous"
            // }
            // span { " <---> " }
            // Link {
            //     to: Route::Blog { id: id + 1 },
            //     "Next"
            // }
        }
    }
}
