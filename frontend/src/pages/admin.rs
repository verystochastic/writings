use dioxus::prelude::*;
use crate::app::{AppState, Route};
use wasm_bindgen::JsCast;

#[derive(Props)]
pub struct AdminProps {
    pub app_state: UseState<AppState>,
}

impl PartialEq for AdminProps {
    fn eq(&self, other: &Self) -> bool {
        let self_state = self.app_state.get();
        let other_state = other.app_state.get();
        
        // Check if wallet connection status changed
        let wallet_changed = self_state.wallet_service.connected != other_state.wallet_service.connected;
        let admin_status_changed = self_state.is_admin() != other_state.is_admin();
        
        // Re-render if wallet status or admin status changed
        !wallet_changed && !admin_status_changed && 
        self_state.current_route == other_state.current_route
    }
}

pub fn Admin(cx: Scope<AdminProps>) -> Element {
    let connecting = use_state(cx, || false);
    let error_msg = use_state(cx, || None::<String>);
    let success_msg = use_state(cx, || None::<String>);
    let is_logged_in = cx.props.app_state.get().is_admin();
    let wallet_connected = cx.props.app_state.get().wallet_service.connected;
    let connected_wallet = cx.props.app_state.get().wallet_service.public_key.clone();

    // Blog posts state - now mutable
    let blog_posts = use_state(cx, || vec![
        BlogPost {
            id: "1".to_string(),
            title: "Welcome to My Solana Blog".to_string(),
            description: "My first post on the decentralized web".to_string(),
            created_at: "2024-01-20".to_string(),
            status: PostStatus::Published,
            arweave_tx: "abc123...".to_string(),
        },
        BlogPost {
            id: "2".to_string(),
            title: "Building with Rust and Dioxus".to_string(),
            description: "How I built this blog using Rust".to_string(),
            created_at: "2024-01-15".to_string(),
            status: PostStatus::Draft,
            arweave_tx: "".to_string(),
        },
    ]);

    // Arweave wallet configuration
    let arweave_service = crate::services::ArweaveService::new();
    let arweave_wallet_configured = arweave_service.get_arweave_key().is_some();
    let arweave_address = arweave_service.get_arweave_address();

    let handle_connect_wallet = {
        let app_state = cx.props.app_state.clone();
        let connecting = connecting.clone();
        let error_msg = error_msg.clone();
        let success_msg = success_msg.clone();
        
        move |_| {
            connecting.set(true);
            error_msg.set(None);
            success_msg.set(None);
            
            cx.spawn({
                let app_state = app_state.clone();
                let connecting = connecting.clone();
                let error_msg = error_msg.clone();
                let success_msg = success_msg.clone();
                
                async move {
                    let mut state = app_state.get().clone();
                    match state.wallet_service.connect_phantom().await {
                        Ok(public_key) => {
                            // Update the app state with the new wallet connection
                            app_state.set(state);
                            success_msg.set(Some(format!("Wallet connected successfully! Address: {}", 
                                crate::utils::truncate_pubkey(&public_key))));
                            
                            // Log for debugging
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("✅ Wallet connected: {}", public_key).into());
                        }
                        Err(err) => {
                            error_msg.set(Some(err));
                        }
                    }
                    connecting.set(false);
                }
            });
        }
    };

    let handle_disconnect = {
        let app_state = cx.props.app_state.clone();
        let success_msg = success_msg.clone();
        move |_| {
            let mut state = app_state.get().clone();
            state.wallet_service.disconnect();
            state.current_route = Route::Home;
            app_state.set(state);
            success_msg.set(Some("Wallet disconnected successfully".to_string()));
        }
    };

    let handle_create_post = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::CreatePost(crate::config::DEMO_BLOG_PUBKEY.to_string());
            app_state.set(new_state);
        }
    };

    let handle_back_home = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::Home;
            app_state.set(new_state);
        }
    };

    // Arweave wallet configuration handlers
    let handle_configure_arweave = {
        let arweave_service = arweave_service.clone();
        let success_msg = success_msg.clone();
        let error_msg = error_msg.clone();
        
        move |key: String| {
            if key.is_empty() {
                error_msg.set(Some("Please enter your Arweave wallet key".to_string()));
                return;
            }
            
            match arweave_service.store_arweave_key(&key) {
                Ok(_) => {
                    success_msg.set(Some("Arweave wallet configured successfully!".to_string()));
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"✅ Arweave wallet stored securely".into());
                }
                Err(err) => {
                    error_msg.set(Some(format!("Failed to store Arweave wallet: {}", err)));
                }
            }
        }
    };

    let handle_clear_arweave = {
        let arweave_service = arweave_service.clone();
        let success_msg = success_msg.clone();
        
        move |_| {
            match arweave_service.clear_arweave_key() {
                Ok(_) => {
                    success_msg.set(Some("Arweave wallet cleared successfully".to_string()));
                }
                Err(err) => {
                    error_msg.set(Some(format!("Failed to clear Arweave wallet: {}", err)));
                }
            }
        }
    };

    // Debug logging
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&format!("Admin: wallet_connected={}, is_logged_in={}", wallet_connected, is_logged_in).into());
    }

    // If wallet is connected and user is admin, show admin panel
    if wallet_connected && is_logged_in {
        cx.render(rsx! {
            div {
                class: "min-h-screen bg-black text-white",
                
                // Header
                header {
                    class: "border-b border-gray-800 bg-black",
                    div {
                        class: "max-w-6xl mx-auto px-4 py-4 flex items-center justify-between",
                        
                        button {
                            class: "text-gray-300 hover:text-white text-sm uppercase tracking-wider",
                            onclick: handle_back_home,
                            "← verystochastic"
                        }
                        
                        div {
                            class: "flex items-center space-x-4",
                            div {
                                class: "text-sm text-gray-400",
                                "Connected: {crate::utils::truncate_pubkey(connected_wallet.as_ref().unwrap_or(&\"Unknown\".to_string()))}"
                            }
                            button {
                                class: "text-red-400 hover:text-red-300 text-sm uppercase tracking-wider",
                                onclick: handle_disconnect,
                                "disconnect"
                            }
                        }
                    }
                }
                
                // Admin panel content
                main {
                    class: "max-w-6xl mx-auto px-4 py-8",
                    
                    div {
                        class: "space-y-8",
                        
                        // Welcome section
                        div {
                            class: "border-b border-gray-800 pb-8",
                            h1 {
                                class: "text-3xl font-bold mb-4 text-white",
                                "ADMIN DASHBOARD"
                            }
                            p {
                                class: "text-gray-400 text-lg",
                                "Manage your decentralized blog content and settings."
                            }
                        }
                        
                        // Success message
                        if let Some(success) = success_msg.get() {
                            rsx! {
                                div {
                                    class: "border border-green-800 bg-green-900/20 text-green-400 p-4 text-sm rounded",
                                    "✅ {success}"
                                }
                            }
                        }

                        // Error message
                        if let Some(error) = error_msg.get() {
                            rsx! {
                                div {
                                    class: "border border-red-800 bg-red-900/20 text-red-400 p-4 text-sm rounded",
                                    "❌ {error}"
                                }
                            }
                        }
                        
                        // Blog Stats
                        div {
                            class: "grid md:grid-cols-4 gap-4",
                            div {
                                class: "border border-gray-700 p-4",
                                div { class: "text-gray-400 text-sm", "Total Posts" }
                                div { class: "text-2xl font-bold text-white", "{blog_posts.get().len()}" }
                            }
                            div {
                                class: "border border-gray-700 p-4",
                                div { class: "text-gray-400 text-sm", "Published" }
                                div { class: "text-2xl font-bold text-green-400", "{blog_posts.get().iter().filter(|p| p.status == PostStatus::Published).count()}" }
                            }
                            div {
                                class: "border border-gray-700 p-4",
                                div { class: "text-gray-400 text-sm", "Drafts" }
                                div { class: "text-2xl font-bold text-yellow-400", "{blog_posts.get().iter().filter(|p| p.status == PostStatus::Draft).count()}" }
                            }
                            div {
                                class: "border border-gray-700 p-4",
                                div { class: "text-gray-400 text-sm", "Last Published" }
                                div { class: "text-sm text-white", "2024-01-20" }
                            }
                        }
                        
                        // Quick Actions
                        div {
                            class: "border border-gray-700 p-6",
                            h2 {
                                class: "text-xl font-bold mb-4 text-white",
                                "QUICK ACTIONS"
                            }
                            div {
                                class: "flex flex-wrap gap-4",
                                button {
                                    class: "border border-gray-600 text-white px-6 py-3 text-sm hover:bg-gray-800 transition-colors flex items-center space-x-2",
                                    onclick: handle_create_post,
                                    span { "📝" }
                                    span { "Create New Post" }
                                }
                                button {
                                    class: "border border-gray-600 text-gray-400 px-6 py-3 text-sm hover:bg-gray-800 transition-colors flex items-center space-x-2",
                                    disabled: true,
                                    span { "📁" }
                                    span { "Upload Markdown" }
                                }
                                button {
                                    class: "border border-gray-600 text-gray-400 px-6 py-3 text-sm hover:bg-gray-800 transition-colors flex items-center space-x-2",
                                    disabled: true,
                                    span { "⚙️" }
                                    span { "Blog Settings" }
                                }
                            }
                        }

                        // Arweave Wallet Configuration
                        div {
                            class: "border border-gray-700 p-6",
                            h2 {
                                class: "text-xl font-bold mb-4 text-white",
                                "ARWEAVE WALLET CONFIGURATION"
                            }
                            
                            if arweave_wallet_configured {
                                rsx! {
                                    div {
                                        class: "space-y-4",
                                        div {
                                            class: "flex items-center space-x-3 p-4 bg-green-900/20 border border-green-700 rounded",
                                            span { class: "text-green-400", "✅" }
                                            div {
                                                class: "flex-1",
                                                div { class: "text-green-400 font-medium", "Arweave Wallet Configured" }
                                                if let Some(address) = arweave_address {
                                                    rsx! {
                                                        div { class: "text-sm text-gray-400", "Address: {crate::utils::truncate_pubkey(&address)}" }
                                                    }
                                                }
                                            }
                                            button {
                                                class: "text-red-400 hover:text-red-300 text-sm",
                                                onclick: handle_clear_arweave,
                                                "Clear"
                                            }
                                        }
                                        div {
                                            class: "text-xs text-gray-500",
                                            "Your Arweave wallet is configured and ready for publishing posts."
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div {
                                        class: "space-y-4",
                                        div {
                                            class: "p-4 bg-yellow-900/20 border border-yellow-700 rounded",
                                            div { class: "text-yellow-400 font-medium mb-2", "⚠️ Arweave Wallet Not Configured" }
                                            div { class: "text-sm text-gray-400 mb-4", "You need to configure your Arweave wallet to publish posts." }
                                            
                                            // Secure key input (for demo purposes - in production, use proper key management)
                                            div {
                                                class: "space-y-2",
                                                label {
                                                    class: "block text-sm font-medium text-gray-300",
                                                    "Arweave Wallet Key (Base64)"
                                                }
                                                input {
                                                    class: "w-full bg-gray-900 border border-gray-700 text-white px-4 py-2 text-sm focus:border-blue-500 focus:outline-none",
                                                    placeholder: "Enter your Arweave wallet key...",
                                                    id: "arweave-key-input"
                                                }
                                                button {
                                                    class: "border border-blue-600 text-blue-400 px-4 py-2 text-sm hover:bg-blue-900/20 transition-colors",
                                                    onclick: move |_| {
                                                        if let Some(input) = web_sys::window()
                                                            .and_then(|w| w.document())
                                                            .and_then(|d| d.get_element_by_id("arweave-key-input"))
                                                            .and_then(|e| e.dyn_into::<web_sys::HtmlInputElement>().ok())
                                                            .map(|e| e.value()) {
                                                            handle_configure_arweave(input);
                                                        }
                                                    },
                                                    "Configure Wallet"
                                                }
                                            }
                                        }
                                        div {
                                            class: "text-xs text-gray-500",
                                            "⚠️ Store your Arweave wallet key securely. This is stored locally in your browser."
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Blog Posts List
                        div {
                            class: "border border-gray-700 p-6",
                            h2 {
                                class: "text-xl font-bold mb-4 text-white",
                                "BLOG POSTS"
                            }
                            div {
                                class: "space-y-4",
                                for post in blog_posts.get().iter() {
                                    rsx! {
                                        div {
                                            class: "border border-gray-700 p-4 hover:bg-gray-900/50 transition-colors",
                                            div {
                                                class: "flex items-center justify-between",
                                                div {
                                                    class: "flex-1",
                                                    div {
                                                        class: "flex items-center space-x-3 mb-2",
                                                        if post.status == PostStatus::Published {
                                                            rsx! { span { class: "text-green-400 text-sm", "✅" } }
                                                        } else {
                                                            rsx! { span { class: "text-yellow-400 text-sm", "⏳" } }
                                                        }
                                                        span { class: "text-gray-400 text-sm", "{post.created_at}" }
                                                        span { class: "text-white font-medium", "{post.title}" }
                                                    }
                                                    p {
                                                        class: "text-gray-400 text-sm",
                                                        "{post.description}"
                                                    }
                                                    if post.status == PostStatus::Published {
                                                        rsx! {
                                                            p {
                                                                class: "text-xs text-gray-500 font-mono",
                                                                "Arweave: {crate::utils::truncate_pubkey(&post.arweave_tx)}"
                                                            }
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "flex space-x-2",
                                                    if post.status == PostStatus::Draft {
                                                        rsx! {
                                                            button {
                                                                class: "border border-green-600 text-green-400 px-3 py-1 text-xs hover:bg-green-900/20 transition-colors",
                                                                onclick: {
                                                                    let blog_posts = blog_posts.clone();
                                                                    let success_msg = success_msg.clone();
                                                                    let post_id = post.id.clone();
                                                                    move |_| {
                                                                        let mut posts = blog_posts.get().clone();
                                                                        if let Some(post) = posts.iter_mut().find(|p| p.id == post_id) {
                                                                            post.status = PostStatus::Published;
                                                                            blog_posts.set(posts);
                                                                            success_msg.set(Some("Post published successfully!".to_string()));
                                                                        }
                                                                    }
                                                                },
                                                                "Publish"
                                                            }
                                                        }
                                                    } else {
                                                        rsx! {
                                                            button {
                                                                class: "border border-gray-600 text-gray-400 px-3 py-1 text-xs hover:bg-gray-800 transition-colors",
                                                                onclick: {
                                                                    let blog_posts = blog_posts.clone();
                                                                    let success_msg = success_msg.clone();
                                                                    let post_id = post.id.clone();
                                                                    move |_| {
                                                                        let mut posts = blog_posts.get().clone();
                                                                        if let Some(post) = posts.iter_mut().find(|p| p.id == post_id) {
                                                                            post.status = PostStatus::Draft;
                                                                            blog_posts.set(posts);
                                                                            success_msg.set(Some("Post hidden successfully!".to_string()));
                                                                        }
                                                                    }
                                                                },
                                                                "Hide"
                                                            }
                                                        }
                                                    }
                                                    button {
                                                        class: "border border-gray-600 text-gray-400 px-3 py-1 text-xs hover:bg-gray-800 transition-colors",
                                                        onclick: {
                                                            let app_state = cx.props.app_state.clone();
                                                            let post_id = post.id.clone();
                                                            move |_| {
                                                                let mut new_state = app_state.get().clone();
                                                                new_state.current_route = Route::CreatePost(crate::config::DEMO_BLOG_PUBKEY.to_string());
                                                                app_state.set(new_state);
                                                            }
                                                        },
                                                        "Edit"
                                                    }
                                                    button {
                                                        class: "border border-red-600 text-red-400 px-3 py-1 text-xs hover:bg-red-900/20 transition-colors",
                                                        onclick: {
                                                            let blog_posts = blog_posts.clone();
                                                            let success_msg = success_msg.clone();
                                                            let post_id = post.id.clone();
                                                            move |_| {
                                                                let mut posts = blog_posts.get().clone();
                                                                posts.retain(|p| p.id != post_id);
                                                                blog_posts.set(posts);
                                                                success_msg.set(Some("Post deleted successfully!".to_string()));
                                                            }
                                                        },
                                                        "Delete"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Status section
                        div {
                            class: "border border-gray-700 p-6",
                            h2 {
                                class: "text-xl font-bold mb-4 text-white",
                                "SYSTEM STATUS"
                            }
                            div {
                                class: "grid grid-cols-1 md:grid-cols-3 gap-4 text-sm",
                                div {
                                    class: "p-3 bg-gray-900 border border-gray-700",
                                    div { class: "text-gray-400", "Wallet Status" }
                                    div { class: "text-green-400 font-mono", "✅ Connected" }
                                }
                                div {
                                    class: "p-3 bg-gray-900 border border-gray-700",
                                    div { class: "text-gray-400", "Admin Access" }
                                    div { class: "text-green-400 font-mono", "✅ Verified" }
                                }
                                div {
                                    class: "p-3 bg-gray-900 border border-gray-700",
                                    div { class: "text-gray-400", "Arweave Wallet" }
                                    if arweave_wallet_configured {
                                        rsx! { div { class: "text-green-400 font-mono", "✅ Configured" } }
                                    } else {
                                        rsx! { div { class: "text-yellow-400 font-mono", "⚠️ Not Configured" } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    } else {
        // Show wallet connection screen
        cx.render(rsx! {
            div {
                class: "min-h-screen bg-black text-white flex items-center justify-center",
                
                div {
                    class: "w-full max-w-md",
                    div {
                        class: "border border-gray-700 p-8",
                        
                        h1 {
                            class: "text-2xl font-bold text-center mb-8 text-white",
                            "ADMIN ACCESS"
                        }
                        
                        p {
                            class: "text-gray-400 text-center mb-8",
                            "Connect your Phantom wallet to access admin features"
                        }
                        
                        div {
                            class: "space-y-6",
                            
                            // Success message
                            if let Some(success) = success_msg.get() {
                                rsx! {
                                    div {
                                        class: "border border-green-800 bg-green-900/20 text-green-400 p-4 text-sm rounded",
                                        "✅ {success}"
                                    }
                                }
                            }
                            
                            // Error message
                            if let Some(error) = error_msg.get() {
                                rsx! {
                                    div {
                                        class: "border border-red-800 bg-red-900/20 text-red-400 p-4 text-sm rounded",
                                        "❌ {error}"
                                    }
                                }
                            }
                            
                            // Connection status for already connected wallets
                            if wallet_connected && !is_logged_in {
                                rsx! {
                                    div {
                                        class: "border border-yellow-800 bg-yellow-900/20 text-yellow-400 p-4 text-sm rounded",
                                        "⚠️ Wallet connected but you don't have admin access. Please use the admin wallet."
                                        br {}
                                        "Connected: {crate::utils::truncate_pubkey(connected_wallet.as_ref().unwrap_or(&\"Unknown\".to_string()))}"
                                    }
                                }
                            }
                            
                            button {
                                class: "w-full border border-gray-700 text-white py-3 text-sm hover:bg-gray-900 transition-colors uppercase tracking-wider flex items-center justify-center space-x-3",
                                disabled: **connecting,
                                onclick: handle_connect_wallet,
                                
                                if **connecting {
                                    rsx! {
                                        div { class: "animate-spin rounded-full h-4 w-4 border-b-2 border-white" }
                                        span { "connecting..." }
                                    }
                                } else if wallet_connected {
                                    rsx! {
                                        span { "🔄" }
                                        span { "try different wallet" }
                                    }
                                } else {
                                    rsx! {
                                        span { "🦋" }
                                        span { "connect phantom wallet" }
                                    }
                                }
                            }
                            
                            // Disconnect button if wallet is connected but not admin
                            if wallet_connected && !is_logged_in {
                                rsx! {
                                    button {
                                        class: "w-full border border-red-700 text-red-400 py-2 text-sm hover:bg-red-900/20 transition-colors uppercase tracking-wider",
                                        onclick: handle_disconnect,
                                        "disconnect wallet"
                                    }
                                }
                            }
                            
                            div {
                                class: "text-xs text-gray-500 text-center space-y-2",
                                p { "Admin access requires a connected Solana wallet" }
                                p { 
                                    "Don't have Phantom? "
                                    a {
                                        href: "https://phantom.app/",
                                        target: "_blank",
                                        class: "text-blue-400 hover:text-blue-300 underline",
                                        "Download here"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

// Blog post data structure
#[derive(Clone, PartialEq)]
struct BlogPost {
    id: String,
    title: String,
    description: String,
    created_at: String,
    status: PostStatus,
    arweave_tx: String,
}

#[derive(Clone, PartialEq)]
enum PostStatus {
    Published,
    Draft,
} 