use crate::components::{footer::Footer, header::Header};
use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;

#[component]
pub fn TermsPage() -> Element {
    rsx! {
        div {
            class: "page-container",
            Header {}

            main {
                class: "main-content",
                
                section {
                    class: "terms-page",
                    div {
                        class: "container",
                        
                        div {
                            class: "page-header",
                            h1 {
                                class: "page-title",
                                "Podmienky služby"
                            }
                            p {
                                class: "page-subtitle",
                                "Posledná aktualizácia: 1. október 2024"
                            }
                        }

                        div {
                            class: "terms-content",
                            
                            section {
                                class: "terms-section",
                                h2 { "1. Všeobecné podmienky" }
                                p {
                                    "Tieto podmienky služby (\"Podmienky\") upravujú používanie webovej stránky "
                                    "a služieb poskytovaných spoločnosťou Coraza (\"my\", \"nás\", \"naša\"). "
                                    "Používaním našej stránky súhlasíte s týmito podmienkami."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "2. Produkty a služby" }
                                p {
                                    "Predávame oblečenie a módne doplnky. Všetky produkty sú prezentované "
                                    "so základnými informáciami vrátane ceny, veľkosti a dostupnosti. "
                                    "Snažíme sa udržiavať presné informácie, ale chyby sa môžu vyskytnúť."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "3. Objednávky a platby" }
                                ul {
                                    li { "Všetky objednávky podliehajú potvrdeniu dostupnosti" }
                                    li { "Ceny sú uvedené v eurách a zahŕňajú DPH" }
                                    li { "Platba je splatná pri objednávke" }
                                    li { "Prijímame platby kartou, bankovým prevodom a hotovosťou pri dobierke" }
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "4. Dodanie" }
                                p { "Dodací čas:" }
                                ul {
                                    li { "Slovensko: 1-3 pracovné dni" }
                                    li { "Európska únia: 3-7 pracovných dní" }
                                }
                                p {
                                    "Poštovné a balné sa účtuje podľa aktuálneho cenníka prepravcu. "
                                    "Bezplatné doručenie pri objednávke nad 50 €."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "5. Vrátenie a reklamácie" }
                                h3 { "Vrátenie tovaru:" }
                                ul {
                                    li { "Máte právo vrátiť tovar do 14 dní od doručenia" }
                                    li { "Tovar musí byť nepoškodený a v pôvodnom obale" }
                                    li { "Náklady na vrátenie hradí kupujúci" }
                                }
                                h3 { "Reklamácie:" }
                                ul {
                                    li { "Záručná doba je 24 mesiacov od kúpy" }
                                    li { "Reklamácie riešime podľa platnej legislatívy" }
                                    li { "Kontaktujte nás na coraza.hoodie@gmail.com" }
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "6. Duševné vlastníctvo" }
                                p {
                                    "Všetok obsah na tejto webovej stránke, vrátane textov, obrázkov, "
                                    "dizajnu a log, je chránený autorskými právami a je vlastníctvom "
                                    "spoločnosti Coraza alebo jej licenciadateľov."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "7. Obmedzenie zodpovednosti" }
                                p {
                                    "Spoločnosť Coraza nenesie zodpovednosť za nepriame, náhodné alebo "
                                    "následné škody vzniknuté používaním našich produktov alebo služieb, "
                                    "okrem prípadov stanovených zákonom."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "8. Ochrana osobných údajov" }
                                p {
                                    "Spracovanie vašich osobných údajov je upravené našimi "
                                    Link { 
                                        to: Route::PrivacyPage {}, 
                                        class: "link-primary",
                                        "Zásadami ochrany súkromia"
                                    }
                                    "."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "9. Zmeny podmienok" }
                                p {
                                    "Tieto podmienky môžeme kedykoľvek zmeniť. O zmenách vás budeme "
                                    "informovať zverejnením aktualizovaných podmienok na našej stránke. "
                                    "Pokračovaním v používaní našich služieb súhlasíte s novými podmienkami."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "10. Riešenie sporov" }
                                p {
                                    "Tieto podmienky sa riadia slovenským právom. Všetky spory budú "
                                    "riešené pred príslušnými súdmi Slovenskej republiky."
                                }
                            }

                            section {
                                class: "terms-section",
                                h2 { "11. Kontakt" }
                                p { "Pre otázky týkajúce sa týchto podmienok nás kontaktujte:" }
                                div {
                                    class: "contact-info",
                                    p {
                                        strong { "Coraza" }
                                        br {}
                                        "E-mail: "
                                        a { href: "mailto:coraza.hoodie@gmail.com", "coraza.hoodie@gmail.com" }
                                        br {}
                                        "Telefón: "
                                        a { href: "tel:+421904196345", "+421 904 196 345" }
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