use dioxus::prelude::*;
use crate::AppState;

#[component]
pub fn Settings() -> Element {
    let app_state = use_context::<AppState>();
    let mut theme_mode = app_state.themeMode;
    let mut theme_light_palette = app_state.themeLightPalette;
    let mut theme_dark_palette = app_state.themeDarkPalette;
    let mut power_unit = app_state.power;
    let mut temp_unit = app_state.temp;
    let mut show_theme_mode_dropdown = use_signal(|| false);
    let mut show_light_palette_dropdown = use_signal(|| false);
    let mut show_dark_palette_dropdown = use_signal(|| false);
    let mut show_temp_unit_dropdown = use_signal(|| false);
    let mut show_power_unit_dropdown = use_signal(|| false);
    let mut show_update_freq_dropdown = use_signal(|| false);
    let mut update_frequency = use_signal(|| "1000".to_string());

    rsx! {
        div { class: "p-8 max-w-[1600px] mx-auto",
            // Appearance and Units Grid
            div { class: "grid grid-cols-2 gap-6 mb-6",
                // Appearance Settings
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    h3 { class: "text-xl font-bold text-[var(--color-base-content)] mb-4",
                        "Appearance"
                    }

                    // Light Theme Palette
                    div { class: "mb-6 relative",
                        label { class: "block text-sm font-semibold text-[var(--color-base-content)] mb-3",
                            "Light Theme Palette"
                        }
                        button {
                            class: "flex items-center justify-between w-full px-4 py-3 rounded-lg bg-[var(--color-base-300)] text-[var(--color-base-content)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-all duration-200 shadow-sm hover:shadow-md border border-transparent hover:border-[var(--color-primary)]",
                            title: "Select Light Theme Palette",
                            onclick: move |_| show_light_palette_dropdown.set(!show_light_palette_dropdown()),
                            span { class: "text-sm font-medium capitalize",
                                "{theme_light_palette()}"
                            }
                            svg {
                                class: if show_light_palette_dropdown() { "w-4 h-4 transform rotate-180 transition-transform duration-200" } else { "w-4 h-4 transition-transform duration-200" },
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M6 9l6 6 6-6" }
                            }
                        }

                        if show_light_palette_dropdown() {
                            div {
                                class: "fixed inset-0 z-40",
                                onclick: move |_| show_light_palette_dropdown.set(false),
                            }
                            div { class: "absolute left-0 right-0 mt-2 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-xl z-50 overflow-hidden",
                                button {
                                    class: if theme_light_palette() == "winter" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        theme_light_palette.set("winter".to_string());
                                        show_light_palette_dropdown.set(false);
                                    },
                                    "Winter"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_light_palette() == "black" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        theme_light_palette.set("black".to_string());
                                        show_light_palette_dropdown.set(false);
                                    },
                                    "Black"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_light_palette() == "nord" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        theme_light_palette.set("nord".to_string());
                                        show_light_palette_dropdown.set(false);
                                    },
                                    "Nord"
                                }
                            }
                        }
                    }

                    // Dark Theme Palette
                    div { class: "mb-0 relative",
                        label { class: "block text-sm font-semibold text-[var(--color-base-content)] mb-3",
                            "Dark Theme Palette"
                        }
                        button {
                            class: "flex items-center justify-between w-full px-4 py-3 rounded-lg bg-[var(--color-base-300)] text-[var(--color-base-content)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-all duration-200 shadow-sm hover:shadow-md border border-transparent hover:border-[var(--color-primary)]",
                            title: "Select Dark Theme Palette",
                            onclick: move |_| show_dark_palette_dropdown.set(!show_dark_palette_dropdown()),
                            span { class: "text-sm font-medium capitalize",
                                "{theme_dark_palette()}"
                            }
                            svg {
                                class: if show_dark_palette_dropdown() { "w-4 h-4 transform rotate-180 transition-transform duration-200" } else { "w-4 h-4 transition-transform duration-200" },
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M6 9l6 6 6-6" }
                            }
                        }

                        if show_dark_palette_dropdown() {
                            div {
                                class: "fixed inset-0 z-40",
                                onclick: move |_| show_dark_palette_dropdown.set(false),
                            }
                            div { class: "absolute left-0 right-0 mt-2 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-xl z-50 overflow-hidden",
                                button {
                                    class: if theme_dark_palette() == "dracula" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        theme_dark_palette.set("dracula".to_string());
                                        show_dark_palette_dropdown.set(false);
                                    },
                                    "Dracula"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_dark_palette() == "night" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        theme_dark_palette.set("night".to_string());
                                        show_dark_palette_dropdown.set(false);
                                    },
                                    "Night"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_dark_palette() == "dim" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        theme_dark_palette.set("dim".to_string());
                                        show_dark_palette_dropdown.set(false);
                                    },
                                    "Dim"
                                }
                            }
                        }
                    }
                }

                // Unit Settings
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    h3 { class: "text-xl font-bold text-[var(--color-base-content)] mb-4",
                        "Units"
                    }

                    // Temperature Unit
                    div { class: "mb-6 relative",
                        label { class: "block text-sm font-semibold text-[var(--color-base-content)] mb-3",
                            "Temperature Unit"
                        }
                        button {
                            class: "flex items-center justify-between w-full px-4 py-3 rounded-lg bg-[var(--color-base-300)] text-[var(--color-base-content)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-all duration-200 shadow-sm hover:shadow-md border border-transparent hover:border-[var(--color-primary)]",
                            title: "Select Temperature Unit",
                            onclick: move |_| show_temp_unit_dropdown.set(!show_temp_unit_dropdown()),
                            span { class: "text-sm font-medium capitalize",
                                {if temp_unit() == "celsius" { "Celsius (°C)" } else { "Fahrenheit (°F)" }}
                            }
                            svg {
                                class: if show_temp_unit_dropdown() { "w-4 h-4 transform rotate-180 transition-transform duration-200" } else { "w-4 h-4 transition-transform duration-200" },
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M6 9l6 6 6-6" }
                            }
                        }

                        if show_temp_unit_dropdown() {
                            div {
                                class: "fixed inset-0 z-40",
                                onclick: move |_| show_temp_unit_dropdown.set(false),
                            }
                            div { class: "absolute left-0 right-0 mt-2 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-xl z-50 overflow-hidden",
                                button {
                                    class: if temp_unit() == "celsius" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        temp_unit.set("celsius".to_string());
                                        show_temp_unit_dropdown.set(false);
                                    },
                                    "Celsius (°C)"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if temp_unit() == "fahrenheit" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        temp_unit.set("fahrenheit".to_string());
                                        show_temp_unit_dropdown.set(false);
                                    },
                                    "Fahrenheit (°F)"
                                }
                            }
                        }
                    }

                    // Power Unit
                    div { class: "mb-0 relative",
                        label { class: "block text-sm font-semibold text-[var(--color-base-content)] mb-3",
                            "Power Unit"
                        }
                        button {
                            class: "flex items-center justify-between w-full px-4 py-3 rounded-lg bg-[var(--color-base-300)] text-[var(--color-base-content)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-all duration-200 shadow-sm hover:shadow-md border border-transparent hover:border-[var(--color-primary)]",
                            title: "Select Power Unit",
                            onclick: move |_| show_power_unit_dropdown.set(!show_power_unit_dropdown()),
                            span { class: "text-sm font-medium capitalize",
                                {if power_unit() == "watt" { "Watt (W)" } else { "Milliwatt (mW)" }}
                            }
                            svg {
                                class: if show_power_unit_dropdown() { "w-4 h-4 transform rotate-180 transition-transform duration-200" } else { "w-4 h-4 transition-transform duration-200" },
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M6 9l6 6 6-6" }
                            }
                        }

                        if show_power_unit_dropdown() {
                            div {
                                class: "fixed inset-0 z-40",
                                onclick: move |_| show_power_unit_dropdown.set(false),
                            }
                            div { class: "absolute left-0 right-0 mt-2 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-xl z-50 overflow-hidden",
                                button {
                                    class: if power_unit() == "watt" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        power_unit.set("watt".to_string());
                                        show_power_unit_dropdown.set(false);
                                    },
                                    "Watt (W)"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if power_unit() == "milliwatt" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        power_unit.set("milliwatt".to_string());
                                        show_power_unit_dropdown.set(false);
                                    },
                                    "Milliwatt (mW)"
                                }
                            }
                        }
                    }
                }
            }

            // Application Settings
            div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)] mb-6",
                h3 { class: "text-xl font-bold text-[var(--color-base-content)] mb-4",
                    "Application"
                }

                // Auto-start
                div { class: "flex items-center justify-between mb-4",
                    div {
                        label { class: "text-sm font-semibold text-[var(--color-base-content)]",
                            "Start on Boot"
                        }
                        p { class: "text-xs text-[var(--color-base-content)]/70",
                            "Launch Ryzone automatically when system starts"
                        }
                    }
                    input {
                        r#type: "checkbox",
                        class: "w-5 h-5 accent-[var(--color-primary)] cursor-pointer"
                    }
                }

                // Minimize to tray
                div { class: "flex items-center justify-between mb-4",
                    div {
                        label { class: "text-sm font-semibold text-[var(--color-base-content)]",
                            "Minimize to System Tray"
                        }
                        p { class: "text-xs text-[var(--color-base-content)]/70",
                            "Keep running in background when window is closed"
                        }
                    }
                    input {
                        r#type: "checkbox",
                        class: "w-5 h-5 accent-[var(--color-primary)] cursor-pointer"
                    }
                }

                // Update frequency
                div { class: "mb-4 relative",
                    label { class: "block text-sm font-semibold text-[var(--color-base-content)] mb-3",
                        "Update Frequency"
                    }
                    button {
                        class: "flex items-center justify-between w-full px-4 py-3 rounded-lg bg-[var(--color-base-300)] text-[var(--color-base-content)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-all duration-200 shadow-sm hover:shadow-md border border-transparent hover:border-[var(--color-primary)]",
                        title: "Select Update Frequency",
                        onclick: move |_| show_update_freq_dropdown.set(!show_update_freq_dropdown()),
                        span { class: "text-sm font-medium",
                            {match update_frequency().as_str() {
                                "500" => "0.5 seconds",
                                "1000" => "1 second",
                                "2000" => "2 seconds",
                                "5000" => "5 seconds",
                                _ => "1 second"
                            }}
                        }
                        svg {
                            class: if show_update_freq_dropdown() { "w-4 h-4 transform rotate-180 transition-transform duration-200" } else { "w-4 h-4 transition-transform duration-200" },
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M6 9l6 6 6-6" }
                        }
                    }

                    if show_update_freq_dropdown() {
                        div {
                            class: "fixed inset-0 z-40",
                            onclick: move |_| show_update_freq_dropdown.set(false),
                        }
                        div { class: "absolute left-0 right-0 mt-2 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-xl z-50 overflow-hidden",
                            button {
                                class: if update_frequency() == "500" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    update_frequency.set("500".to_string());
                                    show_update_freq_dropdown.set(false);
                                },
                                "0.5 seconds"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if update_frequency() == "1000" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    update_frequency.set("1000".to_string());
                                    show_update_freq_dropdown.set(false);
                                },
                                "1 second"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if update_frequency() == "2000" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    update_frequency.set("2000".to_string());
                                    show_update_freq_dropdown.set(false);
                                },
                                "2 seconds"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if update_frequency() == "5000" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    update_frequency.set("5000".to_string());
                                    show_update_freq_dropdown.set(false);
                                },
                                "5 seconds"
                            }
                        }
                    }
                }
            }

            // Action Buttons
            div { class: "flex gap-4 mt-8",
                button { class: "px-6 py-3 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                    "Save Settings"
                }
                button { class: "px-6 py-3 bg-[var(--color-base-300)] text-[var(--color-base-content)] rounded-lg font-semibold hover:bg-[var(--color-neutral)] transition-colors",
                    "Reset to Defaults"
                }
            }
        }
    }
}