use dioxus::prelude::*;
use dioxus_router::prelude::*;
mod components;
mod models;
mod pages;
mod services;
mod state;
mod utils;

use pages::{
    admin::AdminPage,
    dashboard::DashboardPage,
    login::LoginPage,
    products::ProductsPage,
    profile::ProfilePage,
    register::RegisterPage,
    logs::LogsPage,
    users::UsersPage,
};
use state::AppState;

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    LoginPage {},
    #[route("/register")]
    RegisterPage {},
    #[route("/dashboard")]
    DashboardPage {},
    #[route("/users")]
    UsersPage {},
    #[route("/profile")]
    ProfilePage {},
    #[route("/products")]
    ProductsPage {},
    #[route("/admin/logs")]
    LogsPage {},
    #[route("/admin")]
    AdminPage {},
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    launch(app);
}

fn app() -> Element {
    use_context_provider(|| Signal::new(AppState::new()));

    rsx! {
        document::Stylesheet { href: asset!("assets/main.css") },
        document::Stylesheet { href: asset!("assets/products.css") },
        document::Stylesheet { href: asset!("assets/admin.css") },
        document::Stylesheet { href: asset!("assets/dashboard.css") },
        Router::<Route> {}
    }
}
