use dioxus::prelude::*;

#[component]
pub fn Test() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/css/tailwind.css") }

        div { class: "flex items-center justify-center p-12",
        div { class: "mx-auto w-full max-w-[550px] bg-white",
            form {
                action: "/api/upload",
                method: "POST",
                enctype: "multipart/form-data",
                class: "py-6 px-9",
                div { class: "mb-5",
                    label {
                        r#for: "email",
                        class: "mb-3 block text-base font-medium text-[#07074D]",
                        "\n            Send files to this email:\n          "
                    }
                    input {
                        r#type: "email",
                        name: "email",
                        placeholder: "example@domain.com",
                        class: "w-full rounded-md border border-[#e0e0e0] bg-white py-3 px-6 text-base font-medium text-[#6B7280] outline-none focus:border-[#6A64F1] focus:shadow-md",
                        id: "email",
                    }
                }
                div { class: "mb-6 pt-4",
                    label { class: "mb-5 block text-xl font-semibold text-[#07074D]",
                        "Upload File"
                    }
                    div { class: "mb-8",
                        input {
                            name: "file",
                            r#type: "file",
                            class: "sr-only",
                            id: "file",
                        }
                        label {
                            r#for: "file",
                            class: "relative flex min-h-[200px] items-center justify-center rounded-md border border-dashed border-[#e0e0e0] p-12 text-center",
                            div {
                                span { class: "mb-2 block text-xl font-semibold text-[#07074D]",
                                    "\n                  Drop files here\n                "
                                }
                                span { class: "mb-2 block text-base font-medium text-[#6B7280]",
                                    "\n                  Or\n                "
                                }
                                span { class: "inline-flex rounded border border-[#e0e0e0] py-2 px-7 text-base font-medium text-[#07074D]",
                                    "\n                  Browse\n                "
                                }
                            }
                        }
                    }
                    div { class: "mb-5 rounded-md bg-[#F5F7FB] py-4 px-8",
                        div { class: "flex items-center justify-between",
                            span { class: "truncate pr-3 text-base font-medium text-[#07074D]",
                                "banner-design.png\n              "
                            }
                            button { class: "text-[#07074D]",
                                svg {
                                    fill: "none",
                                    "viewBox": "0 0 10 10",
                                    height: "10",
                                    width: "10",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path {
                                        fill: "currentColor",
                                        d: "M0.279337 0.279338C0.651787 -0.0931121 1.25565 -0.0931121 1.6281 0.279338L9.72066 8.3719C10.0931 8.74435 10.0931 9.34821 9.72066 9.72066C9.34821 10.0931 8.74435 10.0931 8.3719 9.72066L0.279337 1.6281C-0.0931125 1.25565 -0.0931125 0.651788 0.279337 0.279338Z",
                                        "fill-rule": "evenodd",
                                        "clip-rule": "evenodd",
                                    }
                                    path {
                                        "fill-rule": "evenodd",
                                        fill: "currentColor",
                                        "clip-rule": "evenodd",
                                        d: "M0.279337 9.72066C-0.0931125 9.34821 -0.0931125 8.74435 0.279337 8.3719L8.3719 0.279338C8.74435 -0.0931127 9.34821 -0.0931123 9.72066 0.279338C10.0931 0.651787 10.0931 1.25565 9.72066 1.6281L1.6281 9.72066C1.25565 10.0931 0.651787 10.0931 0.279337 9.72066Z",
                                    }
                                }
                            }
                        }
                    }
                    div { class: "rounded-md bg-[#F5F7FB] py-4 px-8",
                        div { class: "flex items-center justify-between",
                            span { class: "truncate pr-3 text-base font-medium text-[#07074D]",
                                "\n                banner-design.png\n              "
                            }
                            button { class: "text-[#07074D]",
                                svg {
                                    width: "10",
                                    fill: "none",
                                    "viewBox": "0 0 10 10",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    height: "10",
                                    path {
                                        "fill-rule": "evenodd",
                                        d: "M0.279337 0.279338C0.651787 -0.0931121 1.25565 -0.0931121 1.6281 0.279338L9.72066 8.3719C10.0931 8.74435 10.0931 9.34821 9.72066 9.72066C9.34821 10.0931 8.74435 10.0931 8.3719 9.72066L0.279337 1.6281C-0.0931125 1.25565 -0.0931125 0.651788 0.279337 0.279338Z",
                                        "clip-rule": "evenodd",
                                        fill: "currentColor",
                                    }
                                    path {
                                        fill: "currentColor",
                                        "fill-rule": "evenodd",
                                        d: "M0.279337 9.72066C-0.0931125 9.34821 -0.0931125 8.74435 0.279337 8.3719L8.3719 0.279338C8.74435 -0.0931127 9.34821 -0.0931123 9.72066 0.279338C10.0931 0.651787 10.0931 1.25565 9.72066 1.6281L1.6281 9.72066C1.25565 10.0931 0.651787 10.0931 0.279337 9.72066Z",
                                        "clip-rule": "evenodd",
                                    }
                                }
                            }
                        }
                        div { class: "relative mt-5 h-[6px] w-full rounded-lg bg-[#E2E5EF]",
                            div { class: "absolute left-0 right-0 h-full w-[75%] rounded-lg bg-[#6A64F1]" }
                        }
                    }
                }
                div {
                    button { class: "hover:shadow-form w-full rounded-md bg-[#6A64F1] py-3 px-8 text-center text-base font-semibold text-white outline-none",
                        "\n            Send File\n          "
                    }
                }
            }
        }
    }    }
}
