//! 这个用来放置本blog项目的关于页面

use dioxus::prelude::*;

const ABOUT_CSS: Asset = asset!("/assets/about.css");

#[component]
pub fn About() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: ABOUT_CSS },

        div {
            class:"flex items-center h-screen-4 justify-center bg-gray-100",
                div { class: "text-center max-w-2xl p-8 bg-white shadow-lg rounded-lg border border-gray-200",
                h1 { class: "text-4xl font-bold text-gray-800 mb-4 pt-8 pb-4", "🚧 网站正在建设中" }
                p { class: "text-lg text-gray-600 mb-6 px-9",
                    "这个网站的框架是Rust+Dioxus+tailwindcss+Axum+Postgresql+sqlx"
                }
                p { class: "text-lg text-gray-600 mb-6 px-9",
                    "\n我们正在努力完善本站，以提供更好的服务和体验。当前部分功能尚未完成，感谢您的理解与支持！\n        "
                }
                div { class: "text-left space-y-4 mb-8",
                    p { class: "text-md text-gray-700",
                        "\n                ⚙\u{fe0f} "
                        span { class: "font-semibold", "文章目录栏未实现：" }
                        " 目前文章页面缺少目录栏，我们正在开发中。\n            "
                    }
                    p { class: "text-md text-gray-700",
                        "\n                🔍 "
                        span { class: "font-semibold", "搜索功能未实现：" }
                        " 网站暂时无法提供搜索功能，敬请期待。\n            "
                    }
                    p { class: "text-md text-gray-700",
                        "\n                📚 "
                        span { class: "font-semibold", "文章数量较少：" }
                        " 我们正在积极创作更多优质内容，敬请关注。\n            "
                    }
                    p { class: "text-md text-gray-700",
                        "\n                💬 "
                        span { class: "font-semibold", "评论功能未开放：" }
                        " 懒得做这个。\n            "
                    }
                }
                p { class: "text-md text-gray-500 mb-8",
                    "\n            如果您有任何建议或问题，欢迎通过以下方式联系我们：\n        "
                }
                div { class: "space-y-4",
                    p { class: "text-sm text-gray-600",
                        "📧 邮箱:"
                        a {
                            href: "mailto:support@example.com",
                            class: "text-blue-500 hover:underline",
                            "chenjia01@foxmail.com"
                        }
                    }
                    p { class: "text-sm text-gray-600",
                        "📱 微信:"
                        a {
                            href: "#",
                            class: "text-blue-500 hover:underline",
                            "zackjchen"
                        }
                    }
                }
                div { class: "mt-8 p-5",
                    a {
                        href: "/",
                        class: "px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition duration-300",
                        "返回首页"
                    }
                }
            }
        }

    }
}
