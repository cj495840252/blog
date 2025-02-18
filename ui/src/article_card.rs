use dioxus::prelude::*;

use crate::model;

#[component]
pub fn ArticleCard(article: model::ArticleCard) -> Element {
    rsx! {
        div { class: "rounded overflow-hidden shadow-lg flex flex-col",
            div {
                class: "relative",
                a {
                    href: "/blog/{article.url}",
                    img {
                        src: "/assets/{article.image}",
                        alt: "Sunset in the mountains",
                        class: "w-full",
                    }
                    div { class: "hover:bg-transparent transition duration-300 absolute bottom-0 top-0 right-0 left-0 bg-gray-900 opacity-25" }
                }
                // a { href: "#!",
                //     div { class: "text-xs absolute top-0 right-0 bg-indigo-600 px-4 py-2 text-white mt-3 mr-3 hover:bg-white hover:text-indigo-600 transition duration-500 ease-in-out",
                //         "Cooking"
                //     }
                // }
            }
            div { class: "px-6 py-4 mb-auto",
                a {
                    href: "/blog/{article.url}",
                    class: "font-medium text-lg inline-block hover:text-indigo-600 transition duration-500 ease-in-out inline-block mb-2",
                    "{article.title}"
                }
                p { class: "text-gray-500 text-sm",
                    "{article.description}"
            }
            }
            div { class: "px-6 py-3 flex flex-row items-center justify-between bg-gray-100",
                a {
                    href: "#",
                    span {
                        class: "py-1 text-xs font-regular text-gray-900 mr-1 flex flex-row items-center",
                        svg {
                            version: "1.1",
                            style: "enable-background:new 0 0 512 512;",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "13px",
                            y: "0px",
                            "xmlns:xlink": "http://www.w3.org/1999/xlink",
                            "xml:space": "preserve",
                            height: "13px",
                            x: "0px",
                            "viewBox": "0 0 512 512",
                            id: "Layer_1",
                            g {
                                g {
                                    path { d: "M256,0C114.837,0,0,114.837,0,256s114.837,256,256,256s256-114.837,256-256S397.163,0,256,0z M277.333,256 c0,11.797-9.536,21.333-21.333,21.333h-85.333c-11.797,0-21.333-9.536-21.333-21.333s9.536-21.333,21.333-21.333h64v-128 c0-11.797,9.536-21.333,21.333-21.333s21.333,9.536,21.333,21.333V256z" }
                                }
                            }
                        }
                        span { class: "ml-1", "1 month ago" }
                    }

                    }
                a {
                    href: "#",
                span {
                    class: "py-1 text-xs font-regular text-gray-900 mr-1 flex flex-row items-center",
                    svg {
                        "viewBox": "0 0 24 24",
                        stroke: "currentColor",
                        fill: "none",
                        class: "h-5",
                        path {
                            "stroke-linejoin": "round",
                            "stroke-linecap": "round",
                            d: "M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z",
                            "stroke-width": "2",
                        }
                    }
                    span { class: "ml-1", "{article.comments} Comments" }
                }
                    }

            }
        }

    }
}
