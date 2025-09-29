use dioxus::prelude::*;
use dioxus_router::prelude::*;

mod components;
mod models;
mod pages;
mod services;
mod utils;

use pages::{
    about::AboutPage, contact::ContactPage, home::HomePage, product_detail::ProductDetailPage,
    products::ProductsPage, privacy::PrivacyPage, terms::TermsPage,
};
use components::cookie_banner::CookieBanner;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    HomePage {},
    #[route("/products")]
    ProductsPage {},
    #[route("/products/:id")]
    ProductDetailPage { id: String },
    #[route("/about")]
    AboutPage {},
    #[route("/contact")]
    ContactPage {},
    #[route("/privacy")]
    PrivacyPage {},
    #[route("/terms")]
    TermsPage {},
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com"
        },
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous"
        },
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800;900&display=swap"
        },
        document::Stylesheet { href: asset!("assets/main.css") },
        document::Link {
            rel: "icon",
            href: asset!("assets/favicon.png"),
            r#type: "image/jpeg"
        },
        document::Title { "Coraza - Official Website" },
        Router::<Route> {},
        CookieBanner {}
    }
}
