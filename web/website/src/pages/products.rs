use crate::{
    components::{header::Header, footer::Footer, product_card::ProductCard},
    services,
};
use dioxus::prelude::*;

#[component]
pub fn ProductsPage() -> Element {
    // Get all products (no filtering)
    let products = use_resource(|| async move {
        services::get_products().await.unwrap_or_default()
    });

    rsx! {
        div {
            class: "page-container",
            Header {}
            
            main {
                class: "products-page",
                div {
                    class: "container",
                    
                    // Page Header
                    div {
                        class: "products-header",
                        h1 { 
                            class: "page-title",
                            "Naše Produkty"
                        }
                        p { 
                            class: "page-subtitle",
                            "Objavte našu kompletnú kolekciu prémiového oblečenia. Nejako picoviny tu povypravať o tom že máme všetko čo potrebujete a ešte viac."
                        }
                    }
                    
                    // Products Grid - Centered
                    div {
                        class: "products-centered",
                        if let Some(product_list) = products.read().as_ref() {
                            if product_list.is_empty() {
                                div {
                                    class: "empty-state",
                                    div {
                                        class: "empty-state-content",
                                        h3 { "Žiadne produkty neboli nájdené" }
                                        p { "Momentálne nemáme žiadne produkty v našej kolekci." }
                                    }
                                }
                            } else {
                                div {
                                    class: "products-grid",
                                    for product in product_list {
                                        ProductCard { product: product.clone() }
                                    }
                                }
                            }
                        } else {
                            div {
                                class: "loading-state",
                                div { class: "loading-spinner" }
                                p { "Načítavanie produktov..." }
                            }
                        }
                    }
                }
            }
            
            Footer {}
        }
    }
}