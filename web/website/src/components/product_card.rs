use crate::{models::Product, utils::format_price, Route};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProductCard(product: Product) -> Element {
    let price_display = format_price(product.price);
    let image_src = if let Some(url) = &product.image_url {
        if !url.is_empty() && !url.starts_with("http") {
            format!("http://127.0.0.1:8081{}", url)
        } else if !url.is_empty() {
            url.to_string()
        } else {
            "https://via.placeholder.com/300x300/f3f4f6/9ca3af?text=No+Image".to_string()
        }
    } else {
        "https://via.placeholder.com/300x300/f3f4f6/9ca3af?text=No+Image".to_string()
    };

    rsx! {
        Link {
            to: Route::ProductDetailPage { id: product.id.to_string() },
            class: "product-card-link",
            div {
                class: "product-card-minimal",
                div {
                    class: "product-image-container-minimal",
                    img {
                        src: "{image_src}",
                        alt: "{product.name}",
                        class: "product-image-minimal",
                        loading: "lazy"
                    }
                }
                
                div {
                    class: "product-info-minimal",
                    h3 {
                        class: "product-name-minimal",
                        "{product.name}"
                    }
                    div {
                        class: "product-price-minimal",
                        "{price_display}"
                    }
                }
            }
        }
    }
}