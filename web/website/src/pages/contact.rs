use crate::components::{footer::Footer, header::Header};
use dioxus::prelude::*;

#[component]
pub fn ContactPage() -> Element {
    rsx! {
        div {
            class: "page-container",
            Header {}

            main {
                class: "main-content",
                div {
                    class: "container",
                    div {
                        class: "contact-page",

                        // Page Header
                        div {
                            class: "page-header",
                            h1 {
                                class: "page-title",
                                "Kontakt"
                            }
                        }

                        // Contact Content
                        div {
                            class: "contact-content-simple",

                            div {
                                class: "contact-list",

                                // Instagram
                                div {
                                    class: "contact-item",
                                    h3 {
                                        class: "contact-item-title",
                                        "Instagram"
                                    }
                                    a {
                                        href: "https://www.instagram.com/coraza.clothing/",
                                        class: "contact-item-link",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "@coraza.clothing"
                                    }
                                }

                                // TikTok
                                div {
                                    class: "contact-item",
                                    h3 {
                                        class: "contact-item-title",
                                        "TikTok"
                                    }
                                    a {
                                        href: "https://www.tiktok.com/@coraza.clothing",
                                        class: "contact-item-link",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "@coraza.clothing"
                                    }
                                }

                                // Email
                                div {
                                    class: "contact-item",
                                    h3 {
                                        class: "contact-item-title",
                                        "E-mail"
                                    }
                                    a {
                                        href: "mailto:coraza.hoodie@gmail.com",
                                        class: "contact-item-link",
                                        "coraza.hoodie@gmail.com"
                                    }
                                }

                                // Phone
                                div {
                                    class: "contact-item",
                                    h3 {
                                        class: "contact-item-title",
                                        "Telefónne číslo"
                                    }
                                    a {
                                        href: "tel:+421904196345",
                                        class: "contact-item-link",
                                        "+421 904 196 345"
                                    }
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
