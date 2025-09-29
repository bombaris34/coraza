use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn Header() -> Element {
    let mut is_menu_open = use_signal(|| false);

    let toggle_menu = move |_| {
        is_menu_open.set(!is_menu_open());
    };

    rsx! {
        header {
            class: "header",
            div {
                class: "header-container",
                div {
                    class: "header-left",
                    Link {
                        to: Route::HomePage {},
                        class: "brand-link",
                        div {
                            class: "header-logo-container",
                            img {
                                src: asset!("assets/CORAZA.webp"),
                                alt: "Coraza",
                                class: "header-logo"
                            }
                        }
                    }
                }

                // Desktop Navigation
                nav {
                    class: "header-nav desktop-nav",
                    Link {
                        to: Route::HomePage {},
                        class: "nav-link",
                        "Domov"
                    }
                    // Temporarily disabled
                    // Link {
                    //     to: Route::ProductsPage {},
                    //     class: "nav-link",
                    //     "Produkty"
                    // }
                    span {
                        class: "nav-link",
                        style: "opacity: 0.5; cursor: not-allowed;",
                        "Produkty (Čoskoro)"
                    }
                    Link {
                        to: Route::AboutPage {},
                        class: "nav-link",
                        "O nás"
                    }
                    Link {
                        to: Route::ContactPage {},
                        class: "nav-link",
                        "Kontakt"
                    }
                }

                // Header Actions
                div {
                    class: "header-actions",

                    // Mobile Menu Button
                    button {
                        class: "mobile-menu-button",
                        onclick: toggle_menu,
                        span { class: if is_menu_open() { "hamburger open" } else { "hamburger" } }
                    }
                }
            }

            // Mobile Navigation
            if is_menu_open() {
                nav {
                    class: "mobile-nav",
                    Link {
                        to: Route::HomePage {},
                        class: "mobile-nav-link",
                        onclick: move |_| is_menu_open.set(false),
                        "Domov"
                    }
                    // Temporarily disabled
                    // Link {
                    //     to: Route::ProductsPage {},
                    //     class: "mobile-nav-link",
                    //     onclick: move |_| is_menu_open.set(false),
                    //     "Produkty"
                    // }
                    span {
                        class: "mobile-nav-link",
                        style: "opacity: 0.5; cursor: not-allowed;",
                        "Produkty (Čoskoro)"
                    }
                    Link {
                        to: Route::AboutPage {},
                        class: "mobile-nav-link",
                        onclick: move |_| is_menu_open.set(false),
                        "O nás"
                    }
                    Link {
                        to: Route::ContactPage {},
                        class: "mobile-nav-link",
                        onclick: move |_| is_menu_open.set(false),
                        "Kontakt"
                    }
                }
            }
        }
    }
}
