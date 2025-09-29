use dioxus::prelude::*;

#[component]
pub fn ErrorCard(message: String) -> Element {
    rsx! {
        div {
            class: "error-card",
            div {
                class: "error-card-header",
                div {
                    class: "error-card-icon",
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
                        path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" },
                        line { x1: "12", y1: "9", x2: "12", y2: "13" },
                        line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
                    }
                }
                h3 {
                    class: "error-card-title",
                    "Error occurred!"
                }
            }
            div {
                class: "error-card-body",
                p {
                    "{message}"
                }
            }
            div {
                class: "error-card-footer",
                "You may need to refresh this page to continue."
            }
        }
    }
}
