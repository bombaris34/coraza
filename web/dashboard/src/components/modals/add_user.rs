use crate::models::{User, UserRole};
use crate::services::user_service;
use crate::state::AppState;
use chrono::Utc;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn AddUserModal(show: Signal<bool>) -> Element {
    let state = use_context::<Signal<AppState>>();
    let mut username = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut role = use_signal(|| UserRole::User);

    let handle_add = move |_| {
        let Some(token) = state.read().token.clone() else {
            return;
        };
        let new_user = User {
            id: Uuid::new_v4(),
            username: username(),
            email: email(),
            role: role(),
            created_at: Utc::now(),
            last_login: None,
            is_active: true,
            hardware_id_hash: None,
            hardware_info: None,
            ip_address: None,
            ban_reason: None,
            is_banned: false,
            last_hardware_hash: None,
            last_hardware_info: None,
        };
        spawn(async move {
            let _ = user_service::create_user(&token, new_user).await;
        });
        show.set(false);
    };

    rsx! {
        div {
            class: "modal-overlay",
            div {
                class: "modal-content",
                h3 { "Add New User" }
                form {
                    class: "modal-form",
                    label { "Username" }
                    input {
                        r#type: "text",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                    }
                    label { "Email" }
                    input {
                        r#type: "email",
                        value: "{email}",
                        oninput: move |e| email.set(e.value()),
                    }
                    label { "Role" }
                    select {
                        onchange: move |e| {
                            let new_role = match e.value().as_str() {
                                "Admin" => UserRole::Admin,
                                "User" => UserRole::User,
                                _ => UserRole::User,
                            };
                            role.set(new_role);
                        },
                        option { value: "User", "User" }
                        option { value: "Admin", "Admin" }
                    }
                    div {
                        class: "modal-actions",
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| show.set(false),
                            "Cancel"
                        }
                        button {
                            class: "btn btn-primary",
                            onclick: handle_add,
                            "Add User"
                        }
                    }
                }
            }
        }
    }
}
