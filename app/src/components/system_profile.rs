use dioxus::prelude::*;

#[component]
pub fn SystemProfile() -> Element {
    rsx! {
        div {
            id: "system_profile",
            "System profile"
        }
    }
}