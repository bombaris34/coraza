use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileInfoItemProps {
    label: String,
    value: String,
    children: Element,
}

#[component]
pub fn ProfileInfoItem(props: ProfileInfoItemProps) -> Element {
    let label = props.label;
    let value = props.value;
    rsx! {
        div {
            class: "profile-info-item",
            div {
                class: "profile-info-icon",
                {props.children}
            }
            div {
                class: "profile-info-text",
                p {
                    class: "profile-info-label",
                    "{label}"
                }
                p {
                    class: "profile-info-value",
                    "{value}"
                }
            }
        }
    }
}
