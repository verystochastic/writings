#[cfg(not(target_arch = "wasm32"))]
use solana_blog_client::PostInfo;
#[cfg(target_arch = "wasm32")]
use crate::services::PostInfo;
use dioxus::prelude::*;

#[derive(Props)]
pub struct PostCardProps {
    pub post: PostInfo,
    pub on_click: EventHandler<'static, ()>,
}

impl PartialEq for PostCardProps {
    fn eq(&self, other: &Self) -> bool {
        self.post == other.post
    }
}

pub fn PostCard(cx: Scope<PostCardProps>) -> Element {
    cx.render(rsx! {
        article {
            class: "border-b border-gray-800 pb-6 cursor-pointer hover:bg-gray-900 hover:bg-opacity-30 transition-colors p-4 -mx-4",
            onclick: move |_| cx.props.on_click.call(()),
            
            // Post metadata
            div {
                class: "flex items-center text-xs text-gray-500 mb-3 font-mono uppercase tracking-wider",
                span { "{crate::utils::format_timestamp(cx.props.post.created_at)}" }
                span { class: "mx-2", "•" }
                span { "By {cx.props.post.author:?}" }
            }
            
            // Post title
            h3 {
                class: "text-lg font-bold mb-3 text-white uppercase",
                "{cx.props.post.title}"
            }
            
            // Post preview
            p {
                class: "text-gray-400 mb-4 leading-relaxed",
                "{crate::utils::truncate_string(&cx.props.post.content, 200)}"
            }
            
            // Read more indicator
            div {
                class: "text-sm text-gray-500 uppercase tracking-wider",
                "read more →"
            }
        }
    })
}

// Additional components
#[derive(Props)]
pub struct BlogHeaderProps {
    pub title: String,
    pub description: String,
    pub on_back: EventHandler<'static, ()>,
    pub on_create_post: EventHandler<'static, ()>,
}

impl PartialEq for BlogHeaderProps {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.description == other.description
    }
}

pub fn BlogHeader(cx: Scope<BlogHeaderProps>) -> Element {
    cx.render(rsx! {
        header {
            class: "border-b border-gray-800 bg-black",
            div {
                class: "max-w-6xl mx-auto px-4 py-6",
                div {
                    class: "flex justify-between items-start",
                    
                    div {
                        class: "flex-1",
                        button {
                            class: "text-gray-400 hover:text-white mb-4 flex items-center space-x-2",
                            onclick: move |_| cx.props.on_back.call(()),
                            span { "←" }
                            span { "BACK" }
                        }
                        h1 {
                            class: "text-2xl md:text-3xl font-bold text-white uppercase tracking-wider mb-2",
                            "{cx.props.title}"
                        }
                        p {
                            class: "text-gray-400 max-w-2xl",
                            "{cx.props.description}"
                        }
                    }
                    
                    button {
                        class: "border border-gray-700 text-white px-4 py-2 text-sm hover:bg-gray-900 transition-colors uppercase tracking-wider",
                        onclick: move |_| cx.props.on_create_post.call(()),
                        "+ NEW POST"
                    }
                }
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub struct LoadingSpinnerProps {
    pub message: String,
}

pub fn LoadingSpinner(cx: Scope<LoadingSpinnerProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col items-center justify-center py-12",
            div {
                class: "animate-spin rounded-full h-12 w-12 border-b-2 border-white mb-4"
            }
            p {
                class: "text-gray-400 text-sm uppercase tracking-wider",
                "{cx.props.message}"
            }
        }
    })
} 