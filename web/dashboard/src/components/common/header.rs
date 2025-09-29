use crate::{models::User, state::AppState, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::{use_navigator, Link};

#[component]
pub fn Header(current_user: User) -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let handle_logout = move |_| {
        // Clear the application state
        let mut state_write = state.write();
        state_write.token = None;
        state_write.current_user = None;

        // Clear the token and user from local storage
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                _ = storage.remove_item("APP_STATE_TOKEN");
                _ = storage.remove_item("APP_STATE_USER");
            }
        }

        // Navigate to the login page
        navigator.push(Route::LoginPage {});
    };

    rsx! {
        header {
            class: "header",
            div {
                class: "header-left mobile-hidden",
                div {
                    class: "user-avatar",
                    span {
                        class: "user-avatar-initials",
                        "{current_user.username.chars().next().unwrap_or('?').to_uppercase()}"
                    }
                }
            }
            div {
                class: "header-center",
                nav {
                    class: "header-nav",
                    Link {
                        to: Route::DashboardPage {},
                        class: "nav-item",
                        span { class: "nav-text", "Dashboard" }
                        span {
                            class: "nav-icon",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "feather feather-home",
                                path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                                polyline { points: "9 22 9 12 15 12 15 22" }
                            }
                        }
                    }
                    Link {
                        to: Route::ProfilePage {},
                        class: "nav-item",
                        span { class: "nav-text", "Profile" }
                        span {
                            class: "nav-icon",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "feather feather-user",
                                path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                circle { cx: "12", cy: "7", r: "4" }
                            }
                        }
                    }
                    if current_user.role == crate::models::UserRole::Admin {
                        Link {
                            to: Route::ProductsPage {},
                            class: "nav-item",
                            span { class: "nav-text", "Products" }
                            span {
                                class: "nav-icon",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "feather feather-package",
                                    path { d: "M16.5 9.4l-9-5.19c-.38-.22-.88-.22-1.26 0l-9 5.19c-.38.22-.62.63-.62 1.07v10.3c0 .44.24.85.62 1.07l9 5.19c.38.22.88.22 1.26 0l9-5.19c.38-.22.62-.63.62-1.07V10.47c0-.44-.24-.85-.62-1.07z" }
                                    polyline { points: "7.5 4.21 12 6.81 16.5 4.21" }
                                    polyline { points: "7.5 19.79 7.5 14.6 2 12" }
                                    polyline { points: "16.5 19.79 16.5 14.6 22 12" }
                                }
                            }
                        }
                        Link {
                            to: Route::AdminPage {},
                            class: "nav-item",
                            span { class: "nav-text", "Admin" }
                            span {
                                class: "nav-icon",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "feather feather-shield",
                                    path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
                                }
                            }
                        }
                        Link {
                            to: Route::LogsPage {},
                            class: "nav-item",
                            span { class: "nav-text", "Logs" }
                            span {
                                class: "nav-icon",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    height: "24",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    class: "feather feather-list",
                                    line { x1: "8", y1: "6", x2: "21", y2: "6" }
                                    line { x1: "8", y1: "12", x2: "21", y2: "12" }
                                    line { x1: "8", y1: "18", x2: "21", y2: "18" }
                                }
                            }
                        }
                    }
                    button {
                        class: "nav-item logout-button",
                        onclick: handle_logout,
                        svg {
                            class: "logout-icon",
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "24",
                            height: "24",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" }
                            polyline { points: "16 17 21 12 16 7" }
                            line { x1: "21", y1: "12", x2: "9", y2: "12" }
                        }
                    }
                }
            }
            div {
                class: "header-right",

            }
        }
    }
}
