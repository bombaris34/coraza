use crate::{
    components::{
        common::Header,
        modals::{AddUserModal, BanUserModal, EditUserModal},
    },
    models::{User, UserUpdate},
    services::user_service,
    state::AppState,
    Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use uuid::Uuid;

#[component]
pub fn UsersPage() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();
    let mut show_add_user = use_signal(|| false);
    let edit_user = use_signal(|| None::<Uuid>);
    let ban_user = use_signal(|| None::<Uuid>);
    let resolve_conflict_user = use_signal(|| None::<Uuid>);

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

    let token_for_users = token_opt.clone().unwrap();

    let users = use_resource(move || {
        let token = token_for_users.clone();
        async move { user_service::get_users(&token).await.unwrap_or_default() }
    });

    let current_user = current_user_opt.unwrap();

    rsx! {
        div {
            class: "main-content",
            Header { current_user: current_user.clone() }
            main {
                class: "page-content",
                div {
                    class: "page-header",
                    h3 {
                        class: "page-title",
                        "User Management"
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| show_add_user.set(true),
                        "Add User"
                    }
                }

                div {
                    class: "user-list",
                    match &*users.read() {
                        Some(users) => rsx! {
                            for user in users {
                                UserCard {
                                    user: user.clone(),
                                    edit_user: edit_user,
                                    ban_user: ban_user,
                                    resolve_conflict_user: resolve_conflict_user,
                                    state: state,
                                }
                            }
                        },
                        None => rsx! { p { "Loading..." } },
                    }
                }
            }
        }

        if show_add_user() {
            AddUserModal { show: show_add_user }
        }

        if let Some(user_id) = edit_user() {
            EditUserModal {
                user_id: user_id,
                show: edit_user
            }
        }

        if let Some(user_id) = ban_user() {
            BanUserModal {
                user_id: user_id,
                show: ban_user
            }
        }

        if let Some(user_id) = resolve_conflict_user() {
            HardwareConflictModal {
                user_id: user_id,
                show: resolve_conflict_user
            }
        }
    }
}

#[component]
fn UserCard(
    user: User,
    edit_user: Signal<Option<Uuid>>,
    ban_user: Signal<Option<Uuid>>,
    resolve_conflict_user: Signal<Option<Uuid>>,
    state: Signal<AppState>,
) -> Element {
    let mut is_expanded = use_signal(|| false);
    let user_for_rsx2 = user.clone();
    let user_for_rsx3 = user.clone();
    let user_for_rsx4 = user.clone();

    // Pre-compute all formatted values outside RSX
    let user_initial = user_for_rsx2
        .username
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    let user_role = user_for_rsx2.role.to_string();

    let formatted_last_login = match &user_for_rsx2.last_login {
        Some(datetime) => datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => "Never".to_string(),
    };

    let formatted_created_at = user_for_rsx2
        .created_at
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();

    let ip_address = user_for_rsx2.ip_address.as_deref().unwrap_or("N/A");

    let ban_reason = user_for_rsx2
        .ban_reason
        .as_deref()
        .unwrap_or("No reason provided");

    let hardware_id_hash = user_for_rsx2.hardware_id_hash.as_deref().unwrap_or("N/A");

    let formatted_hardware_info = match &user_for_rsx2.hardware_info {
        Some(json) => {
            serde_json::to_string_pretty(json).unwrap_or_else(|_| "Invalid JSON".to_string())
        }
        None => "None".to_string(),
    };

    // Check for hardware conflicts
    let has_hardware_conflict = user_for_rsx2.hardware_id_hash.is_some() 
        && user_for_rsx2.last_hardware_hash.is_some() 
        && user_for_rsx2.hardware_id_hash != user_for_rsx2.last_hardware_hash;


    let toggle_card = move |_| {
        is_expanded.set(!is_expanded());
    };

    let user_for_toggle = user.clone();
    let toggle_status = move |_| {
        let Some(token) = state.read().token.clone() else {
            return;
        };
        let user_update = UserUpdate {
            is_active: Some(!user_for_toggle.is_active),
            ..Default::default()
        };
        spawn(async move {
            user_service::update_user(&token, &user_for_toggle.id.to_string(), user_update)
                .await
                .unwrap();
        });
    };

    let user_for_delete = user.clone();
    let delete_user = move |_| {
        let Some(token) = state.read().token.clone() else {
            return;
        };
        spawn(async move {
            user_service::delete_user(&token, &user_for_delete.id.to_string())
                .await
                .unwrap();
        });
    };

    let reset_hardware_info = move |_| {
        // TODO: Implement reset hardware info functionality
        println!("Reset hardware info for user: {}", user_for_rsx2.id);
    };
    let ban_reason = user_for_rsx2
        .ban_reason
        .as_deref()
        .unwrap_or("No reason provided");

    let user_for_rsx = user.clone();
    rsx! {
        div {
            class: "main_class",

            // Main card content (always visible)
            div {
                class: "user-card-main",
                onclick: toggle_card,
                div {
                    class: "user-card-left",
                    div {
                        class: "user-avatar",
                        span {
                            class: "user-avatar-initials",
                            "{user_initial}"
                        }
                    }
                    div {
                        class: "user-info",
                        p {
                            class: "user-name",
                            "{user_for_rsx.username}"
                        }
                        p {
                            class: "user-email",
                            "{user_for_rsx.email}"
                        }
                    }
                }

                div {
                    class: "user-card-right",
                    div {
                        class: "user-status",
                        span {
                            class: if user_for_rsx.is_active { "status-active" } else { "status-inactive" },
                            {if user_for_rsx.is_active { "Active" } else { "Inactive" }}
                        }
                        span {
                            class: "user-role",
                            "{user_role}"
                        }
                        if user_for_rsx.is_banned {
                            span {
                                class: "status-banned",
                                "BANNED"
                            }
                        }
                    }
                    div {
                        class: "expand-indicator",
                        svg {
                            class: "svg_rotation",
                            fill: "currentColor",
                            view_box: "0 0 20 20",
                            path {
                                d: "M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                            }
                        }
                    }
                }
            }

            // Expanded content
            if is_expanded() {
                div {
                    class: "user-card-expanded",

                    div {
                        class: "user-details",
                        div {
                            class: "detail-section",
                            h4 { "Account Information" }
                            div {
                                class: "detail-grid",
                                div {
                                    class: "detail-item",
                                    span { class: "detail-label", "Last Login:" }
                                    span { class: "detail-value", "{formatted_last_login}" }
                                }
                                div {
                                    class: "detail-item",
                                    span { class: "detail-label", "Created:" }
                                    span { class: "detail-value", "{formatted_created_at}" }
                                }
                                div {
                                    class: "detail-item",
                                    span { class: "detail-label", "IP Address:" }
                                    span { class: "detail-value", "{ip_address}" }
                                }
                            }
                        }

                        if user_for_rsx.is_banned {
                            div {
                                class: "detail-section ban-section",
                                h4 { "Ban Information" }
                                div {
                                    class: "detail-item",
                                    span { class: "detail-label", "Ban Reason:" }
                                    span {
                                        class: "detail-value ban-reason",
                                        "{ban_reason}"
                                    }
                                }
                            }
                        }

                        div {
                            class: "detail-section",
                            div {
                                class: "section-header",
                                h4 { "Hardware Information" }
                                div {
                                    class: "hardware-conflict-status",
                                    if has_hardware_conflict {
                                        span { class: "conflict-indicator conflict-found", "⚠️ Hardware Conflict Detected" }
                                        button {
                                            class: "btn btn-resolve-conflict",
                                            onclick: move |_| {
                                                resolve_conflict_user.set(Some(user_for_rsx2.id));
                                            },
                                            "Resolve Conflict"
                                        }
                                    } else {
                                        span { class: "conflict-indicator no-conflict", "✅ No Hardware Conflicts" }
                                        button {
                                            class: "btn btn-resolve-conflict disabled",
                                            disabled: true,
                                            "Resolve Conflict"
                                        }
                                    }
                                }
                            }
                            div {
                                class: "detail-item",
                                span { class: "detail-label", "Hardware ID Hash:" }
                                span { class: "detail-value", "{hardware_id_hash}" }
                            }
                            if user_for_rsx.hardware_info.is_some() {
                                div {
                                    class: "detail-item hardware-info",
                                    span { class: "detail-label", "Hardware Info:" }
                                    pre {
                                        class: "detail-value json-display",
                                        "{formatted_hardware_info}"
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "user-actions-expanded",
                        button {
                            class: "btn btn-edit",
                            onclick: move |_| {
                                let user = user_for_rsx3.clone();
                                edit_user.set(Some(user.id))
                            },
                            "Edit User"
                        }
                        button {
                            class: "btn btn-toggle-status",
                            onclick: toggle_status,
                            {if user_for_rsx.is_active { "Deactivate" } else { "Activate" }}
                        }
                        button {
                            class: "btn btn-reset-hardware",
                            onclick: reset_hardware_info,
                            "Reset Hardware Info"
                        }
                        if !user_for_rsx.is_banned {
                            button {
                                class: "btn btn-ban",
                                onclick: move |_| {
                                    let user = user_for_rsx4.clone();
                                    ban_user.set(Some(user.id))
                                },
                                "Ban User"
                            }
                        } else {
                            button {
                                class: "btn btn-unban",
                                onclick: move |_| {
                                    // TODO: Implement unban functionality
                                    println!("Unban user: {}", user_for_rsx.id);
                                },
                                "Unban User"
                            }
                        }
                        button {
                            class: "btn btn-delete",
                            onclick: delete_user,
                            "Delete User"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn HardwareConflictModal(
    user_id: Uuid,
    show: Signal<Option<Uuid>>,
) -> Element {
    let state = use_context::<Signal<AppState>>();
    
    // Get the user data
    let token = state.read().token.clone().unwrap_or_default();
    let user_data = use_resource(move || {
        let token = token.clone();
        async move {
            user_service::get_users(&token)
                .await
                .unwrap_or_default()
                .into_iter()
                .find(|u| u.id == user_id)
        }
    });

    let close_modal = move |_| show.set(None);

    rsx! {
        div {
            class: "modal-backdrop",
            div {
                class: "hardware-conflict-modal",
                button {
                    class: "modal-close-btn",
                    onclick: close_modal,
                    "×"
                }

                if let Some(Some(user)) = user_data.read().as_ref() {
                    HardwareDiffView { user: user.clone(), show }
                } else {
                    div { class: "loading", "Loading user data..." }
                }
            }
        }
    }
}

#[component]
fn HardwareDiffView(user: User, show: Signal<Option<Uuid>>) -> Element {
    let state = use_context::<Signal<AppState>>();
    
    // Parse hardware info JSON
    let current_hardware = user.hardware_info.as_ref()
        .and_then(|json| serde_json::to_string_pretty(json).ok())
        .unwrap_or_default();
    
    let last_hardware = user.last_hardware_info.as_ref()
        .and_then(|json| serde_json::to_string_pretty(json).ok())
        .unwrap_or_default();

    // Generate diff lines
    let diff_lines = generate_diff_lines(&last_hardware, &current_hardware);

    let resolve_conflict = move |_| {
        let Some(token) = state.read().token.clone() else {
            return;
        };
        
        // Clear the last_hardware fields to resolve the conflict
        let user_update = UserUpdate {
            ..Default::default()
        };
        
        spawn(async move {
            // TODO: Add API endpoint to clear hardware conflict
            // For now, just close the modal
            show.set(None);
        });
    };

    rsx! {
        div {
            class: "diff-view",
            div {
                class: "diff-side diff-old",
                h5 { "Previous Hardware Info" }
                div {
                    class: "diff-content",
                    for line in diff_lines.iter().enumerate() {
                        if line.1.starts_with('-') {
                            div {
                                class: "diff-line diff-removed",
                                span { class: "line-number", "{line.0 + 1}" }
                                span { class: "line-content", "{line.1}" }
                            }
                        } else if !line.1.starts_with('+') {
                            div {
                                class: "diff-line diff-unchanged",
                                span { class: "line-number", "{line.0 + 1}" }
                                span { class: "line-content", "{line.1}" }
                            }
                        }
                    }
                }
            }

            div {
                class: "diff-side diff-new",
                h5 { "Current Hardware Info" }
                div {
                    class: "diff-content",
                    for line in diff_lines.iter().enumerate() {
                        if line.1.starts_with('+') {
                            div {
                                class: "diff-line diff-added",
                                span { class: "line-number", "{line.0 + 1}" }
                                span { class: "line-content", "{line.1}" }
                            }
                        } else if !line.1.starts_with('-') {
                            div {
                                class: "diff-line diff-unchanged",
                                span { class: "line-number", "{line.0 + 1}" }
                                span { class: "line-content", "{line.1}" }
                            }
                        }
                    }
                }
            }
        }

        div {
            class: "modal-actions",
            button {
                class: "btn btn-primary",
                onclick: resolve_conflict,
                "Resolve Conflict"
            }
            button {
                class: "btn btn-secondary",
                onclick: move |_| show.set(None),
                "Cancel"
            }
        }
    }
}

// Helper function to generate diff lines
fn generate_diff_lines(old_content: &str, new_content: &str) -> Vec<String> {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();
    
    let mut diff_lines = Vec::new();
    let max_lines = old_lines.len().max(new_lines.len());
    
    for i in 0..max_lines {
        let old_line = old_lines.get(i).unwrap_or(&"");
        let new_line = new_lines.get(i).unwrap_or(&"");
        
        if old_line != new_line {
            if !old_line.is_empty() {
                diff_lines.push(format!("-{}", old_line));
            }
            if !new_line.is_empty() {
                diff_lines.push(format!("+{}", new_line));
            }
        } else {
            diff_lines.push(format!(" {}", old_line));
        }
    }
    
    diff_lines
}
