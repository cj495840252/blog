use super::TAILWIND_CSS;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/css/navbar.css");
#[component]
pub fn WebNavbar() -> Element {
    rsx! {

    // 所有的页面都具有这个导航栏，所以在这里添加一次TAILWIND_CSS
    document::Link { rel: "stylesheet", href:  TAILWIND_CSS}
    document::Link { rel: "stylesheet", href:  NAVBAR_CSS}
    // w-1/4 h-screen bg-gray-200 sticky top-0 p-4
    nav {
        class: "border-bottom bg-white border-gray-200 dark:bg-gray-900",
        div {
            class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",
            a {
                // 跳转到当前网站的首页
                href: "/",
                class: "flex items-center space-x-3 rtl:space-x-reverse",
                img {
                    src: "/assets/avatar.png",
                    alt: "Default avatar",
                    class: "w-10 h-10 rounded",
                }
                span { class: "font-mono self-center text-2xl font-semibold whitespace-nowrap dark:text-white",
                    "Zack"
                }
            }
            div { class: "flex md:order-2",
                button {
                    "data-collapse-toggle": "navbar-search",
                    r#type: "button",
                    "aria-controls": "navbar-search",
                    "aria-expanded": "false",
                    class: "md:hidden text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-700 rounded-lg text-sm p-2.5 me-1",
                    svg {
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        "viewBox": "0 0 20 20",
                        fill: "none",
                        class: "w-5 h-5",
                        path {
                            "stroke-width": "2",
                            "stroke-linejoin": "round",
                            d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                            stroke: "currentColor",
                            "stroke-linecap": "round",
                        }
                    }
                    span { class: "sr-only", "Search" }
                }
                div { class: "relative hidden md:block",
                    div { class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                        svg {
                            fill: "none",
                            "aria-hidden": "true",
                            "viewBox": "0 0 20 20",
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "w-4 h-4 text-gray-500 dark:text-gray-400",
                            path {
                                "stroke-width": "2",
                                stroke: "currentColor",
                                "stroke-linecap": "round",
                                d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                                "stroke-linejoin": "round",
                            }
                        }
                        span { class: "sr-only", "Search icon" }
                    }
                    input {
                        r#type: "text",
                        placeholder: "Search...",
                        class: "block w-full p-2 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        id: "search-navbar",
                    }
                }
                button {
                    "data-collapse-toggle": "navbar-search",
                    r#type: "button",
                    "aria-expanded": "false",
                    "aria-controls": "navbar-search",
                    class: "inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600",
                    span { class: "sr-only", "Open main menu" }
                    svg {
                        "aria-hidden": "true",
                        "viewBox": "0 0 17 14",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        class: "w-5 h-5",
                        path {
                            stroke: "currentColor",
                            d: "M1 1h15M1 7h15M1 13h15",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            "stroke-width": "2",
                        }
                    }
                }
            }
            div {
                class: "items-center justify-between hidden w-full md:flex md:w-auto md:order-1",
                id: "navbar-search",
                div { class: "relative mt-3 md:hidden",
                    div { class: "absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none",
                        svg {
                            "aria-hidden": "true",
                            xmlns: "http://www.w3.org/2000/svg",
                            "viewBox": "0 0 20 20",
                            fill: "none",
                            class: "w-4 h-4 text-gray-500 dark:text-gray-400",
                            path {
                                stroke: "currentColor",
                                d: "m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                "stroke-width": "2",
                            }
                        }
                    }
                    input {
                        r#type: "text",
                        placeholder: "Search...",
                        class: "block w-full p-2 ps-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                        id: "search-navbar",
                    }
                }
                ul { class: "flex flex-col p-4 md:p-0 mt-4 font-medium border border-gray-100 rounded-lg bg-gray-50 md:space-x-8 rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700",
                    li {
                        a {
                            "aria-current": "page",
                            href: "/",
                            class: "block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 md:dark:text-blue-500",
                            "Home"
                        }
                    }
                    li {
                        a {
                            href: "/rust",
                            class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 md:dark:hover:text-blue-500 dark:text-white dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                            "Rust"
                        }
                    }
                    li {
                        a {
                            href: "/big_data",
                            class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                            "Big Data"
                        }
                    }
                    li {
                        a {
                            href: "/about",
                            class: "block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent dark:border-gray-700",
                            "About"
                        }
                    }
                }
            }
        }
    }

    }
}
