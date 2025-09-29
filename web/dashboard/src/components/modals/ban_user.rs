use crate::models::UserUpdate;
use crate::services::user_service;
use crate::state::AppState;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn BanUserModal(user_id: Uuid, show: Signal<Option<Uuid>>) -> Element {
    let state = use_context::<Signal<AppState>>();
    let user_resource = use_resource(move || {
        let token = state.read().token.clone();
        async move {
            if let Some(token) = token {
                if let Ok(users) = user_service::get_users(&token).await {
                    users.into_iter().find(|u| u.id == user_id)
                } else {
                    None
                }
            } else {
                None
            }
        }
    });

    let x = match user_resource.read().as_ref() {
        Some(Some(user)) => {
            let mut ban_reason = use_signal(|| String::new());
            let mut is_submitting = use_signal(|| false);

            let handle_ban = {
                let state = state.clone();
                let ban_reason = ban_reason.clone();
                let show = show.clone();
                let mut is_submitting = is_submitting.clone();

                move |_| {
                    let Some(token) = state.read().token.clone() else {
                        return;
                    };

                    let reason = ban_reason().trim().to_string();
                    if reason.is_empty() {
                        return; // Don't ban without a reason
                    }

                    is_submitting.set(true);

                    let updated_user = UserUpdate {
                        banned: Some(true),
                        ban_reason: Some(reason),
                        is_active: Some(false), // Also deactivate the user
                        ..Default::default()
                    };

                    let token_clone = token.clone();
                    let user_id_str = user_id.to_string();
                    let mut show_clone = show.clone();
                    let mut is_submitting_clone = is_submitting.clone();

                    spawn(async move {
                        match user_service::update_user(&token_clone, &user_id_str, updated_user)
                            .await
                        {
                            Ok(_) => {
                                show_clone.set(None);
                            }
                            Err(_) => {
                                // Handle error - could show error message
                                // For now, just reset submitting state
                            }
                        }
                        is_submitting_clone.set(false);
                    });
                }
            };

            let handle_cancel = move |_| {
                show.set(None);
            };

            rsx! {
                div {
                    class: "modal-overlay",
                    div {
                        class: "modal-content",
                        h3 { "Ban User" }

                        div {
                            class: "ban-user-info",
                            p {
                                class: "ban-user-warning",
                                "⚠️ You are about to ban user: "
                                strong { "{user.username}" }
                            }
                            p {
                                class: "ban-user-description",
                                "This will immediately deactivate the user's account and prevent them from accessing the system. Please provide a reason for this action."
                            }
                        }

                        div {
                            class: "modal-form",
                            label {
                                class: "ban-reason-label",
                                "Ban Reason *"
                            }
                            textarea {
                                class: "ban-reason-input",
                                placeholder: "Enter the reason for banning this user...",
                                value: "{ban_reason}",
                                oninput: move |e| ban_reason.set(e.value()),
                                rows: "4",
                                disabled: is_submitting(),
                            }

                            if ban_reason().trim().is_empty() {
                                p {
                                    class: "ban-reason-required",
                                    "A ban reason is required."
                                }
                            }

                            div {
                                class: "modal-actions",
                                button {
                                    class: "btn btn-secondary",
                                    onclick: handle_cancel,
                                    disabled: is_submitting(),
                                    "Cancel"
                                }
                                button {
                                    class: "btn btn-ban",
                                    onclick: handle_ban,
                                    disabled: is_submitting() || ban_reason().trim().is_empty(),
                                    if is_submitting() {
                                        "Banning..."
                                    } else {
                                        "Ban User"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(None) => rsx! {
            div {
                class: "modal-overlay",
                div {
                    class: "modal-content",
                    div {
                        class: "error-message",
                        "User not found"
                    }
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| show.set(None),
                        "Close"
                    }
                }
            }
        },
        None => rsx! {
            div {
                class: "modal-overlay",
                div {
                    class: "modal-content",
                    div { "Loading user information..." }
                }
            }
        },
    };
    x
}
