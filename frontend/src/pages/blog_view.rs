use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use solana_blog_client::{BlogInfo, PostInfo};
#[cfg(target_arch = "wasm32")]
use crate::services::{BlogInfo, PostInfo};
use crate::app::{AppState, Route};
use crate::components::LoadingSpinner;

#[derive(Props, PartialEq)]
pub struct BlogViewProps {
    pub app_state: UseState<AppState>,
    pub blog_pubkey: String,
}

pub fn BlogView(cx: Scope<BlogViewProps>) -> Element {
    let blog_info = use_state(cx, || None::<BlogInfo>);
    let posts = use_state(cx, || Vec::<PostInfo>::new());
    let loading = use_state(cx, || true);

    // Load blog data
    use_effect(cx, &cx.props.blog_pubkey, {
        let blog_info = blog_info.clone();
        let posts = posts.clone();
        let loading = loading.clone();
        let blog_pubkey = cx.props.blog_pubkey.clone();
        
        move |_| async move {
            // Simulate API delay
            #[cfg(target_arch = "wasm32")]
            {
                let _ = gloo_timers::future::TimeoutFuture::new(1000).await;
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                use std::time::Duration;
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }

            // Mock blog data
            if blog_pubkey == crate::config::DEMO_BLOG_PUBKEY {
                let mock_blog = BlogInfo {
                    #[cfg(target_arch = "wasm32")]
                    pubkey: blog_pubkey.clone(),
                    #[cfg(not(target_arch = "wasm32"))]
                    pubkey: blog_pubkey.parse().unwrap(),
                    #[cfg(target_arch = "wasm32")]
                    authority: "11111111111111111111111111111114".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    authority: "11111111111111111111111111111114".parse().unwrap(),
                    title: "Solana verystochastic".to_string(),
                    description: "Decentralized finance disasters and lessons from the blockchain".to_string(),
                    post_count: 2,
                    created_at: 1699123456,
                };

                let mock_posts = vec![
                    PostInfo {
                        #[cfg(target_arch = "wasm32")]
                        pubkey: "11111111111111111111111111111115".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        pubkey: "11111111111111111111111111111115".parse().unwrap(),
                        #[cfg(target_arch = "wasm32")]
                        blog: blog_pubkey.clone(),
                        #[cfg(not(target_arch = "wasm32"))]
                        blog: blog_pubkey.parse().unwrap(),
                        #[cfg(target_arch = "wasm32")]
                        author: "11111111111111111111111111111116".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        author: "11111111111111111111111111111116".parse().unwrap(),
                        title: "GMX - verystochastic".to_string(),
                        content: "The largest GMX exploit in DeFi history. Over $50M drained from liquidity pools due to a price manipulation attack on Arbitrum.".to_string(),
                        created_at: 1699789012,
                        #[cfg(target_arch = "wasm32")]
                        image_url: Some("/api/placeholder/600/300".to_string()),
                        #[cfg(not(target_arch = "wasm32"))]
                        arweave_hash: "mock_arweave_hash_1".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        updated_at: 1699789012,
                    },
                    PostInfo {
                        #[cfg(target_arch = "wasm32")]
                        pubkey: "11111111111111111111111111111117".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        pubkey: "11111111111111111111111111111117".parse().unwrap(),
                        #[cfg(target_arch = "wasm32")]
                        blog: blog_pubkey.clone(),
                        #[cfg(not(target_arch = "wasm32"))]
                        blog: blog_pubkey.parse().unwrap(),
                        #[cfg(target_arch = "wasm32")]
                        author: "11111111111111111111111111111116".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        author: "11111111111111111111111111111116".parse().unwrap(),
                        title: "Solana Validator - verystochastic".to_string(),
                        content: "Major Solana validator cluster went down for 17 hours due to a botched network upgrade.".to_string(),
                        created_at: 1699702345,
                        #[cfg(target_arch = "wasm32")]
                        image_url: Some("/api/placeholder/600/300".to_string()),
                        #[cfg(not(target_arch = "wasm32"))]
                        arweave_hash: "mock_arweave_hash_2".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        updated_at: 1699702345,
                    },
                ];

                blog_info.set(Some(mock_blog));
                posts.set(mock_posts);
            }

            loading.set(false);
        }
    });

    let handle_back = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::Home;
            app_state.set(new_state);
        }
    };

    let handle_create_post = {
        let app_state = cx.props.app_state.clone();
        let blog_pubkey = cx.props.blog_pubkey.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::CreatePost(blog_pubkey.clone());
            app_state.set(new_state);
        }
    };

    cx.render(rsx! {
        div {
            class: "min-h-screen bg-black text-white",
            style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",

            // Header
            header {
                class: "border-b border-gray-800 bg-black",
                div {
                    class: "max-w-6xl mx-auto px-4 py-4 flex items-center justify-between",
                    
                    button {
                        class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider",
                        onclick: handle_back,
                        "← verystochastic"
                    }
                    
                    if let Some(blog) = blog_info.get() {
                        rsx! {
                            div {
                                class: "flex items-center space-x-4",
                                h1 {
                                    class: "text-xl font-mono text-white",
                                    "{blog.title}"
                                }
                                span {
                                    class: "text-gray-500 text-sm",
                                    "{posts.len()} posts"
                                }
                            }
                        }
                    }
                    
                    div {
                        class: "flex items-center space-x-4",
                        
                        // Wallet status indicator
                        if cx.props.app_state.get().wallet_service.connected {
                            rsx! {
                                div {
                                    class: "text-xs text-green-400 border border-green-800 px-2 py-1 rounded",
                                    "🟢 Connected"
                                }
                            }
                        }
                        
                        // Create post button (only show if user is admin)
                        if cx.props.app_state.get().is_admin() {
                            rsx! {
                                button {
                                    class: "border border-gray-700 text-white px-4 py-2 text-sm hover:bg-gray-900 transition-colors uppercase tracking-wider",
                                    onclick: handle_create_post,
                                    "new post"
                                }
                            }
                        }
                    }
                }
            }

            // Main content
            main {
                class: "max-w-4xl mx-auto px-4 py-8",
                
                // Blog description
                if let Some(blog) = blog_info.get() {
                    rsx! {
                        div {
                            class: "mb-12 border-b border-gray-800 pb-8",
                            p {
                                class: "text-gray-400 text-lg leading-relaxed",
                                "{blog.description}"
                            }
                        }
                    }
                }

                // Loading state
                if *loading.get() {
                    rsx! {
                        div {
                            class: "flex justify-center py-12",
                            LoadingSpinner { message: "Loading posts...".to_string() }
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "space-y-8",
                            
                            h2 {
                                class: "text-xl font-bold mb-6 text-white uppercase border-b border-gray-800 pb-2",
                                "ALL POSTS"
                            }
                            
                            if posts.is_empty() {
                                rsx! {
                                    div {
                                        class: "text-center py-12",
                                        p {
                                            class: "text-gray-500 text-lg",
                                            "No posts yet. Be the first to document an incident!"
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div {
                                        class: "space-y-6",
                                        for (idx, post) in posts.iter().enumerate() {
                                            {
                                                let post_blog = post.blog.to_string();
                                                let post_pubkey = post.pubkey.to_string();
                                                let app_state_clone = cx.props.app_state.clone();
                                                
                                                rsx! {
                                                    article {
                                                        key: "{idx}",
                                                        class: "border-b border-gray-800 pb-6 cursor-pointer hover:bg-gray-900 hover:bg-opacity-30 transition-colors p-4 -mx-4",
                                                        onclick: {
                                                            move |_| {
                                                                let mut new_state = app_state_clone.get().clone();
                                                                new_state.current_route = Route::Post(post_blog.clone(), post_pubkey.clone());
                                                                app_state_clone.set(new_state);
                                                            }
                                                        },
                                                        
                                                        // Post metadata
                                                        div {
                                                            class: "flex items-center text-xs text-gray-500 mb-3 font-mono uppercase tracking-wider",
                                                            span { "{crate::utils::format_timestamp(post.created_at)}" }
                                                            span { class: "mx-2", "•" }
                                                            span { "By {post.author:?}" }
                                                        }
                                                        
                                                        // Post title
                                                        h3 {
                                                            class: "text-lg font-bold mb-3 text-white uppercase",
                                                            "{post.title}"
                                                        }
                                                        
                                                        // Post preview
                                                        p {
                                                            class: "text-gray-400 mb-4 leading-relaxed",
                                                            "{crate::utils::truncate_string(&post.content, 200)}"
                                                        }
                                                        
                                                        // Read more indicator
                                                        div {
                                                            class: "text-sm text-gray-500 uppercase tracking-wider",
                                                            "read more →"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
} 