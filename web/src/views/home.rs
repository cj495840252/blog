use dioxus::prelude::*;
use tracing::info;
use ui::HomeBody;
// use ui::{Echo, Hero};
const HOME_CSS: Asset = asset!("/assets/home.css");
#[component]
pub fn Home() -> Element {
    info!("Home page accessed");
    rsx! {

        document::Link { rel: "stylesheet", href: HOME_CSS }
        div {
            main {
                class: "home",
                div {
                    HomeBody {  }
                }
            }
         }
    }
}
