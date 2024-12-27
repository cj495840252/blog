//! This crate contains all shared UI for the workspace.

mod hero;
use dioxus::prelude::*;
pub use hero::Hero;

mod navbar;
pub use navbar::Navbar;

mod echo;
pub use echo::Echo;

mod web_navbar;
pub use web_navbar::WebNavbar;

mod test_component;
pub use test_component::Test;

pub mod home_body;
pub use home_body::HomeBody;

pub mod markdown;
pub use markdown::Markdown;

pub mod article_home;
pub use article_home::ArticleHomeLayout;

pub mod article_detail;
pub use article_detail::ArticleDetaileLayout;

pub mod menu_sidebar;
pub use menu_sidebar::MenuSidebar;

pub mod articles;
pub use articles::Articles;
pub const TAILWIND_CSS: Asset = asset!("./assets/css/tailwind.css");
