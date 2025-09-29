use crate::models::{UserRole, UserUpdate};
use crate::services::user_service;
use crate::state::AppState;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn EditUserModal(user_id: Uuid, show: Signal<Option<Uuid>>) -> Element {
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
            let mut username = use_signal(|| user.username.clone());
            let mut email = use_signal(|| user.email.clone());
            let mut role = use_signal(|| user.role.clone());

            let handle_save = {
                let state = state.clone();
                let username = username.clone();
                let email = email.clone();
                let role = role.clone();

                move |_| {
                    let Some(token) = state.read().token.clone() else {
                        return;
                    };

                    let updated_user = UserUpdate {
                        username: Some(username()),
                        email: Some(email()),
                        role: Some(role()),
                        ..Default::default()
                    };

                    let token_clone = token.clone();
                    let user_id_str = user_id.to_string();

                    spawn(async move {
                        user_service::update_user(&token_clone, &user_id_str, updated_user)
                            .await
                            .unwrap();
                    });
                }
            };

            rsx! {
                div {
                    class: "modal-overlay",
                    div {
                        class: "modal-content",
                        h3 { "Edit User" }
                        div {
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
                                option { value: "User", selected: user.role == UserRole::User, "User" }
                                option { value: "Admin", selected: user.role == UserRole::Admin, "Admin" }
                            }
                            div {
                                class: "modal-actions",
                                button {
                                    class: "btn btn-secondary",
                                    onclick: move |_| show.set(None),
                                    "Cancel"
                                }
                                button {
                                    class: "btn btn-primary",
                                    onclick: handle_save,
                                    "Save Changes"
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(None) => rsx! {div {"User not found"}},
        None => rsx! {div {"Loading..."}},
    };
    x
}
