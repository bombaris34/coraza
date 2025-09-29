use crate::{
    components::common::Header,
    state::AppState,
};
use dioxus::prelude::*;

#[component]
pub fn DashboardPage() -> Element {
    let state = use_context::<Signal<AppState>>();

    let state_read = state.read();
    if let Some(current_user) = &state_read.current_user {
        rsx! {
            div {
                class: "main-content",
                Header { current_user: current_user.clone() }
                main {
                    class: "page-content",
                    div {
                        class: "dashboard-container",
                        h3 {
                            class: "page-title",
                            "Clothing Management Dashboard"
                        }

                        // Welcome Section
                        div {
                            class: "welcome-section",
                            div {
                                class: "section-header",
                                h4 {
                                    class: "section-title",
                                    "Welcome back, {current_user.username}!"
                                }
                                div {
                                    class: "section-subtitle",
                                    "Manage your clothing inventory and orders from this dashboard"
                                }
                            }
                        }

                        // Quick Stats Section
                        div {
                            class: "stats-section",
                            div {
                                class: "stats-grid",
                                div {
                                    class: "stat-card",
                                    div {
                                        class: "stat-icon",
                                        "👕"
                                    }
                                    div {
                                        class: "stat-content",
                                        h5 { "Total Products" }
                                        p { class: "stat-number", "—" }
                                        p { class: "stat-label", "In inventory" }
                                    }
                                }
                                div {
                                    class: "stat-card",
                                    div {
                                        class: "stat-icon",
                                        "📦"
                                    }
                                    div {
                                        class: "stat-content",
                                        h5 { "In Stock" }
                                        p { class: "stat-number", "—" }
                                        p { class: "stat-label", "Available items" }
                                    }
                                }
                                div {
                                    class: "stat-card",
                                    div {
                                        class: "stat-icon",
                                        "📈"
                                    }
                                    div {
                                        class: "stat-content",
                                        h5 { "Categories" }
                                        p { class: "stat-number", "—" }
                                        p { class: "stat-label", "Product types" }
                                    }
                                }
                                div {
                                    class: "stat-card",
                                    div {
                                        class: "stat-icon",
                                        "👥"
                                    }
                                    div {
                                        class: "stat-content",
                                        h5 { "Users" }
                                        p { class: "stat-number", "{state_read.stats.total_users}" }
                                        p { class: "stat-label", "Registered users" }
                                    }
                                }
                            }
                        }

                        // Quick Actions Section
                        div {
                            class: "actions-section",
                            div {
                                class: "section-header",
                                h4 {
                                    class: "section-title",
                                    "Quick Actions"
                                }
                                div {
                                    class: "section-subtitle",
                                    "Common management tasks"
                                }
                            }

                            div {
                                class: "actions-grid",
                                a {
                                    href: "#/admin",
                                    class: "action-card",
                                    div {
                                        class: "action-icon",
                                        "📝"
                                    }
                                    div {
                                        class: "action-content",
                                        h5 { "Manage Products" }
                                        p { "Add, edit, or remove clothing items" }
                                    }
                                }
                                a {
                                    href: "#/users",
                                    class: "action-card",
                                    div {
                                        class: "action-icon",
                                        "👤"
                                    }
                                    div {
                                        class: "action-content",
                                        h5 { "Manage Users" }
                                        p { "View and manage user accounts" }
                                    }
                                }
                                a {
                                    href: "#/products",
                                    class: "action-card",
                                    div {
                                        class: "action-icon",
                                        "📊"
                                    }
                                    div {
                                        class: "action-content",
                                        h5 { "View Inventory" }
                                        p { "Check product stock and details" }
                                    }
                                }
                                a {
                                    href: "#/logs",
                                    class: "action-card",
                                    div {
                                        class: "action-icon",
                                        "📋"
                                    }
                                    div {
                                        class: "action-content",
                                        h5 { "System Logs" }
                                        p { "Review system activity and changes" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "flex items-center justify-center h-screen",
                "Loading..."
            }
        }
    }
}