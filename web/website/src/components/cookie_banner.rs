use dioxus::prelude::*;

#[component]
pub fn CookieBanner() -> Element {
    let mut show_banner = use_signal(|| {
        // Check if user has already consented
        web_sys::window()
            .and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("cookie_consent").ok().flatten())
            .is_none()
    });

    let accept_cookies = move |_| {
        // Store consent in localStorage
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            let _ = storage.set_item("cookie_consent", "accepted");
        }
        show_banner.set(false);
    };

    let decline_cookies = move |_| {
        // Store decline in localStorage
        if let Ok(Some(storage)) = web_sys::window().unwrap().local_storage() {
            let _ = storage.set_item("cookie_consent", "declined");
        }
        show_banner.set(false);
    };

    if !show_banner() {
        return rsx! { div {} };
    }

    rsx! {
        div {
            class: "cookie-banner",
            div {
                class: "cookie-banner-content",
                div {
                    class: "cookie-banner-text",
                    p {
                        "Používame cookies na zlepšenie vášho zážitku na našej stránke. "
                        "Niektoré cookies sú nevyhnutné pre fungovanie stránky, iné nám pomáhajú "
                        "analyzovať návštevnosť a zlepšovať naše služby."
                    }
                    p {
                        class: "cookie-banner-learn-more",
                        "Viac informácií nájdete v našich "
                        a {
                            href: "/privacy",
                            class: "cookie-banner-link",
                            "zásadách ochrany súkromia"
                        }
                        "."
                    }
                }
                div {
                    class: "cookie-banner-actions",
                    button {
                        class: "btn btn-primary cookie-btn",
                        onclick: accept_cookies,
                        "Súhlasím"
                    }
                    button {
                        class: "btn btn-outline cookie-btn",
                        onclick: decline_cookies,
                        "Odmietnuť"
                    }
                }
            }
        }
    }
}