use dioxus::prelude::*;
use pulldown_cmark::Parser;

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: Signal<String>,

    content: ReadOnlySignal<String>,
}

const MARKDOWN_CSSS: Asset = asset!("/assets/styling/markdown.css");

/// Render some text as markdown.
#[component]
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    let parser = Parser::new(content);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        document::Link{ rel:"stylesheet", href: MARKDOWN_CSSS}
        div {
            class: "markdown",
            padding_top: "2rem",
            id: "{&*props.id.read()}",
            class: "{&*props.class.read()}",
            dangerous_inner_html: "{html_buf}"
        }
    }
}
