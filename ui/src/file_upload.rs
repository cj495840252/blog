use dioxus::prelude::*;
const FILE_UPLOAD_CSS: Asset = asset!("/assets/css/file_upload.css");
#[component]
pub fn FileUpload() -> Element {
    rsx! {

        document::Link { rel: "stylesheet", href: FILE_UPLOAD_CSS }
        div {
            class: "file-upload-box",
            div {
                class: "file-upload w-full p-10 bg-white rounded-xl z-9",
                div {
                    class: "text-center",
                    h2 { class: "mt-5 text-3xl font-bold text-gray-900", "\n\t\t\t\tArticle Upload!\n\t\t\t" }
                    p { class: "mt-2 text-sm text-gray-400", "上传你的文章, 目前只支持markdown." }
                }
                form {
                    action: "/api/upload", method: "POST", class: "mt-8 space-y-3", enctype: "multipart/form-data",
                    div {
                        class: "grid grid-cols-1 space-y-2",
                        label { class: "text-sm font-bold text-gray-500 tracking-wide", "Title" }
                        input {
                            placeholder: "article title",
                            r#type: "",
                            class: "text-base p-2 border border-gray-300 rounded-lg focus:outline-none focus:border-indigo-500 ",
                        }
                    }
                    div {
                        class: "grid grid-cols-1 space-y-2",
                        label { class: "text-sm font-bold text-gray-500 tracking-wide", "Description" }
                        textarea {
                            rows: "4",
                            placeholder: "Your article description...",
                            class: "block p-2.5 w-full text-sm text-gray-900  rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500",
                            id: "article_description",
                        }
                    }
                    div {
                        class: "grid grid-cols-1 space-y-2",
                        label { class: "text-sm font-bold text-gray-500 tracking-wide", "Cover" }
                        link {
                            rel: "stylesheet",
                            href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css",
                        }
                        div {
                            class: "relative border-dotted h-48 rounded-lg border-dashed border-2 text-gray-500  flex justify-center items-center",
                            div { class: "absolute",
                                div { class: "flex flex-col items-center",
                                    i { class: "fa fa-folder-open fa-4x text-blue-700" }
                                    span { class: "block text-gray-400 font-normal", "Attach you files here" }
                                }
                            }
                            input {
                                name: "封面",
                                r#type: "file",
                                class: "h-full w-full opacity-0",
                            }
                        }
                    }

                    div {
                        class: "grid grid-cols-1 space-y-2",
                        label { class: "text-sm font-bold text-gray-500 tracking-wide", "Attach Document" }
                        div {
                            class: "flex items-center justify-center w-full",
                            label {
                                class: "flex flex-col rounded-lg border-4 border-dashed w-full h-60 p-10 group text-center",
                                div {
                                    class: "h-full w-full text-center flex flex-col items-center justify-center items-center",
                                    // 这里应该添加文件后就应该被替换掉
                                    // div { class: "flex flex-auto max-h-48 w-2/5 mx-auto -mt-10",
                                    //     img {
                                    //         src: "https://img.freepik.com/free-vector/image-upload-concept-landing-page_52683-27130.jpg?size=338&ext=jpg",
                                    //         alt: "freepik image",
                                    //         class: "has-mask h-36 object-center",
                                    //     }
                                    // }
                                    // p { class: "pointer-none text-gray-500",
                                    //     span { class: "text-sm", "Drag and drop" }
                                    //     " files here "
                                    //     br {}
                                    //     " or "
                                    //     a {
                                    //         href: "",
                                    //         class: "text-blue-600 hover:underline",
                                    //         id: "",
                                    //         "select a file"
                                    //     }
                                    //     " from your computer"
                                    // }
                                    // 上面这一段等文件上传后应该被下面这个替代
                                    // div {
                                    //     class: "mb-5 rounded-md bg-[#F5F7FB] py-4 px-8",
                                    //     div { class: "items-center justify-between",
                                    //         span { class: "truncate pr-3 text-base font-medium text-[#07074D]",
                                    //             "\n                banner-design.png\n              "
                                    //         }
                                    //         button {
                                    //             class: "text-[#07074D]",
                                    //             svg {
                                    //                 fill: "none",
                                    //                 "viewBox": "0 0 10 10",
                                    //                 height: "10",
                                    //                 width: "10",
                                    //                 xmlns: "http://www.w3.org/2000/svg",
                                    //                 path {
                                    //                     fill: "currentColor",
                                    //                     d: "M0.279337 0.279338C0.651787 -0.0931121 1.25565 -0.0931121 1.6281 0.279338L9.72066 8.3719C10.0931 8.74435 10.0931 9.34821 9.72066 9.72066C9.34821 10.0931 8.74435 10.0931 8.3719 9.72066L0.279337 1.6281C-0.0931125 1.25565 -0.0931125 0.651788 0.279337 0.279338Z",
                                    //                     "fill-rule": "evenodd",
                                    //                     "clip-rule": "evenodd",
                                    //                 }
                                    //                 path {
                                    //                     "fill-rule": "evenodd",
                                    //                     fill: "currentColor",
                                    //                     "clip-rule": "evenodd",
                                    //                     d: "M0.279337 9.72066C-0.0931125 9.34821 -0.0931125 8.74435 0.279337 8.3719L8.3719 0.279338C8.74435 -0.0931127 9.34821 -0.0931123 9.72066 0.279338C10.0931 0.651787 10.0931 1.25565 9.72066 1.6281L1.6281 9.72066C1.25565 10.0931 0.651787 10.0931 0.279337 9.72066Z",
                                    //                 }
                                    //             }
                                    //         }
                                    //     }
                                    // }
                                }
                                input { name: "files", r#type: "file", class: "hidden", multiple: "true"}
                            }
                        }
                    }
                    p { class: "text-sm text-gray-300",
                        span { "上传文件的时候请将图片一起上传，只支持jpg,jepg,png的图片和markdown文件" }
                    }

                    div {
                        button {
                            r#type: "submit",
                            class: "my-5 w-full flex justify-center bg-blue-500 text-gray-100 p-4 rounded-full tracking-wide font-semibold focus:outline-none focus:shadow-outline hover:bg-blue-600 shadow-lg cursor-pointer transition ease-in duration-300",
                            "Upload"
                        }
                    }
                }
            }

         }
    }
}
