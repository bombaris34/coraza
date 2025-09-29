use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileCardProps {
    title: String,
    children: Element,
}

#[component]
pub fn ProfileCard(props: ProfileCardProps) -> Element {
    let title = props.title;
    rsx! {
        div {
            class: "profile-card",
            h3 {
                class: "profile-card-title",
                "{title}"
            }
            div {
                class: "profile-card-content",
                {props.children}
            }
        }
    }
}
