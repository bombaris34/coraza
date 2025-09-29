use crate::components::{header::Header, footer::Footer};
use dioxus::prelude::*;

#[component]
pub fn AboutPage() -> Element {
    rsx! {
        div {
            class: "page-container",
            Header {}
            
            main {
                class: "main-content",
                div {
                    class: "container",
                    div {
                        class: "about-page",
                        
                        // About Content
                        div {
                            class: "about-content-text",
                            
                            // First Section
                            section {
                                class: "about-section",
                                h3 {
                                    class: "about-section-subtitle",
                                    "O nás"
                                }
                                p {
                                    class: "about-text",
                                    "Naše produkty šijeme výhradne z vysokokvalitných bavlnených materiálov. Prispôsobené na časté používanie, pranie a dlhú životnosť. Dávame si záležať na spracovaní ,preto všetky produkty šijeme na Slovensku. Strihy sme prispôsobili tak aby aj pri oversized prevedení zostali veci plne funkčné pri cvičení. A zároveň vyzerali dobre aj v uliciach."
                                }
                            }

                            // Separator
                            div { class: "about-separator" }

                            // Second Section
                            section {
                                class: "about-section",
                                h3 {
                                    class: "about-section-subtitle",
                                    "Myšlienka"
                                }
                                p {
                                    class: "about-text",
                                    "Veríme v dlhodobý proces a tvrdú prácu. Každý má iné podmienky, inú fázu procesu. Záleží len na tom, že sám vieš ,že robíš maximum. Preto odmietame prázdne porovnávanie sa s ostatnými."
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