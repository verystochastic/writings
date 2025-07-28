use dioxus::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use solana_blog_client::{BlogInfo, PostInfo};
#[cfg(target_arch = "wasm32")]
use crate::services::{BlogInfo, PostInfo};
use crate::app::{AppState, Route};
use crate::components::LoadingSpinner;
use crate::config::DEMO_BLOG_PUBKEY;

#[derive(Props, PartialEq)]
pub struct HomeProps {
    pub app_state: UseState<AppState>,
}

pub fn Home(cx: Scope<HomeProps>) -> Element {
    let blogs = use_state(cx, || Vec::<BlogInfo>::new());
    let recent_posts = use_state(cx, || Vec::<PostInfo>::new());
    let loading = use_state(cx, || true);

    // Simulate loading data
    use_effect(cx, (), {
        let blogs = blogs.clone();
        let recent_posts = recent_posts.clone();
        let loading = loading.clone();
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
            let mock_blog = BlogInfo {
                #[cfg(target_arch = "wasm32")]
                pubkey: DEMO_BLOG_PUBKEY.to_string(),
                #[cfg(not(target_arch = "wasm32"))]
                pubkey: DEMO_BLOG_PUBKEY.parse().unwrap(),
                #[cfg(target_arch = "wasm32")]
                authority: "11111111111111111111111111111114".to_string(),
                #[cfg(not(target_arch = "wasm32"))]
                authority: "11111111111111111111111111111114".parse().unwrap(),
                title: "Solana verystochastic".to_string(),
                description: "Decentralized finance disasters and lessons from the blockchain".to_string(),
                post_count: 5,
                created_at: 1699123456,
            };

            let mock_posts = vec![
                PostInfo {
                    #[cfg(target_arch = "wasm32")]
                    pubkey: "11111111111111111111111111111115".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    pubkey: "11111111111111111111111111111115".parse().unwrap(),
                    #[cfg(target_arch = "wasm32")]
                    blog: DEMO_BLOG_PUBKEY.to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    blog: DEMO_BLOG_PUBKEY.parse().unwrap(),
                    #[cfg(target_arch = "wasm32")]
                    author: "11111111111111111111111111111116".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    author: "11111111111111111111111111111116".parse().unwrap(),
                    title: "GMX - verystochastic".to_string(),
                    content: "The largest GMX exploit in DeFi history. Over $50M drained from liquidity pools due to a price manipulation attack on Arbitrum. The attacker exploited the GMX price feed oracle by creating massive positions with borrowed funds, manipulating the underlying asset prices, and profiting from the price discrepancy. This incident highlights the risks of insufficient oracle protection and the importance of robust price validation mechanisms in perpetual trading protocols.".to_string(),
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
                    blog: DEMO_BLOG_PUBKEY.to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    blog: DEMO_BLOG_PUBKEY.parse().unwrap(),
                    #[cfg(target_arch = "wasm32")]
                    author: "11111111111111111111111111111116".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    author: "11111111111111111111111111111116".parse().unwrap(),
                    title: "Solana Validator - verystochastic".to_string(),
                    content: "Major Solana validator cluster went down for 17 hours due to a botched network upgrade. The incident occurred during a routine protocol update that introduced a consensus bug, causing 80% of validators to halt block production. Recovery required emergency coordination between core developers and validator operators to roll back the problematic update and restore network stability.".to_string(),
                    created_at: 1699702345,
                    #[cfg(target_arch = "wasm32")]
                    image_url: Some("/api/placeholder/600/300".to_string()),
                    #[cfg(not(target_arch = "wasm32"))]
                    arweave_hash: "mock_arweave_hash_2".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    updated_at: 1699702345,
                },
                PostInfo {
                    #[cfg(target_arch = "wasm32")]
                    pubkey: "11111111111111111111111111111118".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    pubkey: "11111111111111111111111111111118".parse().unwrap(),
                    #[cfg(target_arch = "wasm32")]
                    blog: DEMO_BLOG_PUBKEY.to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    blog: DEMO_BLOG_PUBKEY.parse().unwrap(),
                    #[cfg(target_arch = "wasm32")]
                    author: "11111111111111111111111111111116".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    author: "11111111111111111111111111111116".parse().unwrap(),
                    title: "DeFi Bridge - verystochastic".to_string(),
                    content: "Cross-chain bridge exploit leads to $80M drainage from Solana-Ethereum bridge. Attackers exploited a verification bug in the bridge's smart contract, allowing them to mint unlimited wrapped tokens on Ethereum using fake Solana transaction proofs. The vulnerability existed for months before discovery, with the protocol's multi-signature security being bypassed through a sophisticated social engineering attack on bridge validators.".to_string(),
                    created_at: 1699615678,
                    #[cfg(target_arch = "wasm32")]
                    image_url: Some("/api/placeholder/600/300".to_string()),
                    #[cfg(not(target_arch = "wasm32"))]
                    arweave_hash: "mock_arweave_hash_3".to_string(),
                    #[cfg(not(target_arch = "wasm32"))]
                    updated_at: 1699615678,
                },
            ];

            blogs.set(vec![mock_blog]);
            recent_posts.set(mock_posts);
            loading.set(false);
        }
    });

    let handle_blog_click = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::Blog(DEMO_BLOG_PUBKEY.to_string());
            app_state.set(new_state);
        }
    };

    let handle_about_click = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::About;
            app_state.set(new_state);
        }
    };

    let handle_admin_click = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::Admin;
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
                    
                    // Logo
                    div {
                        class: "flex items-center space-x-4",
                        h1 {
                            class: "text-xl font-mono text-white",
                            "verystochastic"
                        }
                    }
                    
                    // Navigation
                    nav {
                        class: "flex items-center space-x-8",
                        
                        button {
                            class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider transition-colors",
                            onclick: handle_blog_click,
                            "Blog"
                        }
                        
                        button {
                            class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider transition-colors",
                            onclick: handle_about_click,
                            "About"
                        }
                        
                        button {
                            class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider transition-colors",
                            onclick: handle_admin_click,
                            if cx.props.app_state.get().wallet_service.connected {
                                if cx.props.app_state.get().is_admin() {
                                    "Admin ✅"
                                } else {
                                    "Admin 🔗"
                                }
                            } else {
                                "Admin"
                            }
                        }
                        
                        // Wallet status indicator
                        if cx.props.app_state.get().wallet_service.connected {
                            rsx! {
                                div {
                                    class: "text-xs text-green-400 border border-green-800 px-2 py-1 rounded",
                                    "🟢 Connected"
                                }
                            }
                        }
                    }
                }
            }

            // Main content
            main {
                class: "max-w-6xl mx-auto px-4 py-12",
                
                // Hero section
                div {
                    class: "text-center mb-16",
                    h2 {
                        class: "text-4xl font-bold mb-4 text-white",
                        "SOLANA VERYSTOCHASTIC"
                    }
                    p {
                        class: "text-gray-400 text-lg max-w-2xl mx-auto",
                        "Documenting the chaos, exploits, and lessons learned from the decentralized finance frontier. A chronicle of what happens when code meets reality."
                    }
                }

                if *loading.get() {
                    rsx! {
                        div {
                            class: "flex justify-center py-12",
                            LoadingSpinner { message: "Loading...".to_string() }
                        }
                    }
                } else {
                    rsx! {
                        // Recent posts
                        div {
                            class: "space-y-8",
                            
                            h2 {
                                class: "text-xl font-bold mb-6 text-white uppercase border-b border-gray-800 pb-2",
                                "LATEST INCIDENTS"
                            }
                            
                            div {
                                class: "space-y-6",
                                for (idx, post) in recent_posts.iter().enumerate() {
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
    })
} 