use dioxus::prelude::*;
use crate::utils::types::{AppSettings, ProfileSettings};
use crate::utils::settings::{write_app_settings, write_profile_settings};


#[component]
pub fn Navbar(mut active_tab: Signal<String>) -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut profile = use_context::<Signal<ProfileSettings>>();
    let mut show_profile_dropdown = use_signal(|| false);
    let theme_mode = settings().style.theme_mode.clone();

    let active_profile = profile().active_profile.clone();

    rsx! {
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
                button {
                    class: if active_tab() == "battery" {
                        "px-6 py-3 font-semibold border-b-2 border-[var(--color-primary)] text-[var(--color-base-content)] bg-[var(--color-base-200)] transition-colors"
                    } else {
                        "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors"
                    },
                    onclick: move |_| active_tab.set("battery".to_string()),
                    "Battery"
                }
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
                button {
                    class: if active_tab() == "info" {
                        "px-6 py-3 font-semibold border-b-2 border-[var(--color-primary)] text-[var(--color-base-content)] bg-[var(--color-base-200)] transition-colors"
                    } else {
                        "px-6 py-3 font-semibold border-b-2 border-transparent text-[var(--color-base-content)]/70 hover:text-[var(--color-base-content)] hover:border-[var(--color-base-content)]/30 transition-colors"
                    },
                    onclick: move |_| active_tab.set("info".to_string()),
                    "Info"
                }
            }

            // Right Side - Quick Settings Icons
            div { class: "flex items-center gap-3 py-2",
                // Theme Toggle (Dark/Light Mode)
                button {
                    class: "p-2 rounded-lg bg-[var(--color-base-300)] hover:bg-[var(--color-primary)] stroke-[var(--color-base-content)] hover:stroke-[var(--color-neutral)] transition-colors",
                    title: "Toggle Theme",
                    onclick: move |_| {
                        settings.write().style.theme_mode = if theme_mode == "dark" { "light".to_string() } else { "dark".to_string() };
                        let _ = write_app_settings(&settings());
                    },
                    // Sun/Moon icon using SVG
                    if theme_mode == "dark" {
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
                            "{active_profile}"
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
                                    profile.write().active_profile = "system".to_string();
                                    let _ = write_profile_settings(&profile());
                                    show_profile_dropdown.set(false);
                                },
                                "System"
                            }
                            button {
                                class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors",
                                onclick: move |_| {
                                    profile.write().active_profile = "custom".to_string();
                                    let _ = write_profile_settings(&profile());
                                    show_profile_dropdown.set(false);
                                },
                                "Custom"
                            }
                            button {
                                class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors",
                                onclick: move |_| {
                                    profile.write().active_profile = "turbo".to_string();
                                    let _ = write_profile_settings(&profile());
                                    show_profile_dropdown.set(false);
                                },
                                "Turbo"
                            }
                            button {
                                class: "w-full text-left px-4 py-2 hover:bg-[var(--color-base-300)] transition-colors rounded-b-lg",
                                onclick: move |_| {
                                    profile.write().active_profile = "fixed".to_string();
                                    let _ = write_profile_settings(&profile());
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
    }
}