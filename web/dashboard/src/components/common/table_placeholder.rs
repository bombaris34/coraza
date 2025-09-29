use dioxus::prelude::*;

#[component]
pub fn TablePlaceholder(rows: u32, cols: u32) -> Element {
    rsx! {
        for _ in 0..rows {
            tr {
                for _ in 0..cols {
                    td {
                        div {
                            class: "skeleton-loader",
                        }
                    }
                }
            }
        }
    }
}
