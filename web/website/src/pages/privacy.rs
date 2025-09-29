use crate::components::{footer::Footer, header::Header};
use dioxus::prelude::*;

#[component]
pub fn PrivacyPage() -> Element {
    rsx! {
        div {
            class: "page-container",
            Header {}

            main {
                class: "main-content",
                
                section {
                    class: "privacy-page",
                    div {
                        class: "container",
                        
                        div {
                            class: "page-header",
                            h1 {
                                class: "page-title",
                                "Zásady ochrany súkromia"
                            }
                            p {
                                class: "page-subtitle",
                                "Posledná aktualizácia: 1. október 2024"
                            }
                        }

                        div {
                            class: "privacy-content",
                            
                            section {
                                class: "privacy-section",
                                h2 { "1. Úvod" }
                                p {
                                    "Spoločnosť Coraza (\"my\", \"nás\", \"naša\") sa zaväzuje chrániť súkromie "
                                    "našich zákazníkov a návštevníkov webovej stránky. Tieto zásady ochrany "
                                    "súkromia popisujú, ako zhromažďujeme, používame a chránime vaše osobné údaje."
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "2. Údaje, ktoré zhromažďujeme" }
                                p { "Môžeme zhromažďovať nasledujúce typy údajov:" }
                                ul {
                                    li { "Kontaktné informácie (meno, e-mail, telefónne číslo)" }
                                    li { "Informácie o objednávkach a platbách" }
                                    li { "Technické údaje (IP adresa, typ prehliadača, návštevnosť)" }
                                    li { "Cookies a podobné technológie" }
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "3. Ako používame vaše údaje" }
                                p { "Vaše údaje používame na:" }
                                ul {
                                    li { "Spracovanie objednávok a poskytovanie služieb" }
                                    li { "Komunikáciu s vami ohľadom objednávok" }
                                    li { "Zlepšovanie našej webovej stránky a služieb" }
                                    li { "Zaslanie marketingových materiálov (so súhlasom)" }
                                    li { "Dodržiavanie zákonných povinností" }
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "4. Cookies" }
                                p {
                                    "Používame cookies na zlepšenie vášho zážitku na našej stránke. "
                                    "Cookies sú malé textové súbory, ktoré sa ukladajú vo vašom prehliadači. "
                                    "Môžete ich kedykoľvek odstrániť alebo zakázať v nastaveniach prehliadača."
                                }
                                h3 { "Typy cookies, ktoré používame:" }
                                ul {
                                    li { 
                                        strong { "Nevyhnutné cookies: " } 
                                        "Potrebné pre základné fungovanie stránky" 
                                    }
                                    li { 
                                        strong { "Analytické cookies: " } 
                                        "Pomáhajú nám porozumieť, ako návštevníci používajú našu stránku" 
                                    }
                                    li { 
                                        strong { "Marketingové cookies: " } 
                                        "Používané na zobrazovanie relevantných reklám" 
                                    }
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "5. Zdieľanie údajov" }
                                p {
                                    "Vaše osobné údaje nezdieľame s tretími stranami, okrem prípadov, "
                                    "keď je to nevyhnutné pre poskytovanie našich služieb (napr. doprava, platby) "
                                    "alebo keď nás k tomu zaväzuje zákon."
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "6. Vaše práva" }
                                p { "Podľa GDPR máte právo:" }
                                ul {
                                    li { "Na prístup k vašim osobným údajom" }
                                    li { "Na opravu nesprávnych údajov" }
                                    li { "Na vymazanie vašich údajov" }
                                    li { "Na obmedzenie spracovania" }
                                    li { "Na prenosnosť údajov" }
                                    li { "Namietať proti spracovaniu" }
                                }
                                p {
                                    "Pre uplatnenie týchto práv nás kontaktujte na "
                                    a { href: "mailto:coraza.hoodie@gmail.com", "coraza.hoodie@gmail.com" }
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "7. Bezpečnosť údajov" }
                                p {
                                    "Implementovali sme vhodné technické a organizačné opatrenia na ochranu "
                                    "vašich osobných údajov pred neoprávneným prístupom, zmenou, zverejnením "
                                    "alebo zničením."
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "8. Zmeny týchto zásad" }
                                p {
                                    "Tieto zásady ochrany súkromia môžeme príležitostne aktualizovať. "
                                    "O všetkých zmenách vás budeme informovať zverejnením nových zásad "
                                    "na tejto stránke."
                                }
                            }

                            section {
                                class: "privacy-section",
                                h2 { "9. Kontakt" }
                                p {
                                    "Ak máte otázky ohľadom týchto zásad ochrany súkromia, kontaktujte nás:"
                                }
                                div {
                                    class: "contact-info",
                                    p {
                                        strong { "E-mail: " }
                                        a { href: "mailto:coraza.hoodie@gmail.com", "coraza.hoodie@gmail.com" }
                                    }
                                    p {
                                        strong { "Telefón: " }
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