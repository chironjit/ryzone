use dioxus::prelude::*;

#[component]
pub fn Stats() -> Element {
    rsx! {
        div {
            id: "stats",
            class: "stats stats-vertical lg:stats-horizontal shadow",
            div {
                class: "stat",
                div { class: "stat-title", "Downloads" }
                div { class: "stat-value", "31K" }
                div { class: "stat-desc", "Jan 1st - Feb 1st" }
            }
            div {
                class: "stat",
                div { class: "stat-title", "New Users" }
                div { class: "stat-value", "4,200" }
                div { class: "stat-desc", "↗︎ 400 (22%)" }
            }
            div {
                class: "stat",
                div { class: "stat-title", "New Registers" }
                div { class: "stat-value", "1,200" }
                div { class: "stat-desc", "↘︎ 90 (14%)" }
            }
        }
    }
}