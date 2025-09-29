use crate::{
    components::{footer::Footer, header::Header, product_card::ProductCard},
    models::Product,
    services, Route,
};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn HomePage() -> Element {
    // Temporarily disabled - not fetching products
    // let featured_products =
    //     use_resource(|| async move { services::get_featured_products().await.unwrap_or_default() });
    let featured_products = use_resource(|| async move { Vec::<Product>::new() });

    rsx! {
        div {
            class: "page-container",
            Header {}

            main {
                class: "main-content",

                // Hero Section
                HeroSection {}

                // Featured Products Section - Temporarily disabled
                // section {
                //     class: "featured-section",
                //     div {
                //         class: "container",
                //         div {
                //             class: "section-header",
                //             h2 {
                //                 class: "section-title",
                //                 "Novinky"
                //             }
                //         }

                //         if let Some(products) = featured_products.read().as_ref() {
                //             if products.is_empty() {
                //                 div {
                //                     class: "empty-state",
                //                     h3 { "Čoskoro!" }
                //                     p { "Momentálne pridávame nové produkty do našej kolekcie." }
                //                 }
                //             } else {
                //                 div {
                //                     class: "products-carousel-container",
                //                     div {
                //                         class: "products-carousel",
                //                         for product in products {
                //                             ProductCard { product: product.clone() }
                //                         }
                //                     }
                //                 }
                //             }
                //         } else {
                //             div {
                //                 class: "loading-state",
                //                 div { class: "loading-spinner" }
                //                 p { "Načítavanie odporúčaných produktov..." }
                //             }
                //         }
                //     }
                // }

                // Temporary "Coming Soon" Section
                section {
                    class: "featured-section",
                    div {
                        class: "container",
                        div {
                            style: "text-align: center; padding: 60px 20px;",
                            div {
                                img {
                                    src: asset!("assets/arc.png"),
                                    alt: "Arc",
                                    style:"max-width: 400px; width: 100%; height: auto; "
                                }
                            }
                            h2 {
                                class: "section-title",
                                style: "font-size: 1.2rem; margin-top: 30px; margin-bottom: 40px; white-space: pre-line;",
                                "Predpredaj kolekcie\n„let the arc start\"\nspúšťame 1. Októbra!"
                            }
                            div {
                                style: "display: flex; gap: 30px; justify-content: center; flex-wrap: wrap; margin-top: 40px;",
                                img {
                                    src: asset!("assets/IMG1.webp"),
                                    alt: "Coming Soon Image 1",
                                    style: "max-width: 400px; width: 100%; height: auto; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);"
                                }
                                img {
                                    src: asset!("assets/IMG2.webp"),
                                    alt: "Coming Soon Image 2",
                                    style: "max-width: 400px; width: 100%; height: auto; border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);"
                                }
                            }
                        }
                    }
                }

            }

            Footer {}
        }
    }
}

#[component]
fn HeroSection() -> Element {
    rsx! {
        section {
            class: "hero-section",
            div {
                class: "hero-container",
                div {
                    class: "hero-content-centered",
                    div {
                        class: "hero-logo-container",
                        style: format!(
                            "--desktop-bg: url('{}'); --mobile-bg: url('{}');",
                            asset!("assets/hero_pc2.jpg"),
                            asset!("assets/IMGBAN.jpg")
                        ),
                        img {
                            src: asset!("assets/CORAZA.webp"),
                            alt: "Coraza Logo",
                            class: "hero-logo"
                        }
                        Link {
                            to: Route::AboutPage {},
                            class: "hero-overlay-btn",
                            "O nás"
                        }
                    }
                }
            }
        }
    }
}
