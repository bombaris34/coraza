use crate::{
    components::common::Header,
    components::modals::delete_account::DeleteAccountModal,
    state::AppState,
    utils::{format_date, format_datetime, get_avatar_color},
    Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn ProfilePage() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let current_user_opt = {
        let state_read = state.read();
        state_read.current_user.clone()
    };

    let user_missing = current_user_opt.is_none();
    let token_missing = state.read().token.is_none();

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
    let mut show_delete_modal = use_signal(|| false);

    // Pre-compute formatted values
    let user_initial = current_user
        .username
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();
    let avatar_color = get_avatar_color(&current_user.username);
    let role_display = current_user.role.to_string();
    let created_date = format_date(current_user.created_at);
    let last_login_display = if let Some(last_login) = current_user.last_login {
        format_datetime(last_login)
    } else {
        "Never".to_string()
    };
    let ip_display = current_user
        .ip_address
        .as_deref()
        .unwrap_or("Not available");
    let hardware_id_display = current_user
        .hardware_id_hash
        .as_deref()
        .unwrap_or("Not available");

    let handle_delete_account = move |_| show_delete_modal.set(true);

    rsx! {
        div {
            class: "main-content",
            Header { current_user: current_user.clone() }

            main {
                class: "page-content",
                div {
                    class: "profile-container",

                    // Profile Header Section
                    div {
                        class: "profile-hero-section",
                        div {
                            class: "profile-hero-content",
                            div {
                                class: "profile-avatar-section",
                                div {
                                    class: "profile-avatar-large {avatar_color}",
                                    span {
                                        class: "profile-avatar-initials-large",
                                        "{user_initial}"
                                    }
                                }
                                div {
                                    class: "profile-status-badges",
                                    span {
                                        class: if current_user.is_active { "profile-badge active" } else { "profile-badge inactive" },
                                        {if current_user.is_active { "Active Account" } else { "Inactive Account" }}
                                    }
                                    if current_user.is_banned {
                                        span {
                                            class: "profile-badge banned",
                                            "Account Banned"
                                        }
                                    }
                                }
                            }
                            div {
                                class: "profile-hero-info",
                                h1 {
                                    class: "profile-hero-name",
                                    "{current_user.username}"
                                }
                                p {
                                    class: "profile-hero-email",
                                    "{current_user.email}"
                                }
                                div {
                                    class: "profile-meta-info",
                                    div {
                                        class: "profile-meta-item",
                                        span { class: "profile-meta-label", "Role:" }
                                        span { class: "profile-meta-value role-{role_display.to_lowercase()}", "{role_display}" }
                                    }
                                    div {
                                        class: "profile-meta-item",
                                        span { class: "profile-meta-label", "Member since:" }
                                        span { class: "profile-meta-value", "{created_date}" }
                                    }
                                }
                            }
                        }
                    }

                    // Main Content Grid
                    div {
                        class: "profile-content-grid",

                        // Account Information Card
                        div {
                            class: "profile-info-card",
                            div {
                                class: "card-header",
                                h3 {
                                    class: "card-title",
                                    svg {
                                        class: "card-title-icon",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                        }
                                    }
                                    "Account Information"
                                }
                            }
                            div {
                                class: "card-content",
                                div {
                                    class: "info-grid",
                                    InfoItem {
                                        label: "Username",
                                        value: current_user.username.clone(),
                                        icon: rsx! {
                                            svg {
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                                }
                                            }
                                        }
                                    }
                                    InfoItem {
                                        label: "Email Address",
                                        value: current_user.email.clone(),
                                        icon: rsx! {
                                            svg {
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                                }
                                            }
                                        }
                                    }
                                    InfoItem {
                                        label: "Account Created",
                                        value: created_date,
                                        icon: rsx! {
                                            svg {
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                                                }
                                            }
                                        }
                                    }
                                    InfoItem {
                                        label: "Last Login",
                                        value: last_login_display,
                                        icon: rsx! {
                                            svg {
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Subscription Information Card
                        div {
                            class: "profile-info-card",
                            div {
                                class: "card-header",
                                h3 {
                                    class: "card-title",
                                    svg {
                                        class: "card-title-icon",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                        }
                                    }
                                    "Subscription Status"
                                }
                            }
                            div {
                                class: "card-content",
                                div {
                                    class: "account-info",
                                    div {
                                        class: "account-main",
                                        h4 { class: "account-type", "Clothing Management System" }
                                    }
                                    div {
                                        class: "account-details",
                                        div {
                                            class: "account-feature",
                                            span { class: "feature-label", "Access Level:" }
                                            span { class: "feature-value", "Full Access" }
                                        }
                                        div {
                                            class: "account-feature",
                                            span { class: "feature-label", "Role:" }
                                            span { class: "feature-value", "{current_user.role}" }
                                        }
                                    }
                                }
                            }
                        }

                        // Security & System Information Card
                        div {
                            class: "profile-info-card",
                            div {
                                class: "card-header",
                                h3 {
                                    class: "card-title",
                                    svg {
                                        class: "card-title-icon",
                                        fill: "none",
                                        stroke: "currentColor",
                                        view_box: "0 0 24 24",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            d: "M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"
                                        }
                                    }
                                    "Security & System"
                                }
                            }
                            div {
                                class: "card-content",
                                div {
                                    class: "info-grid",
                                    InfoItem {
                                        label: "Current IP Address",
                                        value: ip_display.to_string(),
                                        icon: rsx! {
                                            svg {
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"
                                                }
                                            }
                                        }
                                    }
                                    InfoItem {
                                        label: "Hardware ID",
                                        value: hardware_id_display.to_string(),
                                        icon: rsx! {
                                            svg {
                                                fill: "none",
                                                stroke: "currentColor",
                                                view_box: "0 0 24 24",
                                                path {
                                                    stroke_linecap: "round",
                                                    stroke_linejoin: "round",
                                                    stroke_width: "2",
                                                    d: "M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"
                                                }
                                            }
                                        }
                                    }
                                }

                                div {
                                    class: "security-actions",
                                    button {
                                        class: "btn btn-delete",
                                        onclick: handle_delete_account,
                                        "Delete Account"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if show_delete_modal() {
            DeleteAccountModal { show: show_delete_modal }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct InfoItemProps {
    label: String,
    value: String,
    icon: Element,
}

#[component]
fn InfoItem(props: InfoItemProps) -> Element {
    rsx! {
        div {
            class: "info-item",
            div {
                class: "info-item-icon",
                {props.icon}
            }
            div {
                class: "info-item-content",
                span { class: "info-item-label", "{props.label}" }
                span { class: "info-item-value", "{props.value}" }
            }
        }
    }
}
