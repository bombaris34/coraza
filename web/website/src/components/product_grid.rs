use crate::{components::product_card::ProductCard, models::Product};
use dioxus::prelude::*;

#[component]
pub fn ProductGrid(products: Vec<Product>) -> Element {
    rsx! {
        div {
            class: "products-grid",
            for product in products {
                ProductCard { product: product.clone() }
            }
        }
    }
}