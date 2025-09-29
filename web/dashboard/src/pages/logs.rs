use crate::{
    components::common::{Header, TablePlaceholder},
    services::admin_service,
    state::AppState,
};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use serde_json::Value;

#[component]
pub fn LogsPage() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let (current_user_opt, token_opt) = {
        let state = state.read();
        (state.current_user.clone(), state.token.clone())
    };

    let nav = navigator.clone();
    let redirect = current_user_opt.is_none() || token_opt.is_none();
    use_effect(move || {
        if redirect {
            nav.push(crate::Route::LoginPage {});
        }
    });

    if redirect {
        return rsx!(div { "Redirecting..." });
    }

    let current_user = current_user_opt.unwrap();
    let token = token_opt.unwrap();

    let logs = use_resource(move || {
        let token = token.clone();
        async move { admin_service::get_logs(&token).await.unwrap_or_default() }
    });

    rsx! {
        div {
            class: "main-content",
            Header { current_user: current_user.clone() }
            main {
                class: "page-content",
                h3 { class: "page-title", "System Logs" }
                div {
                    class: "logs-container",
                    LogsCard { logs: logs.clone() }
                }
            }
        }
    }
}

#[component]
fn LogsCard(logs: Resource<Vec<serde_json::Value>>) -> Element {
    let mut search_filter = use_signal(String::new);
    let mut log_type_filter = use_signal(|| "all".to_string());

    rsx! {
        div {
            class: "admin-card logs-card",
            div {
                class: "logs-header",
                h4 { class: "card-title", "Activity Logs" }
                div {
                    class: "logs-filters",
                    input {
                        r#type: "text",
                        placeholder: "Search logs...",
                        class: "search-input",
                        value: "{search_filter}",
                        oninput: move |e| search_filter.set(e.value())
                    }
                    select {
                        class: "filter-select",
                        value: "{log_type_filter}",
                        onchange: move |e| log_type_filter.set(e.value()),
                        option { value: "all", "All Types" }
                        option { value: "login", "Login Events" }
                        option { value: "register", "Registration" }
                        option { value: "admin", "Admin Actions" }
                        option { value: "error", "Errors" }
                    }
                }
            }
            div {
                class: "logs-content",
                if let Some(log_list) = logs.read().as_ref() {
                    if log_list.is_empty() {
                        div { class: "empty-logs", "No logs found." }
                    } else {
                        for (index, log) in log_list.iter().enumerate() {
                            LogEntry {
                                key: "{index}",
                                log: log.clone(),
                                search_term: search_filter(),
                                filter_type: log_type_filter()
                            }
                        }
                    }
                } else {
                    div { class: "loading-logs", "Loading logs..." }
                }
            }
        }
    }
}

#[component]
fn LogEntry(log: serde_json::Value, search_term: String, filter_type: String) -> Element {
    let mut expanded = use_signal(|| false);

    // Extract key information from the log
    let empty = serde_json::json!("");
    let data = log.get("action_data").unwrap_or(&empty);

    let timestamp = log
        .get("created_at")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown time");

    let action = data
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let result = data
        .get("result")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    // Filter logic
    let should_show = if filter_type == "all" {
        true
    } else {
        action.contains(&filter_type)
    };

    let matches_search = if search_term.is_empty() {
        true
    } else {
        let search_lower = search_term.to_lowercase();
        log.to_string().to_lowercase().contains(&search_lower)
    };

    if !should_show || !matches_search {
        return rsx! { div { style: "display: none;" } };
    }

    // Determine log level styling
    let log_level = get_log_level(&action, &result);
    let formatted_json = format_json(&log);

    rsx! {
        div {
            class: "log-entry {log_level}",
            div {
                class: "log-summary",
                onclick: move |_| expanded.set(!expanded()),
                div {
                    class: "log-main-info",
                    div {
                        class: "log-timestamp",
                        "{format_timestamp(timestamp)}"
                    }
                    div {
                        class: "log-action",
                        span { class: "action-badge {get_action_type(&action)}", "{action}" }
                        span { class: "result-badge {get_result_type(&result)}", "{result}" }
                    }
                }
                div {
                    class: "log-expand-icon",
                    if expanded() {
                        svg {
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "m18 15-6-6-6 6" }
                        }
                    } else {
                        svg {
                            width: "20",
                            height: "20",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            path { d: "m6 9 6 6 6-6" }
                        }
                    }
                }
            }
            if expanded() {
                div {
                    class: "log-details",
                    div {
                        class: "json-container",
                        pre {
                            class: "json-content",
                            code {
                                class: "json-code",
                                "{formatted_json}"
                            }
                        }
                    }
                }
            }
        }
    }
}

// Helper functions
fn get_log_level(action: &str, result: &str) -> String {
    match result {
        "success" => "level-success".to_string(),
        "error" | "failed" | "wrong_password" | "banned" => "level-error".to_string(),
        "warning" | "wrong_hwid" => "level-warning".to_string(),
        _ => "level-info".to_string(),
    }
}

fn get_action_type(action: &str) -> String {
    if action.contains("login") {
        "action-login".to_string()
    } else if action.contains("register") {
        "action-register".to_string()
    } else if action.contains("admin") {
        "action-admin".to_string()
    } else {
        "action-default".to_string()
    }
}

fn get_result_type(result: &str) -> String {
    match result {
        "success" => "result-success".to_string(),
        "error" | "failed" | "wrong_password" | "banned" => "result-error".to_string(),
        "warning" | "wrong_hwid" => "result-warning".to_string(),
        _ => "result-info".to_string(),
    }
}

fn format_json(value: &serde_json::Value) -> String {
    match serde_json::to_string_pretty(value) {
        Ok(formatted) => formatted,
        Err(_) => value.to_string(),
    }
}

fn format_timestamp(timestamp: &str) -> String {
    // Try to parse and format the timestamp
    if let Ok(dt) = timestamp.parse::<DateTime<Utc>>() {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    } else {
        timestamp.to_string()
    }
}
