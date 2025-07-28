use dioxus::prelude::*;
use crate::app::{AppState, Route};

#[derive(Props)]
pub struct AdminProps {
    pub app_state: UseState<AppState>,
}

impl PartialEq for AdminProps {
    fn eq(&self, other: &Self) -> bool {
        self.app_state.get().current_route == other.app_state.get().current_route
    }
}

pub fn Admin(cx: Scope<AdminProps>) -> Element {
    let connecting = use_state(cx, || false);
    let error_msg = use_state(cx, || None::<String>);
    let success_msg = use_state(cx, || None::<String>);
    let is_logged_in = cx.props.app_state.get().is_admin();
    let wallet_connected = cx.props.app_state.get().wallet_service.connected;
    let connected_wallet = cx.props.app_state.get().wallet_service.public_key.clone();

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

    let handle_edit_about = {
        let app_state = cx.props.app_state.clone();
        move |_| {
            let mut new_state = app_state.get().clone();
            new_state.current_route = Route::About; // For now, redirect to About page
            app_state.set(new_state);
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
                    class: "max-w-4xl mx-auto px-4 py-8",
                    
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
                                "Welcome to the admin panel. Manage your blog content and settings."
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
                        
                        // Admin actions
                        div {
                            class: "grid md:grid-cols-2 gap-6",
                            
                            // Content management
                            div {
                                class: "border border-gray-700 p-6",
                                h2 {
                                    class: "text-xl font-bold mb-4 text-white",
                                    "CONTENT MANAGEMENT"
                                }
                                div {
                                    class: "space-y-4",
                                    button {
                                        class: "w-full border border-gray-600 text-white py-3 text-sm hover:bg-gray-800 transition-colors",
                                        onclick: handle_create_post,
                                        "📝 CREATE NEW POST"
                                    }
                                    button {
                                        class: "w-full border border-gray-600 text-gray-400 py-3 text-sm hover:bg-gray-800 transition-colors",
                                        onclick: handle_edit_about,
                                        "✏️ EDIT ABOUT PAGE"
                                    }
                                }
                            }
                            
                            // Blog settings
                            div {
                                class: "border border-gray-700 p-6",
                                h2 {
                                    class: "text-xl font-bold mb-4 text-white",
                                    "BLOG SETTINGS"
                                }
                                div {
                                    class: "space-y-4",
                                    button {
                                        class: "w-full border border-gray-600 text-gray-400 py-3 text-sm hover:bg-gray-800 transition-colors",
                                        disabled: true,
                                        "⚙️ BLOG CONFIGURATION"
                                    }
                                    button {
                                        class: "w-full border border-gray-600 text-gray-400 py-3 text-sm hover:bg-gray-800 transition-colors",
                                        disabled: true,
                                        "📊 ANALYTICS"
                                    }
                                }
                            }
                        }
                        
                        // Status section
                        div {
                            class: "border border-gray-700 p-6",
                            h2 {
                                class: "text-xl font-bold mb-4 text-white",
                                "STATUS"
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
                                    div { class: "text-gray-400", "Network" }
                                    div { class: "text-blue-400 font-mono", "🌐 Devnet" }
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