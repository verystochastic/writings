mod app;
mod components;
mod config;
mod pages;
mod services;
mod utils;

fn main() {
    #[cfg(feature = "web")]
    {
        // Launch in web mode
        dioxus_web::launch(app::App);
    }

    #[cfg(feature = "desktop")]
    {
        // Launch in desktop mode
        dioxus_desktop::launch(app::App);
    }
} 