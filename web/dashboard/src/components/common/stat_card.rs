use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StatCardProps {
    title: String,
    value: String,
    icon_bg_color: String,
    children: Element,
}

#[component]
pub fn StatCard(props: StatCardProps) -> Element {
    let title = props.title;
    let value = props.value;
    let icon_bg_color = props.icon_bg_color;

    rsx! {
        div {
            class: "stat-card",
            div {
                class: "stat-card-content",
                div {
                    class: "stat-card-icon-background {icon_bg_color}",
                    {props.children}
                }
                div {
                    class: "stat-card-text",
                    h4 {
                        class: "stat-card-title",
                        "{title}"
                    }
                    p {
                        class: "stat-card-value",
                        "{value}"
                    }
                }
            }
        }
    }
}
