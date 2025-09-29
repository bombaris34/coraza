use crate::{components::common::Header, state::AppState, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[derive(Clone)]
struct Server {
    name: String,
    address: String,
    port: u16,
    protocol: String,
    status: String,
}

#[component]
pub fn ServersPage() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let (current_user_opt, token_opt) = {
        let state_read = state.read();
        (state_read.current_user.clone(), state_read.token.clone())
    };

    let user_missing = current_user_opt.is_none();
    let token_missing = token_opt.is_none();

    let nav = navigator.clone();
    use_effect(move || {
        if user_missing || token_missing {
            nav.push(Route::LoginPage {});
        }
    });

    if user_missing || token_missing {
        return rsx!(div { "Redirecting..." });
    }

    let current_user = current_user_opt.unwrap();
    let _token = token_opt.unwrap();

    // Placeholder data
    let servers: Vec<Server> = vec![];

    rsx! {
        div {
            class: "main-content",
            Header { current_user: current_user.clone() }
            main {
                class: "page-content",
                h3 { class: "page-title", "Servers" }
                if servers.is_empty() {
                    p { "No servers added yet." }
                } else {
                    ul {
                        for server in servers {
                            li { "{server.name} - {server.status}" }
                        }
                    }
                }
            }
        }
    }
}
