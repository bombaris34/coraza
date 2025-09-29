use crate::{
    components::common::{profile_card::ProfileCard, profile_info_item::ProfileInfoItem},
    models::User,
    utils::{format_date, format_datetime},
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileInfoSectionProps {
    user: User,
}

#[component]
pub fn ProfileInfoSection(props: ProfileInfoSectionProps) -> Element {
    let user = props.user;

    rsx! {
        ProfileCard {
            title: "Profile Information",
            ProfileInfoItem {
                label: "Username",
                value: user.username.clone(),
                svg {
                    class: "profile-info-icon-svg",
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
            ProfileInfoItem {
                label: "Email",
                value: user.email.clone(),
                svg {
                    class: "profile-info-icon-svg",
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
            ProfileInfoItem {
                label: "Member Since",
                value: format_date(user.created_at),
                svg {
                    class: "profile-info-icon-svg",
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
            ProfileInfoItem {
                label: "Last Login",
                value: if let Some(last_login) = user.last_login {
                    format_datetime(last_login)
                } else {
                    "Never".to_string()
                },
                svg {
                    class: "profile-info-icon-svg",
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
