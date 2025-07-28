use dioxus::prelude::*;
use crate::app::{AppState, Route};
use crate::components::LoadingSpinner;
use crate::services::{ArweaveService, PostContent};

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
    
    // Form state
    let title = use_state(cx, || String::new());
    let description = use_state(cx, || String::new());
    let content = use_state(cx, || String::new());
    let tags = use_state(cx, || String::new());
    let show_preview = use_state(cx, || false);
    
    // Arweave service
    let arweave_service = ArweaveService::new();
    
    // Get wallet state
    let wallet_connected = cx.props.app_state.get().wallet_service.connected;
    let wallet_public_key = cx.props.app_state.get().wallet_service.public_key.clone();

    let handle_back = {
        let app_state = cx.props.app_state.clone();
        let blog_pubkey = blog_pubkey.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::Admin;
            app_state.set(new_state);
        }
    };

    let handle_save_draft = {
        let title = title.clone();
        let description = description.clone();
        let content = content.clone();
        let tags = tags.clone();
        let error = error.clone();
        let success = success.clone();
        
        move |_| {
            // Save to localStorage for now
            #[cfg(target_arch = "wasm32")]
            {
                if let Some(window) = web_sys::window() {
                    if let Ok(Some(local_storage)) = window.local_storage() {
                        let draft = serde_json::json!({
                            "title": title.get(),
                            "description": description.get(),
                            "content": content.get(),
                            "tags": tags.get(),
                            "timestamp": chrono::Utc::now().timestamp()
                        });
                        
                        if let Ok(draft_str) = serde_json::to_string(&draft) {
                            let _ = local_storage.set_item("blog_draft", &draft_str);
                        }
                    }
                }
            }
            
            success.set(true);
        }
    };

    let handle_publish = {
        let title = title.clone();
        let description = description.clone();
        let content = content.clone();
        let tags = tags.clone();
        let loading = loading.clone();
        let error = error.clone();
        let success = success.clone();
        let arweave_service = arweave_service;
        let wallet_public_key = wallet_public_key.clone();
        
        move |_| {
            // Check if wallet is connected
            if !wallet_connected {
                error.set(Some("Wallet not connected. Please connect your wallet first.".to_string()));
                return;
            }
            
            if title.get().is_empty() || content.get().is_empty() {
                error.set(Some("Title and content are required".to_string()));
                return;
            }
            
            error.set(None);
            loading.set(true);
            
            cx.spawn({
                let loading = loading.clone();
                let success = success.clone();
                let error = error.clone();
                let arweave_service = arweave_service.clone();
                let title = title.get().clone();
                let description = description.get().clone();
                let content = content.get().clone();
                let tags_str = tags.get().clone();
                let wallet_public_key = wallet_public_key.clone().unwrap_or_default();
                
                async move {
                    // Parse tags
                    let tags: Vec<String> = tags_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    
                    // Create post content
                    let post = PostContent {
                        title: title.clone(),
                        description: description.clone(),
                        content: content.clone(),
                        tags,
                        created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        author: "verystochastic".to_string(),
                    };
                    
                    // Check wallet balance first
                    match arweave_service.check_wallet_balance(&wallet_public_key).await {
                        Ok(has_balance) => {
                            if !has_balance {
                                loading.set(false);
                                error.set(Some("Insufficient wallet balance for Arweave upload".to_string()));
                                return;
                            }
                        }
                        Err(err) => {
                            loading.set(false);
                            error.set(Some(format!("Failed to check wallet balance: {}", err)));
                            return;
                        }
                    }
                    
                    // Get upload cost estimate
                    let content_size = content.len();
                    match arweave_service.get_upload_cost(content_size).await {
                        Ok(cost) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("💰 Estimated upload cost: ${:.2}", cost).into());
                        }
                        Err(err) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("⚠️ Could not estimate cost: {}", err).into());
                        }
                    }
                    
                    // Upload to Arweave with wallet authentication
                    match arweave_service.upload_post(post, &wallet_public_key).await {
                        Ok(tx_id) => {
                            // Clear draft from localStorage
                            #[cfg(target_arch = "wasm32")]
                            {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(local_storage)) = window.local_storage() {
                                        let _ = local_storage.remove_item("blog_draft");
                                    }
                                }
                            }
                            
                            loading.set(false);
                            success.set(true);
                        }
                        Err(err) => {
                            loading.set(false);
                            error.set(Some(format!("Failed to publish post: {}", err)));
                        }
                    }
                }
            });
        }
    };

    // Show wallet connection required message if not connected
    if !wallet_connected {
        return cx.render(rsx! {
            div {
                class: "min-h-screen bg-black text-white",
                style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
                div {
                    class: "max-w-2xl mx-auto px-4 py-24 text-center",
                    div {
                        class: "mb-8",
                        div {
                            class: "text-4xl mb-6 text-red-400",
                            "🔐 WALLET REQUIRED"
                        }
                        h2 {
                            class: "text-2xl font-light mb-4 tracking-wide",
                            "CONNECT YOUR WALLET TO PUBLISH"
                        }
                        p {
                            class: "text-gray-400 mb-8",
                            "Publishing to Arweave requires wallet authentication and a small fee in AR tokens."
                        }
                        div {
                            class: "space-x-4",
                            button {
                                class: "border border-gray-700 text-white px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                onclick: handle_back,
                                "← back to admin"
                            }
                        }
                    }
                }
            }
        });
    }

    if *loading.get() {
        return cx.render(rsx! {
            div {
                class: "min-h-screen bg-black text-white flex items-center justify-center",
                style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
                LoadingSpinner {
                    message: "Publishing to Arweave...".to_string()
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
                            "POST PUBLISHED TO ARWEAVE"
                        }
                        p {
                            class: "text-gray-400 mb-8",
                            "Your post has been permanently stored on Arweave. It may take a few moments to propagate across the network."
                        }
                        div {
                            class: "space-x-4",
                            button {
                                class: "border border-gray-700 text-white px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                onclick: handle_back,
                                "← back to admin"
                            }
                            button {
                                class: "border border-gray-700 text-white px-6 py-2 text-sm hover:bg-gray-900 transition-colors",
                                onclick: move |_| {
                                    success.set(false);
                                    title.set(String::new());
                                    description.set(String::new());
                                    content.set(String::new());
                                    tags.set(String::new());
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
                class: "border-b border-gray-800 bg-black",
                div {
                    class: "max-w-6xl mx-auto px-4 py-4 flex items-center justify-between",
                    
                    button {
                        class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider",
                        onclick: handle_back,
                        "← back to admin"
                    }
                    
                    div {
                        class: "flex items-center space-x-4",
                        div {
                            class: "text-sm text-gray-400",
                            "Connected: {crate::utils::truncate_pubkey(wallet_public_key.as_ref().unwrap_or(&\"Unknown\".to_string()))}"
                        }
                        div {
                            class: "text-xs text-green-400 bg-green-900/20 px-2 py-1 rounded",
                            "✅ WALLET CONNECTED"
                        }
                    }
                }
            }
            
            // Main content
            main {
                class: "max-w-4xl mx-auto px-4 py-8",
                
                div {
                    class: "space-y-8",
                    
                    // Title section
                    div {
                        class: "border-b border-gray-800 pb-8",
                        h1 {
                            class: "text-3xl font-bold mb-4 text-white",
                            "CREATE NEW POST"
                        }
                        p {
                            class: "text-gray-400 text-lg",
                            "Write and publish your post to Arweave. This requires wallet authentication and a small fee."
                        }
                    }
                    
                    // Error message
                    if let Some(error_msg) = error.get() {
                        rsx! {
                            div {
                                class: "border border-red-800 bg-red-900/20 text-red-400 p-4 text-sm rounded",
                                "❌ {error_msg}"
                            }
                        }
                    }
                    
                    // Form
                    div {
                        class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                        
                        // Left column - form fields
                        div {
                            class: "lg:col-span-2 space-y-6",
                            
                            // Title
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-300 mb-2",
                                    "TITLE"
                                }
                                input {
                                    class: "w-full bg-gray-900 border border-gray-700 text-white px-4 py-3 text-sm focus:border-blue-500 focus:outline-none",
                                    placeholder: "Enter post title...",
                                    value: "{title.get()}",
                                    oninput: move |e| title.set(e.value.clone())
                                }
                            }
                            
                            // Description
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-300 mb-2",
                                    "DESCRIPTION"
                                }
                                input {
                                    class: "w-full bg-gray-900 border border-gray-700 text-white px-4 py-3 text-sm focus:border-blue-500 focus:outline-none",
                                    placeholder: "Brief description of your post...",
                                    value: "{description.get()}",
                                    oninput: move |e| description.set(e.value.clone())
                                }
                            }
                            
                            // Tags
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-300 mb-2",
                                    "TAGS (comma-separated)"
                                }
                                input {
                                    class: "w-full bg-gray-900 border border-gray-700 text-white px-4 py-3 text-sm focus:border-blue-500 focus:outline-none",
                                    placeholder: "rust, blockchain, solana...",
                                    value: "{tags.get()}",
                                    oninput: move |e| tags.set(e.value.clone())
                                }
                            }
                            
                            // Content
                            div {
                                label {
                                    class: "block text-sm font-medium text-gray-300 mb-2",
                                    "CONTENT (Markdown)"
                                }
                                textarea {
                                    class: "w-full bg-gray-900 border border-gray-700 text-white px-4 py-3 text-sm focus:border-blue-500 focus:outline-none font-mono",
                                    rows: "20",
                                    placeholder: "# Your Post Title\n\nWrite your content in Markdown...",
                                    value: "{content.get()}",
                                    oninput: move |e| content.set(e.value.clone())
                                }
                            }
                        }
                        
                        // Right column - preview and actions
                        div {
                            class: "space-y-6",
                            
                            // Preview toggle
                            div {
                                class: "border border-gray-700 p-4",
                                div {
                                    class: "flex items-center justify-between mb-4",
                                    h3 {
                                        class: "text-lg font-medium text-white",
                                        "PREVIEW"
                                    }
                                    button {
                                        class: "text-sm text-gray-400 hover:text-white",
                                        onclick: move |_| show_preview.set(!show_preview.get()),
                                        if *show_preview.get() { "Hide" } else { "Show" }
                                    }
                                }
                                
                                if *show_preview.get() {
                                    rsx! {
                                        div {
                                            class: "prose prose-invert max-w-none",
                                            dangerous_inner_html: "{crate::utils::markdown_to_html(&content.get())}"
                                        }
                                    }
                                }
                            }
                            
                            // Actions
                            div {
                                class: "border border-gray-700 p-4 space-y-4",
                                h3 {
                                    class: "text-lg font-medium text-white mb-4",
                                    "ACTIONS"
                                }
                                
                                button {
                                    class: "w-full border border-gray-600 text-white py-3 text-sm hover:bg-gray-800 transition-colors",
                                    onclick: handle_save_draft,
                                    "💾 Save Draft"
                                }
                                
                                button {
                                    class: "w-full border border-green-600 text-green-400 py-3 text-sm hover:bg-green-900/20 transition-colors",
                                    onclick: handle_publish,
                                    "🚀 Publish to Arweave"
                                }
                                
                                div {
                                    class: "text-xs text-gray-500 mt-4 p-3 bg-gray-900/50 rounded",
                                    div { class: "font-medium mb-2", "ℹ️ Publishing Info:" }
                                    div { "• Requires wallet authentication" }
                                    div { "• Small fee in AR tokens (~$0.50)" }
                                    div { "• Permanent storage on Arweave" }
                                    div { "• Transaction may take 2-3 seconds" }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
} 