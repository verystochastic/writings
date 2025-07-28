use dioxus::prelude::*;
use crate::app::{AppState, Route};

#[derive(Props, PartialEq)]
pub struct AboutProps {
    pub app_state: UseState<AppState>,
}

pub fn About(cx: Scope<AboutProps>) -> Element {
    let handle_back = move |_| {
        let mut new_state = cx.props.app_state.get().clone();
        new_state.current_route = Route::Home;
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
                        
                        // Admin edit button (only show if user is admin)
                        if cx.props.app_state.get().is_admin() {
                            rsx! {
                                button {
                                    class: "text-green-400 hover:text-green-300 text-sm uppercase tracking-wider",
                                    onclick: {
                                        let admin_app_state = cx.props.app_state.clone();
                                        move |_| {
                                            let mut new_state = admin_app_state.get().clone();
                                            new_state.current_route = Route::Admin;
                                            admin_app_state.set(new_state);
                                        }
                                    },
                                    "admin panel"
                                }
                            }
                        }
                    }
                }
            }

            // Main content
            main {
                class: "max-w-4xl mx-auto px-4 py-12",
                
                // About section
                div {
                    class: "space-y-12",
                    
                    // Hero section
                    div {
                        class: "text-center space-y-6",
                        h1 {
                            class: "text-4xl font-bold text-white uppercase",
                            "About verystochastic"
                        }
                        p {
                            class: "text-xl text-gray-400 max-w-3xl mx-auto leading-relaxed",
                            "A comprehensive documentation of disasters, exploits, and hard lessons learned from Solana's decentralized finance ecosystem."
                        }
                    }
                    
                    // Mission section
                    div {
                        class: "border-t border-gray-800 pt-12",
                        h2 {
                            class: "text-2xl font-bold text-white mb-6 uppercase",
                            "Our Mission"
                        }
                        div {
                            class: "space-y-6 text-gray-300 leading-relaxed",
                            p {
                                "The DeFi space moves fast, and with that speed comes risk. Every exploit, every hack, every \"rug pull\" teaches us something valuable about the vulnerabilities in our systems."
                            }
                            p {
                                "verystochastic exists to document these incidents in detail, not to spread fear, but to educate. We believe that by understanding what went wrong, we can build better, more secure protocols for the future."
                            }
                            p {
                                "Our focus is on Solana's ecosystem - one of the most innovative and rapidly growing blockchain platforms. But the lessons learned here apply across all of DeFi."
                            }
                        }
                    }
                    
                    // What we cover section
                    div {
                        class: "border-t border-gray-800 pt-12",
                        h2 {
                            class: "text-2xl font-bold text-white mb-6 uppercase",
                            "What We Cover"
                        }
                        div {
                            class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                            
                            div {
                                class: "space-y-4",
                                h3 {
                                    class: "text-lg font-semibold text-green-400 uppercase",
                                    "Protocol Exploits"
                                }
                                p {
                                    class: "text-gray-400",
                                    "Detailed analysis of smart contract vulnerabilities, oracle manipulations, and protocol-level attacks that led to significant losses."
                                }
                            }
                            
                            div {
                                class: "space-y-4",
                                h3 {
                                    class: "text-lg font-semibold text-green-400 uppercase",
                                    "Network Issues"
                                }
                                p {
                                    class: "text-gray-400",
                                    "Documentation of network outages, validator problems, and infrastructure failures that affected users and protocols."
                                }
                            }
                            
                            div {
                                class: "space-y-4",
                                h3 {
                                    class: "text-lg font-semibold text-green-400 uppercase",
                                    "Bridge Failures"
                                }
                                p {
                                    class: "text-gray-400",
                                    "Analysis of cross-chain bridge exploits and the unique challenges of maintaining security across multiple blockchains."
                                }
                            }
                            
                            div {
                                class: "space-y-4",
                                h3 {
                                    class: "text-lg font-semibold text-green-400 uppercase",
                                    "Governance Attacks"
                                }
                                p {
                                    class: "text-gray-400",
                                    "Examination of governance token manipulation, flash loan attacks on voting systems, and protocol takeovers."
                                }
                            }
                        }
                    }
                    
                    // Philosophy section
                    div {
                        class: "border-t border-gray-800 pt-12",
                        h2 {
                            class: "text-2xl font-bold text-white mb-6 uppercase",
                            "Our Philosophy"
                        }
                        div {
                            class: "space-y-6 text-gray-300 leading-relaxed",
                            p {
                                "We approach each incident with technical rigor and without judgment. Builders in this space are pushing the boundaries of what's possible with programmable money, and failures are an inevitable part of that process."
                            }
                            p {
                                "Our goal is not to discourage innovation, but to ensure that the lessons learned from each incident are preserved and shared. Every exploit documented here represents knowledge that can help prevent similar issues in the future."
                            }
                            p {
                                "We believe in the future of decentralized finance, but we also believe that future depends on our ability to learn from our mistakes."
                            }
                        }
                    }
                    
                    // Contact section
                    div {
                        class: "border-t border-gray-800 pt-12",
                        h2 {
                            class: "text-2xl font-bold text-white mb-6 uppercase",
                            "Get Involved"
                        }
                        div {
                            class: "space-y-6 text-gray-300 leading-relaxed",
                            p {
                                "Have information about an incident we haven't covered? Want to contribute analysis or corrections to existing posts? We welcome contributions from the community."
                            }
                            p {
                                "This is an open-source project built on Solana, and we believe in the power of community collaboration to build something valuable for everyone in the ecosystem."
                            }
                            
                            div {
                                class: "flex flex-wrap gap-4 mt-8",
                                a {
                                    class: "border border-gray-700 text-white px-6 py-3 text-sm hover:bg-gray-900 transition-colors uppercase tracking-wider",
                                    href: "https://github.com/verystochastic/solana-blog",
                                    target: "_blank",
                                    "View on GitHub"
                                }
                                a {
                                    class: "border border-gray-700 text-white px-6 py-3 text-sm hover:bg-gray-900 transition-colors uppercase tracking-wider",
                                    href: "https://twitter.com/verystochastic",
                                    target: "_blank",
                                    "Follow on Twitter"
                                }
                            }
                        }
                    }
                }
            }
        }
    })
} 