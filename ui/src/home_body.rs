use dioxus::prelude::*;

// use crate::TAILWIND_CSS;

const HOMEBODY_CSS: Asset = asset!("/assets/css/home_body.css");

#[component]
pub fn HomeBody() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOMEBODY_CSS }

        div {
            class: " relative overflow-hidden bg-white ",
            section {
                class: "bg-zinc-50 overflow-hidden ",
                div {
                    class: "max-w-screen-xl 2xl:max-w-screen-3xl px-8 md:px-12 mx-auto py-12 lg:py-24 space-y-24 h-svh flex flex-col justify-center test-t",
                    p {
                        class:"desc-font font-mono pointer-events-none fixed inset-x-0 top-0 sm:flex sm:justify-center sm:px-6 lg:px-8 z-10",
                        "Hello, Welcome my blog!" br {  }
                        "My name is Zack." br {  }
                        "I am a data engineer and a Rustaceans." br {  }
                        "I also love photography."
                     }
                    div {
                        class: "flex flex-col sm:flex-row mx-auto",
                        a { href: "#_",
                            img {
                                alt: "#_",
                                src: "https://images.unsplash.com/photo-1530035415911-95194de4ebcc?q=80&amp;w=2670&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
                                class: "rounded-xl rotate-6 hover:rotate-0 duration-500 hover:-translate-y-12 h-full w-full object-cover hover:scale-150 transform origin-bottom",
                            }
                        }
                        a { href: "#_",
                            img {
                                alt: "#_",
                                src: "https://images.unsplash.com/photo-1487180144351-b8472da7d491?q=80&amp;w=2672&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D ",
                                class: "rounded-xl -rotate-12 hover:rotate-0 duration-500 hover:-translate-y-12 h-full w-full object-cover hover:scale-150 transform origin-bottom",
                            }
                        }
                        a { href: "#_",
                            img {
                                src: "https://images.unsplash.com/photo-1586996292898-71f4036c4e07?q=80&amp;w=2670&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
                                alt: "#_",
                                class: "rounded-xl rotate-6 hover:rotate-0 duration-500 hover:-translate-y-12 h-full w-full object-cover hover:scale-150 transform origin-bottom",
                            }
                        }
                        a { href: "#_",
                            img {
                                src: "https://images.unsplash.com/photo-1522775417749-29284fb89f43?q=80&amp;w=2574&amp;auto=format&amp;fit=crop&amp;ixlib=rb-4.0.3&amp;ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
                                alt: "#_",
                                class: "rounded-xl -rotate-12 hover:rotate-0 duration-500 hover:-translate-y-12 h-full w-full object-cover hover:scale-150 transform origin-bottom",
                            }
                        }
                    }

                }
            }
            div {
                class: "pointer-events-none fixed inset-x-0 bottom-1 sm:flex sm:justify-center sm:px-6 sm:pb-5 lg:px-8",
                div {
                    class: "pointer-events-auto flex w-full max-w-md divide-x divide-neutral-200 rounded-lg bg-white shadow-lg ring-1 ring-black ring-opacity-5",
                    div { class: "flex w-0 flex-1 items-center p-4",
                        div { class: "w-full",
                            p { class: "text-sm font-medium text-neutral-900", "Tutorial" }
                            p { class: "mt-1 text-sm text-neutral-500",
                                "\n        虽然还不知道这个组件应该放什么内容，但我喜欢这个组件，占位\n       "
                            }
                            p { class: "mt-2 text-xs text-orange-500 underline",
                                a { href: "https://lexingtonthemes.com",
                                    "\n         给这组件打个广告"
                                }
                            }
                        }
                    }
                    div { class: "flex",
                        div { class: "flex flex-col divide-y divide-neutral-200",
                            div { class: "flex h-0 flex-1",
                                a {
                                    href: "https://lexingtonthemes.com/tutorials/how-to-create-animated-images-with-tailwind-css-and-astro-js/",
                                    target: "_blank",
                                    r#type: "button",
                                    class: "flex w-full items-center justify-center rounded-none rounded-tr-lg border border-transparent px-4 py-3 text-sm font-medium text-orange-600 hover:text-orange-500 focus:z-10 focus:outline-none focus:ring-2 focus:ring-orange-500",
                                    "View my photography works"
                                }
                            }
                            div { class: "flex h-0 flex-1",
                                a {
                                    target: "_blank",
                                    href: "https://github.com/UnwrappedDesign/lexington-tutorials/tree/main/src/pages/simplified-cards",
                                    class: "flex w-full items-center justify-center rounded-none rounded-br-lg border border-transparent px-4 py-3 text-sm font-medium text-neutral-700 hover:text-neutral-500 focus:outline-none focus:ring-2 focus:ring-orange-500",
                                    "Get the code"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
