use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use solana_blog_client::{BlogInfo, PostInfo};
#[cfg(target_arch = "wasm32")]
use crate::services::{BlogInfo, PostInfo};
use crate::app::{AppState, Route};
use crate::config::DEMO_BLOG_PUBKEY;

#[derive(Props, PartialEq)]
pub struct PostViewProps {
    pub blog_pubkey: String,
    pub post_pubkey: String,
    pub app_state: UseState<AppState>,
}

pub fn PostView(cx: Scope<PostViewProps>) -> Element {
    let post_info = use_state(cx, || None::<PostInfo>);
    let blog_info = use_state(cx, || None::<BlogInfo>);
    let loading = use_state(cx, || true);
    
    // Load post data on component mount
    use_effect(cx, (&cx.props.blog_pubkey, &cx.props.post_pubkey), {
        let post_info = post_info.clone();
        let blog_info = blog_info.clone();
        let loading = loading.clone();
        let blog_pubkey = cx.props.blog_pubkey.clone();
        let post_pubkey = cx.props.post_pubkey.clone();
        
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
            
            // Mock data
            if blog_pubkey == DEMO_BLOG_PUBKEY {
                let mock_blog = BlogInfo {
                    #[cfg(target_arch = "wasm32")]
                    pubkey: blog_pubkey.clone(),
                    #[cfg(not(target_arch = "wasm32"))]
                    pubkey: blog_pubkey.parse().unwrap_or_default(),
                    #[cfg(target_arch = "wasm32")]
                    authority: "11111111111111111111111111111114".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    authority: "11111111111111111111111111111114".parse().unwrap(),
                    title: "Solana verystochastic".to_string(),
                    description: "Decentralized finance disasters and lessons from the blockchain".to_string(),
                    post_count: 3,
                    created_at: 1699123456,
                };
                
                // Determine which post to show based on post_pubkey
                let mock_post = if post_pubkey == "11111111111111111111111111111115" {
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
                        content: "The largest GMX exploit in DeFi history. Over $50M drained from liquidity pools due to a price manipulation attack on Arbitrum. The attacker exploited the GMX price feed oracle by creating massive positions with borrowed funds, manipulating the underlying asset prices, and profiting from the price discrepancy. This incident highlights the risks of insufficient oracle protection and the importance of robust price validation mechanisms in perpetual trading protocols.".to_string(),
                        created_at: 1699789012,
                        #[cfg(target_arch = "wasm32")]
                        image_url: Some("/api/placeholder/800/400".to_string()),
                        #[cfg(not(target_arch = "wasm32"))]
                        arweave_hash: "mock_arweave_hash_1".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        updated_at: 1699789012,
                    }
                } else if post_pubkey == "11111111111111111111111111111117" {
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
                        content: "Major Solana validator cluster went down for 17 hours due to a botched network upgrade. The incident occurred during a routine protocol update that introduced a consensus bug, causing 80% of validators to halt block production. Recovery required emergency coordination between core developers and validator operators to roll back the problematic update and restore network stability.".to_string(),
                        created_at: 1699702345,
                        #[cfg(target_arch = "wasm32")]
                        image_url: Some("/api/placeholder/800/400".to_string()),
                        #[cfg(not(target_arch = "wasm32"))]
                        arweave_hash: "mock_arweave_hash_2".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        updated_at: 1699702345,
                    }
                } else {
                    PostInfo {
                        #[cfg(target_arch = "wasm32")]
                        pubkey: "11111111111111111111111111111118".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        pubkey: "11111111111111111111111111111118".parse().unwrap(),
                        #[cfg(target_arch = "wasm32")]
                        blog: blog_pubkey.clone(),
                        #[cfg(not(target_arch = "wasm32"))]
                        blog: blog_pubkey.parse().unwrap(),
                        #[cfg(target_arch = "wasm32")]
                        author: "11111111111111111111111111111116".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        author: "11111111111111111111111111111116".parse().unwrap(),
                        title: "DeFi Bridge - verystochastic".to_string(),
                        content: "Cross-chain bridge exploit leads to $80M drainage from Solana-Ethereum bridge. Attackers exploited a verification bug in the bridge's smart contract, allowing them to mint unlimited wrapped tokens on Ethereum using fake Solana transaction proofs. The vulnerability existed for months before discovery, with the protocol's multi-signature security being bypassed through a sophisticated social engineering attack on bridge validators.".to_string(),
                        created_at: 1699615678,
                        #[cfg(target_arch = "wasm32")]
                        image_url: Some("/api/placeholder/800/400".to_string()),
                        #[cfg(not(target_arch = "wasm32"))]
                        arweave_hash: "mock_arweave_hash_3".to_string(),
                        #[cfg(not(target_arch = "wasm32"))]
                        updated_at: 1699615678,
                    }
                };
                
                blog_info.set(Some(mock_blog));
                post_info.set(Some(mock_post));
            } else {
                blog_info.set(None);
                post_info.set(None);
            }
            
            loading.set(false);
        }
    });
    
    let handle_back = move |_| {
        let mut new_state = cx.props.app_state.get().clone();
        new_state.current_route = Route::Blog(cx.props.blog_pubkey.clone());
        cx.props.app_state.set(new_state);
    };

    cx.render(rsx! {
        div {
            class: "min-h-screen bg-black text-white",
            style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",

            // Header with back navigation
            header {
                class: "border-b border-gray-800 bg-black",
                div {
                    class: "max-w-6xl mx-auto px-4 py-4 flex items-center justify-between",
                    
                    button {
                        class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider",
                        onclick: handle_back,
                        "← verystochastic"
                    }
                    
                    // Admin edit button (only show if user is admin)
                    if cx.props.app_state.get().is_admin() {
                        rsx! {
                            button {
                                class: "text-green-400 hover:text-green-300 text-sm uppercase tracking-wider",
                                onclick: {
                                    let blog_pubkey = cx.props.blog_pubkey.clone();
                                    let post_pubkey = cx.props.post_pubkey.clone();
                                    let admin_app_state = cx.props.app_state.clone();
                                    move |_| {
                                        let mut new_state = admin_app_state.get().clone();
                                        new_state.current_route = Route::Admin;
                                        admin_app_state.set(new_state);
                                    }
                                },
                                "edit post"
                            }
                        }
                    }
                }
            }

            main {
                class: "max-w-4xl mx-auto px-4 py-8",

                if **loading {
                    rsx! {
                        div {
                            class: "flex justify-center py-12",
                            div { class: "animate-spin rounded-full h-8 w-8 border-b-2 border-white" }
                        }
                    }
                } else if let (Some(post), Some(_blog)) = (post_info.as_ref(), blog_info.as_ref()) {
                    rsx! {
                        article {
                            class: "space-y-8",
                            
                            // Post header
                            div {
                                class: "border-b border-gray-800 pb-8",
                                h1 {
                                    class: "text-3xl md:text-4xl font-bold mb-6 text-white uppercase leading-tight",
                                    "{post.title}"
                                }
                                div {
                                    class: "flex items-center space-x-4 text-sm text-gray-500",
                                    span { "By {post.author:?}" }
                                    span { "•" }
                                    span { "{crate::utils::format_timestamp(post.created_at)}" }
                                }
                            }
                            
                            // Post image - only show for WASM builds that have image_url
                            {
                                #[cfg(target_arch = "wasm32")]
                                if let Some(image_url) = &post.image_url {
                                    rsx! {
                                        div {
                                            class: "mb-8",
                                            img {
                                                src: "{image_url}",
                                                alt: "Post image",
                                                class: "w-full h-64 md:h-96 object-cover border border-gray-700"
                                            }
                                        }
                                    }
                                } else {
                                    rsx! { div {} }
                                }
                                
                                #[cfg(not(target_arch = "wasm32"))]
                                rsx! { div {} }
                            }
                            
                            // Post content
                            div {
                                class: "prose prose-invert prose-lg max-w-none",
                                style: "color: #d1d5db; font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
                                
                                for paragraph in post.content.split("\n\n") {
                                    p {
                                        class: "mb-6 leading-relaxed text-gray-300",
                                        "{paragraph}"
                                    }
                                }
                            }
                            
                            // Post footer
                            div {
                                class: "border-t border-gray-800 pt-8 mt-12",
                                div {
                                    class: "text-center text-gray-500",
                                    p { class: "mb-2", "End of article" }
                                    button {
                                        class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider border border-gray-700 px-4 py-2 hover:bg-gray-900 transition-colors",
                                        onclick: handle_back,
                                        "← Back to posts"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    rsx! {
                        div {
                            class: "text-center py-12 text-gray-500",
                            h1 { class: "text-2xl mb-4", "Post not found" }
                            p { class: "mb-8", "The post you're looking for doesn't exist." }
                            button {
                                class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider border border-gray-700 px-4 py-2 hover:bg-gray-900 transition-colors",
                                onclick: handle_back,
                                "← Back to posts"
                            }
                        }
                    }
                }
            }
        }
    })
} 