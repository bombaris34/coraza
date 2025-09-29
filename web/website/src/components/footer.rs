use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",
            div {
                class: "footer-container",
                div {
                    class: "footer-grid",

                    // Brand Section
                    div {
                        class: "footer-section",
                        div {
                            class: "footer-brand",
                            img {
                                src: asset!("assets/CORAZA.webp"),
                                alt: "Coraza",
                                class: "footer-logo"
                            }
                            p { class: "footer-tagline", "𝗙𝗼𝗿 𝘁𝗵𝗲 𝘂𝗻𝗰𝗼𝗻𝘃𝗲𝗻𝘁𝗶𝗼𝗻𝗮𝗹 𝘀𝗶𝗱𝗲 𝗼𝗳 𝗴𝘆𝗺 𝗰𝘂𝗹𝘁𝘂𝗿𝗲." }
                        }
                        div {
                            class: "footer-social",
                            a {
                                href: "https://www.instagram.com/coraza.clothing",
                                class: "social-link",
                                title: "Follow us on Instagram",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                img {
                                    src: asset!("assets/INSTAGRAM.png"),
                                    alt: "Instagram",
                                    class: "social-icon"
                                }
                            }
                            a {
                                href: "https://www.tiktok.com/@coraza.clothing",
                                class: "social-link",
                                title: "Follow us on TikTok",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                img {
                                    src: asset!("assets/TIKTOK.webp"),
                                    alt: "TikTok",
                                    class: "social-icon"
                                }
                            }
                        }
                    }

                    // Quick Links
                    div {
                        class: "footer-section",
                        h4 { "Rýchle odkazy" }
                        ul {
                            class: "footer-links",
                            li {
                                Link {
                                    to: Route::HomePage {},
                                    class: "footer-link",
                                    "Domov"
                                }
                            }
                            // Temporarily disabled
                            // li {
                            //     Link {
                            //         to: Route::ProductsPage {},
                            //         class: "footer-link",
                            //         "Produkty"
                            //     }
                            // }
                            li {
                                span {
                                    class: "footer-link",
                                    style: "opacity: 0.5; cursor: not-allowed;",
                                    "Produkty (Čoskoro)"
                                }
                            }
                            li {
                                Link {
                                    to: Route::AboutPage {},
                                    class: "footer-link",
                                    "O nás"
                                }
                            }
                            li {
                                Link {
                                    to: Route::ContactPage {},
                                    class: "footer-link",
                                    "Kontakt"
                                }
                            }
                        }
                    }

                    // Social Media
                    div {
                        class: "footer-section",
                        h4 { "Sledujte nás" }
                        ul {
                            class: "footer-links",
                            li {
                                a {
                                    href: "https://www.instagram.com/coraza.clothing",
                                    class: "footer-link",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "Instagram"
                                }
                            }
                            li {
                                a {
                                    href: "https://www.tiktok.com/@coraza.clothing",
                                    class: "footer-link",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "TikTok"
                                }
                            }
                        }
                    }

                    // Contact Info
                    div {
                        class: "footer-section",
                        h4 { "Kontaktujte nás" }
                        div {
                            class: "footer-contact",
                            p {
                                strong { "Email: " }
                                a {
                                    href: "mailto:coraza.hoodie@gmail.com",
                                    class: "contact-link",
                                    "coraza.hoodie@gmail.com"
                                }
                            }
                            p {
                                strong { "Telefón: " }
                                a {
                                    href: "tel:+421904196345",
                                    class: "contact-link",
                                    "+421 904 196 345"
                                }
                            }
                        }
                    }
                }

                div {
                    class: "footer-bottom",
                    div {
                        class: "footer-bottom-content",
                        p {
                            class: "footer-copyright",
                            "© 2025 Coraza. Všetky práva vyhradené."
                        }
                        div {
                            class: "footer-legal",
                            Link {
                                to: Route::PrivacyPage {},
                                class: "legal-link",
                                "Ochrana súkromia"
                            }
                            Link {
                                to: Route::TermsPage {},
                                class: "legal-link",
                                "Podmienky služby"
                            }
                            a { href: "#", class: "legal-link", "Vrátenie a výmeny" }
                        }
                    }
                }
            }
        }
    }
}
