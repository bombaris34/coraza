use crate::models::RegistrationStats;
use dioxus::prelude::*;

#[component]
pub fn RegistrationChart(stats: Vec<RegistrationStats>) -> Element {
    let max_count = stats.iter().map(|s| s.count).max().unwrap_or(0) as f64;
    let bar_width = 100.0 / (stats.len() as f64 * 1.5);

    rsx! {
        div {
            class: "bg-white rounded-lg shadow-md p-6",
            h4 {
                class: "text-gray-700 text-xl font-medium mb-4",
                "User Registrations (Last 30 Days)"
            }
            div {
                class: "registration-chart",
                svg {
                    width: "100%",
                    height: "100%",
                    view_box: "0 0 100 100",
                    preserve_aspect_ratio: "none",
                    for (i, stat) in stats.iter().enumerate() {
                        rect {
                            x: "{i as f64 * bar_width * 1.5}",
                            y: "{100.0 - (stat.count as f64 / max_count * 100.0)}",
                            width: "{bar_width}",
                            height: "{stat.count as f64 / max_count * 100.0}",
                            class: "bar"
                        }
                    }
                }
            }
        }
    }
}
