use crate::{state::AppState, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[derive(Props, PartialEq, Clone)]
pub struct DeleteAccountModalProps {
    show: Signal<bool>,
}

#[component]
pub fn DeleteAccountModal(props: DeleteAccountModalProps) -> Element {
    let mut show = props.show;
    let mut state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();
    let confirmation = use_signal(String::new);
    let mut error = use_signal(|| None::<String>);

    let _handle_delete = move |_: Event<FormData>| {
        if confirmation() == "DELETE" {
            let mut state_write = state.write();
            let user_id = state_write.current_user.as_ref().map(|u| u.id);
            if let Some(id) = user_id {
                state_write.users.remove(&id);
                state_write.current_user = None;
                navigator.push(Route::LoginPage {});
                show.set(false);
            }
        } else {
            error.set(Some("Please type DELETE to confirm".to_string()));
        }
    };

    // TODO: Implement actual modal UI
    rsx! {
        div {
            class: "modal-overlay",
            div {
                class: "modal-content",
                h3 { "Delete Account" }
                p { "Type DELETE to confirm account deletion" }
                // Add form fields here
                if let Some(error_msg) = error() {
                    p { class: "error", "{error_msg}" }
                }
                button {
                    class: "btn btn-secondary",
                    onclick: move |_| show.set(false),
                    "Cancel"
                }
            }
        }
    }
}
