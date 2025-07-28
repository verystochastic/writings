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

#[cfg(target_arch = "wasm32")]
pub fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}

#[cfg(not(target_arch = "wasm32"))]
pub fn log(message: &str) {
    println!("{}", message);
} 