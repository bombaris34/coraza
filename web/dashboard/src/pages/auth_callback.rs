use crate::{services::user_service, state::AppState, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn AuthCallbackPage() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let token = use_memo(move || {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let search = location.search().unwrap_or_default();
        let params = web_sys::UrlSearchParams::new_with_str(&search).unwrap();
        params.get("token")
    });

    use_effect(move || {
        if let Some(token_str) = token() {
            spawn(async move {
                match user_service::get_current_user(&token_str).await {
                    Ok(user) => {
                        let mut state_write = state.write();
                        state_write.token = Some(token_str.clone());
                        state_write.current_user = Some(user.clone());

                        if let Some(window) = web_sys::window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                _ = storage.set_item("APP_STATE_TOKEN", &token_str);
                                _ = storage.set_item(
                                    "APP_STATE_USER",
                                    &serde_json::to_string(&user).unwrap(),
                                );
                            }
                        }

                        navigator.push(Route::DashboardPage {});
                    }
                    Err(_) => {
                        navigator.push(Route::LoginPage {});
                    }
                }
            });
        } else {
            navigator.push(Route::LoginPage {});
        }
    });

    rsx! {
        div {
            class: "flex items-center justify-center h-screen",
            "Logging in..."
        }
    }
}
