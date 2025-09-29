use crate::components::common::profile_card::ProfileCard;
use dioxus::prelude::*;

#[component]
pub fn SubscriptionSection() -> Element {
    rsx! {
        ProfileCard {
            title: "Account Status",
            p { "Premium clothing management system" }
            p { "Full access to all features" }
        }
    }
}
