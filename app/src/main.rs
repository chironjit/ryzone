// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use components::{Navbar, Tabs, Stats, };

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
    pub themeSource: Signal<String>,
    pub themeMode: Signal<String>,
    pub themeLightPalette: Signal<String>,
    pub themeDarkPalette: Signal<String>,
    tab: Signal<String>,
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
        temp:               Signal::new("celcius".to_string()),
        themeSource:        Signal::new("system".to_string()),
        themeMode:          Signal::new("dark".to_string()),
        themeLightPalette:  Signal::new("black".to_string()),
        themeDarkPalette:   Signal::new("dim".to_string()),
        tab:                Signal::new("notice".to_string()),
    });


    let app_state = use_context::<AppState>();
    let mut theme_mode = app_state.themeMode;
    let theme_light = app_state.themeLightPalette;
    let theme_dark = app_state.themeDarkPalette;

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
                        button { class: "px-6 py-3 font-semibold border-b-2 border-[var(--color-primary)] text-[var(--color-primary)] transition-colors",
                            "Dashboard"
                        }
                        button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                            "CPU Control"
                        }
                        button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                            "GPU Control"
                        }
                        button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                            "Power Settings"
                        }
                        button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                            "Fan Control"
                        }
                        button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                            "Profiles"
                        }
                        button { class: "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors",
                            "Settings"
                        }
                    }

                    // Right Side - Quick Settings Icons
                    div { class: "flex items-center gap-3 py-2",
                        // Theme Toggle (Dark/Light Mode)
                        button {
                            class: "p-2 rounded-lg bg-[var(--color-base-300)] hover:bg-[var(--color-neutral)] transition-colors",
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
                                    stroke: "currentColor",
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
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" }
                                }
                            }
                        }

                        // Profile Selector
                        div { class: "relative",
                            button {
                                class: "flex items-center gap-2 px-3 py-2 rounded-lg bg-[var(--color-base-300)] hover:bg-[var(--color-neutral)] transition-colors",
                                title: "Select Profile",
                                // Profile icon using SVG
                                svg {
                                    class: "w-5 h-5",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" }
                                    circle { cx: "12", cy: "7", r: "4" }
                                }
                                span { class: "text-sm font-medium",
                                    "Performance"
                                }
                                // Dropdown arrow
                                svg {
                                    class: "w-4 h-4",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M6 9l6 6 6-6" }
                                }
                            }
                        }
                    }
                }
            }

            // Main Content Area
            main { class: "flex-1 overflow-auto",
                div { class: "p-8 max-w-[1600px] mx-auto",
                        // Header
                        h2 { class: "text-3xl font-bold mb-6 text-[var(--color-base-content)]",
                            "System Statistics"
                        }

                        // Stats Grid - Row 1
                        div { class: "grid grid-cols-3 gap-6 mb-6",
                            // CPU Card
                            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                                div { class: "text-sm font-semibold text-[var(--color-primary)] mb-2",
                                    "CPU"
                                }
                                div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                                    "Frequency"
                                }
                                div { class: "text-4xl font-bold text-[var(--color-base-content)] mb-4",
                                    "4.2 GHz"
                                }
                                div { class: "space-y-2 text-sm text-[var(--color-base-content)]/70",
                                    div { "Temperature: 62°C" }
                                    div { "Load: 45%" }
                                }
                                // Progress bar
                                div { class: "mt-4",
                                    div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                                        div { class: "h-full bg-[var(--color-primary)] rounded-full", style: "width: 45%" }
                                    }
                                }
                            }

                            // GPU Card
                            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                                div { class: "text-sm font-semibold text-[var(--color-success)] mb-2",
                                    "GPU"
                                }
                                div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                                    "Frequency"
                                }
                                div { class: "text-4xl font-bold text-[var(--color-base-content)] mb-4",
                                    "1.8 GHz"
                                }
                                div { class: "space-y-2 text-sm text-[var(--color-base-content)]/70",
                                    div { "Temperature: 58°C" }
                                    div { "Load: 72%" }
                                }
                                div { class: "mt-4",
                                    div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                                        div { class: "h-full bg-[var(--color-success)] rounded-full", style: "width: 72%" }
                                    }
                                }
                            }

                            // Power Card
                            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                                div { class: "text-sm font-semibold text-[var(--color-warning)] mb-2",
                                    "POWER"
                                }
                                div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                                    "Current Draw"
                                }
                                div { class: "text-4xl font-bold text-[var(--color-base-content)] mb-4",
                                    "85 W"
                                }
                                div { class: "space-y-2 text-sm text-[var(--color-base-content)]/70",
                                    div { "Limit: 120W" }
                                    div { "Efficiency: 91%" }
                                }
                                div { class: "mt-4",
                                    div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                                        div { class: "h-full bg-[var(--color-warning)] rounded-full", style: "width: 71%" }
                                    }
                                }
                            }
                        }

                        // Stats Grid - Row 2
                        div { class: "grid grid-cols-3 gap-6 mb-8",
                            // Memory Card
                            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                                div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                                    "MEMORY"
                                }
                                div { class: "text-2xl font-bold text-[var(--color-base-content)] mb-2",
                                    "12.4 / 16.0 GB"
                                }
                                div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                    "Used: 77%"
                                }
                                div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                                    div { class: "h-full bg-[var(--color-secondary)] rounded-full", style: "width: 77%" }
                                }
                            }

                            // Fan Speed Card
                            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                                div { class: "text-sm font-semibold text-[var(--color-info)] mb-2",
                                    "FAN SPEED"
                                }
                                div { class: "text-2xl font-bold text-[var(--color-base-content)] mb-2",
                                    "2400 RPM"
                                }
                                div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                    "Mode: Auto (65%)"
                                }
                                div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                                    div { class: "h-full bg-[var(--color-info)] rounded-full", style: "width: 65%" }
                                }
                            }

                            // System Status Card
                            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                                div { class: "text-sm font-semibold text-[var(--color-accent)] mb-2",
                                    "SYSTEM"
                                }
                                div { class: "text-2xl font-bold text-[var(--color-base-content)] mb-2",
                                    "Performance Mode"
                                }
                                div { class: "text-sm text-[var(--color-base-content)]/70 mb-1",
                                    "Uptime: 4h 23m"
                                }
                                div { class: "text-sm text-[var(--color-success)] flex items-center gap-2",
                                    span { class: "w-2 h-2 bg-[var(--color-success)] rounded-full" }
                                    "All Systems Normal"
                                }
                            }
                        }

                        // Quick Controls Section
                        h2 { class: "text-3xl font-bold mb-6 text-[var(--color-base-content)]",
                            "Quick Controls"
                        }

                        div { class: "bg-[var(--color-base-200)] rounded-xl p-8 border border-[var(--color-base-300)]",
                            div { class: "grid grid-cols-2 gap-x-12 gap-y-8",
                                // CPU Frequency Control
                                div {
                                    label { class: "block text-sm font-semibold text-[var(--color-primary)] mb-3",
                                        "CPU Frequency"
                                    }
                                    div { class: "flex justify-between text-xs text-[var(--color-base-content)]/70 mb-2",
                                        span { "Min: 800 MHz" }
                                        span { "Max: 5.0 GHz" }
                                    }
                                    input {
                                        r#type: "range",
                                        class: "w-full h-2 bg-[var(--color-base-300)] rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[var(--color-primary)]",
                                        min: "0",
                                        max: "100",
                                        value: "67"
                                    }
                                }

                                // Power Limit Control
                                div {
                                    label { class: "block text-sm font-semibold text-[var(--color-warning)] mb-3",
                                        "Power Limit"
                                    }
                                    div { class: "flex justify-between text-xs text-[var(--color-base-content)]/70 mb-2",
                                        span { "50W" }
                                        span { "150W" }
                                    }
                                    input {
                                        r#type: "range",
                                        class: "w-full h-2 bg-[var(--color-base-300)] rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[var(--color-warning)]",
                                        min: "0",
                                        max: "100",
                                        value: "60"
                                    }
                                }

                                // GPU Clock Offset Control
                                div {
                                    label { class: "block text-sm font-semibold text-[var(--color-success)] mb-3",
                                        "GPU Clock Offset"
                                    }
                                    div { class: "flex justify-between text-xs text-[var(--color-base-content)]/70 mb-2",
                                        span { "-200 MHz" }
                                        span { "+200 MHz" }
                                    }
                                    input {
                                        r#type: "range",
                                        class: "w-full h-2 bg-[var(--color-base-300)] rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[var(--color-success)]",
                                        min: "0",
                                        max: "100",
                                        value: "50"
                                    }
                                }

                                // Fan Speed Control
                                div {
                                    label { class: "block text-sm font-semibold text-[var(--color-info)] mb-3",
                                        "Fan Speed"
                                    }
                                    div { class: "flex justify-between text-xs text-[var(--color-base-content)]/70 mb-2",
                                        span { "0%" }
                                        span { "100%" }
                                    }
                                    input {
                                        r#type: "range",
                                        class: "w-full h-2 bg-[var(--color-base-300)] rounded-full appearance-none cursor-pointer [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[var(--color-info)]",
                                        min: "0",
                                        max: "100",
                                        value: "65"
                                    }
                                }
                            }

                            // Action Buttons
                            div { class: "flex gap-4 mt-8",
                                button { class: "px-6 py-3 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    "Apply"
                                }
                                button { class: "px-6 py-3 bg-[var(--color-base-300)] text-[var(--color-base-content)] rounded-lg font-semibold hover:bg-[var(--color-neutral)] transition-colors",
                                    "Reset"
                                }
                                button { class: "px-6 py-3 bg-[var(--color-success)] text-[var(--color-success-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity ml-auto",
                                    "Save Profile"
                                }
                            }
                        }
                }
            }
        }
    }
}
