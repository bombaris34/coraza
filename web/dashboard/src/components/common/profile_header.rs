use crate::{models::User, utils::get_avatar_color};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileHeaderProps {
    user: User,
}

#[component]
pub fn ProfileHeader(props: ProfileHeaderProps) -> Element {
    let user = props.user;
    let avatar_color = get_avatar_color(&user.username);

    rsx! {
        div {
            class: "profile-header",
            div {
                class: "profile-avatar {avatar_color}",
                span {
                    class: "profile-avatar-initials",
                    "{user.username.chars().next().unwrap_or('?').to_uppercase()}"
                }
            }
            div {
                class: "profile-header-info",
                h2 {
                    class: "profile-name",
                    "{user.username}"
                }
                p {
                    class: "profile-email",
                    "{user.email}"
                }
                div {
                    class: "profile-tags",
                    span {
                        class: "tag tag-role",
                        "{user.role.to_string()}"
                    }
                    if user.is_active {
                        span {
                            class: "tag tag-active",
                            "Active"
                        }
                    }
                }
            }
        }
    }
}
