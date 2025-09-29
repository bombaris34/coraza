use crate::models::Category;
use dioxus::prelude::*;

#[component]
pub fn CategoryFilter(
    categories: Vec<Category>,
    selected_category: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "category-filter",
            label {
                class: "filter-label",
                "Category"
            }
            select {
                class: "filter-select",
                onchange: move |e| {
                    let value = e.value();
                    selected_category.set(if value.is_empty() { None } else { Some(value) });
                },
                option { value: "", "All Categories" }
                for category in categories {
                    option { 
                        value: "{category.name}",
                        selected: selected_category() == Some(category.name.clone()),
                        "{category.name} ({category.count})"
                    }
                }
            }
        }
    }
}