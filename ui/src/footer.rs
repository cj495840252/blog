use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "relative bg-blueGray-200 pt-8 pb-6",
            div { class: "container mx-auto px-4",
                div { class: "flex flex-wrap items-center md:justify-between justify-center",
                    div { class: "w-full md:w-4/12 px-4 mx-auto text-center",
                        div { class: "text-sm text-blueGray-500 font-semibold py-1",
                            "\n          Copyright © "
                            span { id: "get-current-year", "2021" }
                            "<a href=\"https://www.creative-tim.com/product/notus-js\" class=\"text-blueGray-500 hover:text-gray-800\" target=\"_blank\"> Notus JS by\n          "
                            a {
                                href: "https://www.creative-tim.com?ref=njs-profile",
                                class: "text-blueGray-500 hover:text-blueGray-800",
                                "Creative Tim"
                            }
                            ".\n        "
                        }
                    }
                }
            }
        }
    }
}
