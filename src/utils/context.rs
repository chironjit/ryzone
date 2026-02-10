use dioxus::prelude::*;
use crate::utils::types::{AppSettings, ProfileSettings};

pub fn use_app_settings() -> Signal<AppSettings> {
    use_context::<Signal<AppSettings>>()
}

pub fn use_profile_settings() -> Signal<ProfileSettings> {
    use_context::<Signal<ProfileSettings>>()
}