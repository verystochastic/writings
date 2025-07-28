// Utility functions for the frontend

pub fn format_date(timestamp: i64) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Date;
        let date = Date::new(&(timestamp as f64 * 1000.0).into());
        date.to_date_string().as_string().unwrap_or_default()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use chrono::{DateTime, Utc};
        let dt = DateTime::<Utc>::from_timestamp(timestamp, 0).unwrap_or_default();
        dt.format("%Y-%m-%d").to_string()
    }
}

pub fn format_timestamp(timestamp: i64) -> String {
    #[cfg(target_arch = "wasm32")]
    {
        use js_sys::Date;
        let date = Date::new(&(timestamp as f64 * 1000.0).into());
        date.to_date_string().as_string().unwrap_or_else(|| "Unknown date".to_string())
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        use chrono::{DateTime, Utc};
        DateTime::<Utc>::from_timestamp(timestamp, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "Unknown date".to_string())
    }
}

pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length])
    }
}

pub fn truncate_pubkey(pubkey: &str) -> String {
    if pubkey.len() > 12 {
        format!("{}...{}", &pubkey[..6], &pubkey[pubkey.len()-6..])
    } else {
        pubkey.to_string()
    }
}

pub fn validate_solana_pubkey(pubkey: &str) -> bool {
    pubkey.len() == 44 && pubkey.chars().all(|c| c.is_alphanumeric())
}

/// Convert markdown to HTML for preview
pub fn markdown_to_html(markdown: &str) -> String {
    // Simple markdown to HTML conversion
    // In a real implementation, you'd use a proper markdown parser
    let mut html = markdown.to_string();
    
    // Headers
    html = html.replace("# ", "<h1>").replace("\n# ", "</h1>\n<h1>");
    html = html.replace("## ", "<h2>").replace("\n## ", "</h2>\n<h2>");
    html = html.replace("### ", "<h3>").replace("\n### ", "</h3>\n<h3>");
    
    // Bold
    html = html.replace("**", "<strong>").replace("**", "</strong>");
    html = html.replace("__", "<strong>").replace("__", "</strong>");
    
    // Italic
    html = html.replace("*", "<em>").replace("*", "</em>");
    html = html.replace("_", "<em>").replace("_", "</em>");
    
    // Code blocks
    html = html.replace("```", "<pre><code>").replace("```", "</code></pre>");
    html = html.replace("`", "<code>").replace("`", "</code>");
    
    // Links
    // Simple link replacement (basic)
    html = html.replace("[", "<a href=\"").replace("](", "\">").replace(")", "</a>");
    
    // Line breaks
    html = html.replace("\n\n", "</p>\n<p>");
    html = html.replace("\n", "<br>");
    
    // Wrap in paragraphs if not already wrapped
    if !html.starts_with("<h") && !html.starts_with("<p") && !html.starts_with("<pre") {
        html = format!("<p>{}</p>", html);
    }
    
    html
}

#[cfg(target_arch = "wasm32")]
pub fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(message: &str) {
    println!("{}", message);
} 