use dioxus::prelude::*;

#[component]
pub fn CustomProfile() -> Element {
    rsx! {
        div {
            id: "custom_profile",
            "Custom profile"
        }
    }
}