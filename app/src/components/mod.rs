//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.


mod navbar;
mod stats;
mod tab;
mod custom_profile;
mod override_profile;
mod system_profile;
mod settings;


pub use navbar::Navbar;
pub use stats::Stats;
pub use tab::Tabs;
pub use custom_profile::CustomProfile;
pub use override_profile::OverrideProfile;
pub use system_profile::SystemProfile;
pub use settings::Settings;

