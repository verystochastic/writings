use dioxus::prelude::*;
use crate::app::{AppState, Route};
use crate::components::LoadingSpinner;

#[derive(Props, PartialEq)]
pub struct CreatePostProps {
    pub blog_pubkey: String,
    pub app_state: UseState<AppState>,
}

pub fn CreatePost(cx: Scope<CreatePostProps>) -> Element {
    let blog_pubkey = &cx.props.blog_pubkey;
    let loading = use_state(cx, || false);
    let error = use_state(cx, || None::<String>);
    let success = use_state(cx, || false);

    let handle_back = {
        let app_state = cx.props.app_state.clone();
        let blog_pubkey = blog_pubkey.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::Blog(blog_pubkey.clone());
            app_state.set(new_state);
        }
    };

    if *loading.get() {
        return cx.render(rsx! {
            div {
                class: "min-h-screen bg-black text-white flex items-center justify-center",
                style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
                LoadingSpinner {
                    message: "Creating your post...".to_string()
                }
            }
        });
    }

    if *success.get() {
        return cx.render(rsx! {
            div {
                class: "min-h-screen bg-black text-white",
                style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
                div {
                    class: "max-w-2xl mx-auto px-4 py-24 text-center",
                    div {
                        class: "mb-8",
                        div {
                            class: "text-4xl mb-6",
                            "PUBLISHED"
                        }
                        h2 {
                            class: "text-2xl font-light mb-4 tracking-wide",
                            "POST PUBLISHED"
                        }
                        p {
                            class: "text-gray-400 mb-8",
                            "Your post has been created and stored on Arweave. It may take a few moments to propagate across the network."
                        }
                        div {
                            class: "space-x-4",
                            button {
                                class: "border border-gray-700 text-white px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                onclick: handle_back,
                                "← back to blog"
                            }
                            button {
                                class: "border border-gray-700 text-white px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                onclick: move |_| {
                                    success.set(false);
                                },
                                "write another"
                            }
                        }
                    }
                }
            }
        });
    }

    cx.render(rsx! {
        div {
            class: "min-h-screen bg-black text-white",
            style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
            
            // Header
            header {
                class: "border-b border-gray-800 px-4 py-4",
                div {
                    class: "max-w-4xl mx-auto flex items-center justify-between",
                    div {
                        class: "flex items-center space-x-8",
                        button {
                            class: "text-gray-400 hover:text-white transition-colors",
                            onclick: handle_back,
                            "← back"
                        }
                        h1 {
                            class: "text-xl font-bold tracking-wide",
                            "write post"
                        }
                    }
                    div {
                        class: "text-sm text-gray-500",
                        "#{crate::utils::truncate_pubkey(blog_pubkey)}"
                    }
                }
            }

            // Main content
            main {
                class: "max-w-4xl mx-auto px-4 py-12",
                
                // Error message
                if let Some(error_msg) = error.get() {
                    rsx! {
                        div {
                            class: "border border-red-800 bg-red-900/20 text-red-400 p-4 mb-8 text-sm",
                            "Error: {error_msg}"
                        }
                    }
                }

                // Form
                form {
                    class: "space-y-8",
                    onsubmit: move |e: FormEvent| {
                        let title = e.values.get("title").and_then(|v| v.first().cloned()).unwrap_or_default();
                        let content = e.values.get("content").and_then(|v| v.first().cloned()).unwrap_or_default();
                        
                        if title.is_empty() || content.is_empty() {
                            error.set(Some("Title and content are required".to_string()));
                            return;
                        }
                        
                        error.set(None);
                        loading.set(true);
                        
                        // Simulate post creation
                        cx.spawn({
                            let loading = loading.clone();
                            let success = success.clone();
                            async move {
                                // Simulate API delay
                                #[cfg(target_arch = "wasm32")]
                                {
                                    let _ = gloo_timers::future::TimeoutFuture::new(2000).await;
                                }
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    use std::time::Duration;
                                    tokio::time::sleep(Duration::from_millis(2000)).await;
                                }
                                
                                // Simulate success (in real app, this would be the actual API call)
                                loading.set(false);
                                success.set(true);
                            }
                        });
                    },
                    
                    // Title field
                    div {
                        class: "space-y-2",
                        label {
                            class: "block text-sm text-gray-400",
                            r#for: "title",
                            "title"
                        }
                        input {
                            id: "title",
                            name: "title",
                            r#type: "text",
                            class: "w-full bg-black border border-gray-700 text-white px-4 py-3 focus:border-gray-500 focus:outline-none text-lg font-light",
                            placeholder: "Enter your post title...",
                            required: true
                        }
                    }

                    // Content field
                    div {
                        class: "space-y-2",
                        label {
                            class: "block text-sm text-gray-400",
                            r#for: "content",
                            "content"
                        }
                        textarea {
                            id: "content",
                            name: "content",
                            class: "w-full bg-black border border-gray-700 text-white px-4 py-3 focus:border-gray-500 focus:outline-none font-light leading-relaxed resize-none",
                            placeholder: "Write your post content here...\n\nThis will be permanently stored on Arweave and indexed on Solana blockchain.",
                            rows: "20",
                            required: true
                        }
                    }

                    // Submit section
                    div {
                        class: "pt-4 border-t border-gray-800",
                        div {
                            class: "flex justify-between items-center",
                            div {
                                class: "text-sm text-gray-500",
                                "content will be stored permanently on arweave"
                            }
                            div {
                                class: "flex space-x-4",
                                button {
                                    r#type: "button",
                                    class: "border border-gray-700 text-gray-400 px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                    onclick: {
                                        let app_state = cx.props.app_state.clone();
                                        let blog_pubkey = blog_pubkey.clone();
                                        move |_| {
                                            let mut new_state = app_state.get().clone();
                                            new_state.current_route = Route::Blog(blog_pubkey.clone());
                                            app_state.set(new_state);
                                        }
                                    },
                                    "cancel"
                                }
                                button {
                                    r#type: "submit",
                                    class: "border border-gray-700 text-white px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                    "publish post"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
} 