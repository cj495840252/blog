use crate::views::{About, BigData, Blog, Home, Navbar, Rust};
use dioxus::prelude::*;
use ui::{file_upload::FileUpload, login::Login, Test};
// use web::model::ArticleCard;
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum FrontendRouter {

    #[route("/test")]
    Test{},

    #[layout(Navbar)]
    #[route("/")]
    Home {},

    #[route("/blog/:title")]
    Blog { title: String },

    #[route("/about")]
    About {},

    #[route("/rust")]
    Rust {},

    #[route("/big_data")]
    BigData { },

    #[route("/login")]
    Login,

    #[route("/article/upload")]
    FileUpload
}
