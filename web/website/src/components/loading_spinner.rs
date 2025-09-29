use dioxus::prelude::*;

#[component]
pub fn LoadingSpinner(message: Option<String>) -> Element {
    let display_message = message.unwrap_or_else(|| "Loading...".to_string());
    
    rsx! {
        div {
            class: "loading-state",
            div { class: "loading-spinner" }
            p { "{display_message}" }
        }
    }
}