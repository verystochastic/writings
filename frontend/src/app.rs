use dioxus::prelude::*;
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use solana_blog_client::{BlogInfo, PostInfo};
#[cfg(target_arch = "wasm32")]
use crate::services::{BlogInfo, PostInfo, BlogService, WalletService};
#[cfg(not(target_arch = "wasm32"))]
use crate::services::{BlogService, WalletService};
use crate::pages::{Home, About, BlogView, CreatePost, PostView, Admin};

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Home,
    About,
    Blog(String), // blog_pubkey
    CreatePost(String), // blog_pubkey
    Post(String, String), // blog_pubkey, post_pubkey
    Admin,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub current_route: Route,
    pub blog_service: BlogService,
    pub wallet_service: WalletService,
    pub blogs: HashMap<String, BlogInfo>,
    pub posts: HashMap<String, Vec<PostInfo>>,
    pub loading: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_route: Route::Home,
            blog_service: BlogService::new(),
            wallet_service: WalletService::new(),
            blogs: HashMap::new(),
            posts: HashMap::new(),
            loading: false,
        }
    }

    pub fn is_admin(&self) -> bool {
        self.wallet_service.is_admin()
    }

    pub fn is_blog_admin(&self, blog_pubkey: &str) -> bool {
        if let Some(connected_pubkey) = &self.wallet_service.public_key {
            if let Some(blog) = self.blogs.get(blog_pubkey) {
                return blog.authority == *connected_pubkey && self.wallet_service.connected;
            }
        }
        false
    }
}

pub fn App(cx: Scope) -> Element {
    let app_state = use_state(cx, AppState::new);

    cx.render(rsx! {
        style { 
            "
            @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;600;700&family=IBM+Plex+Mono:wght@300;400;500;600;700&display=swap');
            body {{ 
                font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace; 
                margin: 0; 
                padding: 0; 
                background-color: #000000; 
                color: #ffffff; 
            }}
            "
        }
        
        div {
            class: "min-h-screen bg-black text-white",
            style: "font-family: 'JetBrains Mono', 'IBM Plex Mono', Consolas, monospace;",
            
            match &app_state.current_route {
                Route::Home => rsx! {
                    Home {
                        app_state: app_state.clone()
                    }
                },
                Route::About => rsx! {
                    About {
                        app_state: app_state.clone()
                    }
                },
                Route::Blog(blog_pubkey) => rsx! {
                    BlogView {
                        app_state: app_state.clone(),
                        blog_pubkey: blog_pubkey.clone()
                    }
                },
                Route::CreatePost(blog_pubkey) => rsx! {
                    CreatePost {
                        app_state: app_state.clone(),
                        blog_pubkey: blog_pubkey.clone()
                    }
                },
                Route::Post(blog_pubkey, post_pubkey) => rsx! {
                    PostView {
                        app_state: app_state.clone(),
                        blog_pubkey: blog_pubkey.clone(),
                        post_pubkey: post_pubkey.clone()
                    }
                },
                Route::Admin => rsx! {
                    Admin {
                        app_state: app_state.clone()
                    }
                }
            }

            // Loading overlay
            if app_state.loading {
                rsx! {
                    div {
                        class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                        div {
                            class: "animate-spin rounded-full h-12 w-12 border-b-2 border-white"
                        }
                    }
                }
            }
        }
    })
} 