use crate::{
    components::common::{Header, TablePlaceholder},
    models::Product,
    services::admin_service,
    state::AppState,
    Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

#[component]
pub fn ProductsPage() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let (current_user_opt, token_opt) = {
        let state_read = state.read();
        (state_read.current_user.clone(), state_read.token.clone())
    };

    let user_missing = current_user_opt.is_none();
    let token_missing = token_opt.is_none();

    let nav = navigator.clone();
    use_effect(move || {
        if user_missing || token_missing {
            nav.push(Route::LoginPage {});
        }
    });

    if user_missing || token_missing {
        return rsx!(div { "Redirecting..." });
    }

    let current_user = current_user_opt.unwrap();
    let token = token_opt.unwrap();

    let products = use_resource(move || {
        let token = token.clone();
        async move {
            admin_service::get_products(&token)
                .await
                .unwrap_or_default()
        }
    });

    rsx! {
        div {
            class: "main-content",
            Header { current_user: current_user.clone() }
            main {
                class: "page-content",
                div {
                    class: "products-header",
                    h3 {
                        class: "page-title",
                        svg {
                            class: "page-title-icon",
                            fill: "none",
                            stroke: "currentColor",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
                            }
                        }
                        "Product Catalog"
                    }
                    p {
                        class: "page-subtitle",
                        "Browse our clothing collection"
                    }
                }
                
                ProductGrid { products: products }
            }
        }
    }
}

#[component]
fn ProductGrid(products: Resource<Vec<Product>>) -> Element {
    rsx! {
        div {
            class: "products-container",
            if let Some(product_list) = products.read().as_ref() {
                if product_list.is_empty() {
                    div {
                        class: "empty-state",
                        div {
                            class: "empty-state-content",
                            svg {
                                class: "empty-state-icon",
                                fill: "none",
                                stroke: "currentColor",
                                view_box: "0 0 24 24",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    d: "M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"
                                }
                            }
                            h4 { "No Products Available" }
                            p { "Check back later for new arrivals!" }
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
                    div {
                        class: "loading-spinner"
                    }
                    p { "Loading products..." }
                }
            }
        }
    }
}

#[component]
fn ProductCard(product: Product) -> Element {
    let price_display = format!("${:.2}", product.price);
    let category_display = product.category.as_deref().unwrap_or("Uncategorized");
    let stock_status = if product.in_stock { "In Stock" } else { "Out of Stock" };
    let stock_class = if product.in_stock { "stock-available" } else { "stock-unavailable" };

    rsx! {
        div {
            class: "product-card",
            if let Some(image_url) = &product.image_url {
                div {
                    class: "product-image",
                    img {
                        src: "{image_url}",
                        alt: "{product.name}",
                        loading: "lazy"
                    }
                }
            } else {
                div {
                    class: "product-image-placeholder",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        view_box: "0 0 24 24",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                        }
                    }
                }
            }
            div {
                class: "product-info",
                div {
                    class: "product-header",
                    h4 {
                        class: "product-name",
                        "{product.name}"
                    }
                    span {
                        class: "product-category",
                        "{category_display}"
                    }
                }
                if !product.description.is_empty() {
                    p {
                        class: "product-description",
                        "{product.description}"
                    }
                }
                div {
                    class: "product-footer",
                    div {
                        class: "product-price",
                        "{price_display}"
                    }
                    div {
                        class: "product-stock {stock_class}",
                        "{stock_status}"
                    }
                }
            }
        }
    }
}