// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::{Dashboard, Profiles, Settings};

/// Define a components module that contains all shared components for our app.
mod components;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/icon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

// State tracking
#[derive(Clone, Copy)]
pub struct AppState {
    power: Signal<String>,
    temp: Signal<String>,
    pub themeMode: Signal<String>,
    pub themeLightPalette: Signal<String>,
    pub themeDarkPalette: Signal<String>,
    tab: Signal<String>,
    profile: Signal<String>,
}

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Initialise the state
    // Create the state at the root
    use_context_provider(|| AppState {
        power:              Signal::new("watt".to_string()),
        temp:               Signal::new("celsius".to_string()),
        themeMode:          Signal::new("dark".to_string()),
        themeLightPalette:  Signal::new("nord".to_string()),
        themeDarkPalette:   Signal::new("dim".to_string()),
        tab:                Signal::new("dashboard".to_string()),
        profile:            Signal::new("system".to_string()),
    });


    let app_state = use_context::<AppState>();
    let mut theme_mode = app_state.themeMode;
    let theme_light = app_state.themeLightPalette;
    let theme_dark = app_state.themeDarkPalette;
    let mut active_tab = app_state.tab;
    let mut profile = app_state.profile;
    let mut show_profile_dropdown = use_signal(|| false);

    // Compute the active theme based on mode
    let active_theme = use_memo(move || {
        if theme_mode() == "dark" {
            theme_dark()
        } else {
            theme_light()
        }
    });

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // Root div with data-theme attribute
        // div {
        //     "data-theme": "{active_theme}",
        //     class: "min-h-screen",
        //     style: "background-color: var(--color-base-100); color: var(--color-base-content);",

        //     div {
        //         Navbar{}
        //     }

        //     div {
        //         class: "my-10",
        //         Tabs {  }
        //     }

        //     div {
        //         class: "my-10",
        //         Stats {}
        //     }
            

            

            
        // }
        div { class: "flex flex-col h-screen bg-[var(--color-base-100)] text-[var(--color-base-content)]",
            "data-theme": "{active_theme}",

            // Top Navigation Bar
            header { class: "bg-[var(--color-base-200)] border-b border-[var(--color-base-300)]",
                // Tab Navigation with Quick Settings
                nav { class: "flex items-center justify-between px-6",
                    // Left Side - Tabs
                    div { class: "flex gap-1",
                        button {
                            class: if active_tab() == "dashboard" {
                                "px-6 py-3 font-semibold border-b-2 border-[var(--color-primary)] text-[var(--color-base-content)] bg-[var(--color-base-200)] transition-colors"
                            } else {
                                "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors"
                            },
                            onclick: move |_| active_tab.set("dashboard".to_string()),
                            "Dashboard"
                        }
                        // button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                        //     "CPU Control"
                        // }
                        // button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                        //     "GPU Control"
                        // }
                        // button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                        //     "Power Settings"
                        // }
                        // button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                        //     "Fan Control"
                        // }
                        button {
                            class: if active_tab() == "profiles" {
                                "px-6 py-3 font-semibold border-b-2 border-[var(--color-primary)] text-[var(--color-base-content)] bg-[var(--color-base-200)] transition-colors"
                            } else {
                                "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors"
                            },
                            onclick: move |_| active_tab.set("profiles".to_string()),
                            "Profiles"
                        }
                        button {
                            class: if active_tab() == "settings" {
                                "px-6 py-3 font-semibold border-b-2 border-[var(--color-primary)] text-[var(--color-base-content)] bg-[var(--color-base-200)] transition-colors"
                            } else {
                                "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors"
                            },
                            onclick: move |_| active_tab.set("settings".to_string()),
                            "Settings"
                        }
                    }

                    // Right Side - Quick Settings Icons
                    div { class: "flex items-center gap-3 py-2",
                        // Theme Toggle (Dark/Light Mode)
                        button {
                            class: "p-2 rounded-lg bg-[var(--color-base-300)] hover:bg-[var(--color-primary)] stroke-[var(--color-base-content)] hover:stroke-[var(--color-neutral)] transition-colors",
                            title: "Toggle Theme",
                            onclick: move |_| {
                                let current = theme_mode();
                                theme_mode.set(if current == "dark" { "light".to_string() } else { "dark".to_string() });
                            },
                            // Sun/Moon icon using SVG
                            if theme_mode() == "dark" {
                                // Sun icon for light mode
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    circle { cx: "12", cy: "12", r: "4" }
                                    path { d: "M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" }
                                }
                            } else {
                                // Moon icon for dark mode
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
                                }
                            }
                        }

                        // Profile Selector
                        div { class: "relative",
                            button {
                                class: "flex items-center gap-2 px-3 py-2 rounded-lg bg-[var(--color-base-300)] stroke-[var(--color-base-content)] hover:stroke-[var(--color-neutral)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-colors",
                                title: "Select Profile",
                                onclick: move |_| show_profile_dropdown.set(!show_profile_dropdown()),
                                // Profile icon using SVG
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                    circle { cx: "12", cy: "7", r: "4" }
                                }
                                span { class: "text-sm font-medium capitalize",
                                    "{profile()}"
                                }
                                // Dropdown arrow
                                svg {
                                    class: "w-4 h-4",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M6 9l6 6 6-6" }
                                }
                            }

                            if show_profile_dropdown() {
                                div {
                                    class: "fixed inset-0 z-40",
                                    onclick: move |_| show_profile_dropdown.set(false),
                                }
                                div { class: "absolute right-0 mt-2 w-40 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-lg z-50",
                                    button {
                                        class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors rounded-t-lg",
                                        onclick: move |_| {
                                            profile.set("system".to_string());
                                            show_profile_dropdown.set(false);
                                        },
                                        "System"
                                    }
                                    button {
                                        class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors",
                                        onclick: move |_| {
                                            profile.set("custom".to_string());
                                            show_profile_dropdown.set(false);
                                        },
                                        "Custom"
                                    }
                                    button {
                                        class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors",
                                        onclick: move |_| {
                                            profile.set("turbo".to_string());
                                            show_profile_dropdown.set(false);
                                        },
                                        "Turbo"
                                    }
                                    button {
                                        class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors rounded-b-lg",
                                        onclick: move |_| {
                                            profile.set("fixed".to_string());
                                            show_profile_dropdown.set(false);
                                        },
                                        "Fixed"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-1 overflow-auto",
                match active_tab().as_str() {
                    "dashboard" => rsx! { Dashboard {} },
                    "profiles" => rsx! { Profiles {} },
                    "settings" => rsx! { Settings {} },
                    _ => rsx! { Dashboard {} },
                }
            }
        }
    }
}
