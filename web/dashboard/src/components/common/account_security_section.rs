use crate::components::common::profile_card::ProfileCard;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AccountSecuritySectionProps {
    on_delete_account: EventHandler<()>,
}

#[component]
pub fn AccountSecuritySection(props: AccountSecuritySectionProps) -> Element {
    rsx! {
        div {
            class: "profile-column",
            ProfileCard {
                title: "Account Security",
                div {
                    class: "security-actions",
                    button {
                        class: "btn btn-delete",
                        onclick: move |_| props.on_delete_account.call(()),
                        "Delete Account"
                    }
                }
            }
        }
    }
}
