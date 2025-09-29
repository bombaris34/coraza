use crate::models::User;
use crate::services::user_service;
use dioxus::prelude::spawn;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub users: HashMap<Uuid, User>,
    pub current_user: Option<User>,
    pub token: Option<String>,
    pub stats: DashboardStats,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self::default();
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                if let Ok(Some(token)) = storage.get_item("APP_STATE_TOKEN") {
                    state.token = Some(token.clone());

                    let token_for_user = token.clone();
                    spawn(async move {
                        match user_service::get_current_user(&token_for_user).await {
                            Ok(user) => {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        storage
                                            .set_item(
                                                "APP_STATE_USER",
                                                &serde_json::to_string(&user).unwrap(),
                                            )
                                            .unwrap();
                                    }
                                }
                            }
                            Err(_) => {
                                if let Some(window) = web_sys::window() {
                                    if let Ok(Some(storage)) = window.local_storage() {
                                        storage.remove_item("APP_STATE_TOKEN").unwrap();
                                    }
                                }
                            }
                        }
                    });

                }
                if let Ok(Some(user_str)) = storage.get_item("APP_STATE_USER") {
                    if let Ok(user) = serde_json::from_str(&user_str) {
                        state.current_user = Some(user);
                    }
                }
            }
        }
        state
    }
}

#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub total_users: usize,
    pub active_users: usize,
    pub new_users_today: usize,
    pub total_admins: usize,
}
