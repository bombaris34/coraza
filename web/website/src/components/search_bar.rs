use dioxus::prelude::*;

#[component]
pub fn SearchBar(search_query: Signal<String>) -> Element {
    rsx! {
        div {
            class: "search-bar",
            label {
                class: "filter-label",
                "Search"
            }
            input {
                class: "filter-input",
                r#type: "text",
                placeholder: "Search products...",
                value: "{search_query}",
                oninput: move |e| search_query.set(e.value())
            }
        }
    }
}