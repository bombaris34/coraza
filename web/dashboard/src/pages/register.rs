use crate::{components::common::ErrorCard, services::user_service, state::AppState, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, Link};

#[component]
pub fn RegisterPage() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    use_effect(move || {
        if state.read().token.is_some() && state.read().current_user.is_some() {
            navigator.push(Route::DashboardPage {});
        }
    });
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| None::<String>);
    let mut trigger_register = use_signal(|| false);

    let _ = use_resource(move || async move {
        if !*trigger_register.read() {
            return Ok(None);
        }
        let username = username.read().clone();
        let email = email.read().clone();
        let password = password.read().clone();
        match user_service::register(crate::models::UserRegistration {
            username,
            email,
            password,
        })
        .await
        {
            Ok(token) => {
                if let Ok(user) = user_service::validate_token(&token).await {
                    if user.is_banned {
                        error.set(Some("Your account has been banned.".into()));
                        return Ok(None);
                    }
                    state.write().token = Some(token.clone());
                    state.write().current_user = Some(user.clone());

                    if let Some(window) = web_sys::window() {
                        if let Ok(Some(storage)) = window.local_storage() {
                            _ = storage.set_item("APP_STATE_TOKEN", &token);
                            _ = storage.set_item(
                                "APP_STATE_USER",
                                &serde_json::to_string(&user).unwrap(),
                            );
                        }
                    }
                    navigator.push(Route::DashboardPage {});
                    Ok(Some(token))
                } else {
                    error.set(Some("Registration failed".into()));
                    Ok(None)
                }
            }
            Err(e) => {
                error.set(Some("Registration failed".into()));
                Err(e)
            }
        }
    });

    rsx! {
        div {
            class: "login-container",
            div {
                class: "login-card",
                h2 { class: "login-title", "Register" }
                if let Some(err) = error() { ErrorCard { message: err } }
                input {
                    r#type: "text",
                    placeholder: "Username",
                    value: "{username}",
                    oninput: move |evt| username.set(evt.value().to_string()),
                }
                input {
                    r#type: "email",
                    placeholder: "Email",
                    value: "{email}",
                    oninput: move |evt| email.set(evt.value().to_string()),
                }
                input {
                    r#type: "password",
                    placeholder: "Password",
                    value: "{password}",
                    oninput: move |evt| password.set(evt.value().to_string()),
                }
                button { class: "btn", onclick: move |_| trigger_register.set(true), "Register" }
                p { class: "toggle-form", "Already have an account? ", Link { to: Route::LoginPage {}, "Login" } }
            }
        }
    }
}
