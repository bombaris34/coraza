use dioxus::prelude::*;

#[component]
pub fn InfoTooltip(text: String) -> Element {
    rsx! {
        div {
            class: "info-tooltip",
            "i",
            div {
                class: "info-tooltip-text",
                "{text}"
            }
        }
    }
}
