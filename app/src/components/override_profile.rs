use dioxus::prelude::*;

#[component]
pub fn OverrideProfile() -> Element {
    rsx! {
        div {
            id: "override_profile",
            "Override profile"
        }
    }
}