use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn HeroSection() -> Element {
    rsx! {
        section {
            class: "hero-section",
            div {
                class: "hero-container",
                div {
                    class: "hero-content",
                    div {
                        class: "hero-text",
                        h1 {
                            class: "hero-title",
                            "Premium Clothing"
                            br {}
                            "for the Modern"
                            br {}
                            span { class: "hero-accent", "Lifestyle" }
                        }
                        p {
                            class: "hero-description",
                            "Discover our carefully curated collection of high-quality clothing designed for comfort, style, and durability. From casual wear to statement pieces, find your perfect look."
                        }
                        div {
                            class: "hero-actions",
                            Link {
                                to: Route::ProductsPage {},
                                class: "btn btn-primary btn-large",
                                "Shop Now"
                            }
                            Link {
                                to: Route::AboutPage {},
                                class: "btn btn-secondary btn-large",
                                "Learn More"
                            }
                        }
                    }
                    div {
                        class: "hero-image",
                        img {
                            src: "https://images.unsplash.com/photo-1441986300917-64674bd600d8?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80",
                            alt: "Premium clothing collection",
                            class: "hero-img"
                        }
                    }
                }
            }
        }
    }
}