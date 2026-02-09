//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals. In this template, we define a Hero
//! component  to be used in our app.


mod dashboard;
mod profiles;
mod settings;
mod battery;
mod info;

pub use dashboard::Dashboard;
pub use profiles::Profiles;
pub use settings::Settings;
pub use battery::Battery;
pub use info::Info;

