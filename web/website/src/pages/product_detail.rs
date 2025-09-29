use crate::{
    components::{footer::Footer, header::Header, product_card::ProductCard},
    services,
    utils::format_price,
    Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use uuid::Uuid;

#[component]
pub fn ProductDetailPage(id: String) -> Element {
    let navigator = use_navigator();

    let product_id = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            // Invalid UUID, redirect to products page
            navigator.push(Route::ProductsPage {});
            return rsx! { div { "Presmierovanie..." } };
        }
    };

    let product =
        use_resource(move || async move { services::get_product_by_id(product_id).await });

    let related_products =
        use_resource(
            move || async move { services::get_featured_products().await.unwrap_or_default() },
        );

    rsx! {
        div {
            class: "page-container",
            Header {}

            main {
                class: "product-detail-page",
                match product.read().as_ref() {
                    Some(Ok(product)) => rsx! {
                        ProductDetailContent { product: product.clone() }
                        RelatedProductsSection {
                            related_products: related_products,
                            current_product_id: product_id
                        }
                    },
                    Some(Err(_)) => rsx! {
                        div {
                            class: "error-state container",
                            div {
                                class: "error-content",
                                h1 { "Produkt nebol nájdený" }
                                p { "Produkt, ktorý hľadáte, neexistuje alebo bol odstránený." }
                                Link {
                                    to: Route::ProductsPage {},
                                    class: "btn btn-primary",
                                    "Prehľadať všetky produkty"
                                }
                            }
                        }
                    },
                    None => rsx! {
                        div {
                            class: "loading-state container",
                            div { class: "loading-spinner" }
                            p { "Načítavanie detailov produktu..." }
                        }
                    }
                }
            }

            Footer {}
        }
    }
}

#[component]
fn ProductDetailContent(product: crate::models::Product) -> Element {
    let mut selected_size = use_signal(|| None::<String>);
    let mut quantity = use_signal(|| 1u32);
    let product_image_url = if let Some(url) = &product.image_url {
        if !url.is_empty() && !url.starts_with("http") {
            format!("http://127.0.0.1:8081{}", url)
        } else if !url.is_empty() {
            url.to_string()
        } else {
            "https://via.placeholder.com/600x600/f3f4f6/9ca3af?text=No+Image".to_string()
        }
    } else {
        "https://via.placeholder.com/600x600/f3f4f6/9ca3af?text=No+Image".to_string()
    };
    let main_image = use_signal({
        let initial_image = product_image_url.clone();
        move || initial_image.clone()
    });

    let price_display = format_price(product.price);
    let category_display = product.category.as_deref().unwrap_or("Uncategorized");
    let product_name = product.name.clone();
    let product_desc = product.description.clone();
    let product_in_stock = product.in_stock;

    // Mock sizes for demo
    let sizes = vec!["XS", "S", "M", "L", "XL", "XXL"];

    rsx! {
        div {
            class: "container",

            // Breadcrumb Navigation
            nav {
                class: "breadcrumb",
                Link {
                    to: Route::HomePage {},
                    class: "breadcrumb-link",
                    "Domov"
                }
                span { class: "breadcrumb-separator", "/" }
                Link {
                    to: Route::ProductsPage {},
                    class: "breadcrumb-link",
                    "Produkty"
                }
                span { class: "breadcrumb-separator", "/" }
                span { class: "breadcrumb-current", "{category_display}" }
                span { class: "breadcrumb-separator", "/" }
                span { class: "breadcrumb-current", "{product_name}" }
            }

            div {
                class: "product-detail-content",

                // Product Images
                div {
                    class: "product-images",
                    div {
                        class: "main-image-container",
                        img {
                            src: "{main_image}",
                            alt: "{product_name}",
                            class: "main-product-image"
                        }
                        if !product_in_stock {
                            div {
                                class: "product-overlay",
                                span { class: "out-of-stock-badge", "Nie je skladom" }
                            }
                        }
                    }

                    // Thumbnails section removed to fix duplicate image issue
                }

                // Product Info
                div {
                    class: "product-info",
                    div {
                        class: "product-header",
                        span { class: "product-category", "{category_display}" }
                        h1 { class: "product-title", "{product_name}" }
                        div { class: "product-price", "{price_display}" }
                    }

                    div {
                        class: "product-description",
                        p { "{product_desc}" }
                    }

                    // Product Details
                    div {
                        class: "product-details",
                        h3 { "Detaily produktu" }
                        ul {
                            class: "details-list",
                            li { "Prémiové materiály" }
                            li { "Pohodlné a odolavné" }
                            li { "Dostupné v rôznych veľkostiach" }
                            li { "Jednoduchá starostlivosť" }
                            li { "30-dňová záruka vrátenia" }
                        }
                    }

                    div {
                        class: "product-options",

                        // Size Selection
                        div {
                            class: "option-group",
                            label { class: "option-label", "Veľkosť" }
                            div {
                                class: "size-options",
                                for size in &sizes {
                                    button {
                                        class: if selected_size() == Some(size.to_string()) { "size-btn active" } else { "size-btn" },
                                        onclick: {
                                            let size_str = size.to_string();
                                            move |_| selected_size.set(Some(size_str.clone()))
                                        },
                                        "{size}"
                                    }
                                }
                            }
                        }

                        // Quantity Selection
                        div {
                            class: "option-group",
                            label { class: "option-label", "Množstvo" }
                            div {
                                class: "quantity-controls",
                                button {
                                    class: "quantity-btn",
                                    disabled: quantity() <= 1,
                                    onclick: move |_| {
                                        if quantity() > 1 {
                                            quantity.set(quantity() - 1);
                                        }
                                    },
                                    "−"
                                }
                                span { class: "quantity-display", "{quantity()}" }
                                button {
                                    class: "quantity-btn",
                                    disabled: quantity() >= 10,
                                    onclick: move |_| {
                                        if quantity() < 10 {
                                            quantity.set(quantity() + 1);
                                        }
                                    },
                                    "+"
                                }
                            }
                        }
                    }

                    // Action Buttons
                    div {
                        class: "product-actions",
                        button {
                            class: "btn btn-primary btn-large btn-full",
                            disabled: !product_in_stock,
                            onclick: {
                                let name = product_name.clone();
                                move |_| {
                                    // TODO: Add to cart functionality
                                    log::info!("Add to cart: {} x{}", name, quantity());
                                }
                            },
                            if product_in_stock {
                                "Pridať do košíka"
                            } else {
                                "Nie je skladom"
                            }
                        }
                        button {
                            class: "btn btn-outline btn-large btn-full",
                            onclick: {
                                let name = product_name.clone();
                                move |_| {
                                    // TODO: Add to wishlist functionality
                                    log::info!("Add to wishlist: {}", name);
                                }
                            },
                            "Pridať do zoznamu želanií"
                        }
                    }


                    // Stock Status
                    div {
                        class: "stock-status",
                        div {
                            class: if product_in_stock { "stock-indicator in-stock" } else { "stock-indicator out-of-stock" },
                            if product_in_stock {
                                "✓ Skladom - Pripravený na odoslanie"
                            } else {
                                "✗ Momentálne nie je skladom"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RelatedProductsSection(
    related_products: Resource<Vec<crate::models::Product>>,
    current_product_id: Uuid,
) -> Element {
    let filtered_products = related_products.read().as_ref().map(|products| {
        products
            .iter()
            .filter(|p| p.id != current_product_id)
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        section {
            class: "related-products-section",
            div {
                class: "container",
                div {
                    class: "section-header",
                    h2 {
                        class: "section-title",
                        "Môže sa vám páčiť aj"
                    }
                }

                if let Some(products) = filtered_products.as_ref() {
                    if !products.is_empty() {
                        div {
                            class: "related-products-grid",
                            for product in products {
                                ProductCard { product: product.clone() }
                            }
                        }
                    }
                } else {
                    div {
                        class: "related-products-placeholder",
                        for i in 0..4 {
                            div {
                                class: "product-skeleton",
                                key: "{i}"
                            }
                        }
                    }
                }
            }
        }
    }
}
