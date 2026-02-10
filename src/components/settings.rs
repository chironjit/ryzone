use dioxus::prelude::*;
use crate::utils::types::AppSettings;
use crate::utils::settings::write_app_settings;


#[component]
pub fn Settings() -> Element {
    // App state context
    let mut settings = use_context::<Signal<AppSettings>>();
    let theme_mode = settings().style.theme_mode.clone();
    let theme_light_palette = settings().style.theme_light_palette.clone();
    let theme_dark_palette = settings().style.theme_dark_palette.clone();
    let power_unit = settings().units.power.clone();
    let temp_unit = settings().units.temp.clone();
    let start_on_login = settings().app.start_on_login.clone();
    let minimize_to_tray = settings().app.minimize_to_tray.clone();
    let enable_logging = settings().app.enable_logging.clone();
    let update_frequency = settings().app.update_frequency.clone();
    let logging_frequency = settings().app.logging_frequency.clone();

    // Signals that only are used in this component
    let mut show_theme_mode_dropdown = use_signal(|| false);
    let mut show_light_palette_dropdown = use_signal(|| false);
    let mut show_dark_palette_dropdown = use_signal(|| false);
    let mut show_temp_unit_dropdown = use_signal(|| false);
    let mut show_power_unit_dropdown = use_signal(|| false);
    let mut show_update_freq_dropdown = use_signal(|| false);
    let mut show_logging_freq_dropdown = use_signal(|| false);

    
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
                                "{theme_light_palette}"
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
                                    class: if theme_light_palette == "winter" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().style.theme_light_palette = "winter".to_string();
                                        let _ = write_app_settings(&settings());
                                        show_light_palette_dropdown.set(false);
                                    },
                                    "Winter"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_light_palette == "black" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().style.theme_light_palette = "black".to_string();
                                        let _ = write_app_settings(&settings());
                                        show_light_palette_dropdown.set(false);
                                    },
                                    "Black"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_light_palette == "nord" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().style.theme_light_palette = "nord".to_string();
                                        let _ = write_app_settings(&settings());
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
                                "{theme_dark_palette}"
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
                                    class: if theme_dark_palette == "dracula" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().style.theme_dark_palette = "dracula".to_string();
                                        let _ = write_app_settings(&settings());
                                        show_dark_palette_dropdown.set(false);
                                    },
                                    "Dracula"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_dark_palette == "night" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().style.theme_dark_palette = "night".to_string();
                                        let _ = write_app_settings(&settings());
                                        show_dark_palette_dropdown.set(false);
                                    },
                                    "Night"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if theme_dark_palette == "dim" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().style.theme_dark_palette = "dim".to_string();
                                        let _ = write_app_settings(&settings());
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
                                {if temp_unit == "celsius" { "Celsius (°C)" } else { "Fahrenheit (°F)" }}
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
                                    class: if temp_unit == "celsius" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().units.temp = "celsius".to_string();
                                        let _ = write_app_settings(&settings());
                                        show_temp_unit_dropdown.set(false);
                                    },
                                    "Celsius (°C)"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if temp_unit == "fahrenheit" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().units.temp = "fahrenheit".to_string();
                                        let _ = write_app_settings(&settings());
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
                            span { class: "text-sm font-medium",
                                {if power_unit == "watt" { "Watt (W)" } else { "Milliwatt (mW)" }}
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
                                    class: if power_unit == "watt" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().units.power = "watt".to_string();
                                        let _ = write_app_settings(&settings());
                                        show_power_unit_dropdown.set(false);
                                    },
                                    "Watt (W)"
                                }
                                div { class: "border-t border-[var(--color-base-300)]" }
                                button {
                                    class: if power_unit == "milliwatt" { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                    onclick: move |_| {
                                        settings.write().units.power = "milliwatt".to_string();
                                        let _ = write_app_settings(&settings());
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
                            "Start on Login"
                        }
                        p { class: "text-xs text-[var(--color-base-content)]/70",
                            "Launch Ryzone automatically when system starts"
                        }
                    }
                    input {
                        r#type: "checkbox",
                        class: "w-5 h-5 appearance-none bg-[var(--color-base-100)] border-2 border-[var(--color-base-300)] rounded cursor-pointer checked:bg-[var(--color-primary)] checked:border-[var(--color-primary)] checked:bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTIiIGhlaWdodD0iOSIgdmlld0JveD0iMCAwIDEyIDkiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PHBhdGggZD0iTTEgNEw0LjUgNy41TDExIDEiIHN0cm9rZT0id2hpdGUiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIi8+PC9zdmc+')] checked:bg-center checked:bg-no-repeat",
                        checked: start_on_login,
                        onchange: move |_| {
                            settings.write().app.start_on_login = !settings().app.start_on_login;
                            let _ = write_app_settings(&settings());
                        },
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
                        class: "w-5 h-5 appearance-none bg-[var(--color-base-100)] border-2 border-[var(--color-base-300)] rounded cursor-pointer checked:bg-[var(--color-primary)] checked:border-[var(--color-primary)] checked:bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTIiIGhlaWdodD0iOSIgdmlld0JveD0iMCAwIDEyIDkiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PHBhdGggZD0iTTEgNEw0LjUgNy41TDExIDEiIHN0cm9rZT0id2hpdGUiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIi8+PC9zdmc+')] checked:bg-center checked:bg-no-repeat",
                        checked: minimize_to_tray,
                        onchange: move |_| {
                            settings.write().app.minimize_to_tray = !settings().app.minimize_to_tray;
                            let _ = write_app_settings(&settings());
                        },
                    }
                }

                // Enable logging
                div { class: "flex items-center justify-between mb-4",
                    div {
                        label { class: "text-sm font-semibold text-[var(--color-base-content)]",
                            "Enable logging"
                        }
                        p { class: "text-xs text-[var(--color-base-content)]/70",
                            "Store log files in the /home/<user>/.ryzone/logs folder for future reference"
                        }
                    }
                    input {
                        r#type: "checkbox",
                        class: "w-5 h-5 appearance-none bg-[var(--color-base-100)] border-2 border-[var(--color-base-300)] rounded cursor-pointer checked:bg-[var(--color-primary)] checked:border-[var(--color-primary)] checked:bg-[url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTIiIGhlaWdodD0iOSIgdmlld0JveD0iMCAwIDEyIDkiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+PHBhdGggZD0iTTEgNEw0LjUgNy41TDExIDEiIHN0cm9rZT0id2hpdGUiIHN0cm9rZS13aWR0aD0iMiIgc3Ryb2tlLWxpbmVjYXA9InJvdW5kIiBzdHJva2UtbGluZWpvaW49InJvdW5kIi8+PC9zdmc+')] checked:bg-center checked:bg-no-repeat",
                        checked: enable_logging,
                        onchange: move |_| {
                            settings.write().app.enable_logging = !settings().app.enable_logging;
                            let _ = write_app_settings(&settings());
                        },
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
                            {match update_frequency {
                                1000 => "1 second",
                                5000 => "5 seconds",
                                10000 => "10 seconds",
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
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if update_frequency == 1000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.update_frequency = 1000;
                                    let _ = write_app_settings(&settings());
                                    show_update_freq_dropdown.set(false);
                                },
                                "1 second"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if update_frequency == 5000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.update_frequency = 5000;
                                    let _ = write_app_settings(&settings());
                                    show_update_freq_dropdown.set(false);
                                },
                                "5 seconds"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if update_frequency == 10000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.update_frequency = 10000;
                                    let _ = write_app_settings(&settings());
                                    show_update_freq_dropdown.set(false);
                                },
                                "10 seconds"
                            }
                        }
                    }
                }

                // Logging frequency
                div { class: "mb-4 relative",
                    label { class: "block text-sm font-semibold text-[var(--color-base-content)] mb-3",
                        "Logging Frequency"
                    }
                    button {
                        class: "flex items-center justify-between w-full px-4 py-3 rounded-lg bg-[var(--color-base-300)] text-[var(--color-base-content)] hover:bg-[var(--color-primary)] hover:text-[var(--color-neutral)] transition-all duration-200 shadow-sm hover:shadow-md border border-transparent hover:border-[var(--color-primary)]",
                        title: "Select Logging Frequency",
                        onclick: move |_| show_logging_freq_dropdown.set(!show_logging_freq_dropdown()),
                        span { class: "text-sm font-medium",
                            {match logging_frequency {
                                1000 => "1 seconds",
                                5000 => "5 seconds",
                                10000 => "10 seconds",
                                30000 => "30 seconds",
                                60000 => "1 minute",
                                _ => "10 seconds"
                            }}
                        }
                        svg {
                            class: if show_logging_freq_dropdown() { "w-4 h-4 transform rotate-180 transition-transform duration-200" } else { "w-4 h-4 transition-transform duration-200" },
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path { d: "M6 9l6 6 6-6" }
                        }
                    }

                    if show_logging_freq_dropdown() {
                        div {
                            class: "fixed inset-0 z-40",
                            onclick: move |_| show_logging_freq_dropdown.set(false),
                        }
                        div { class: "absolute left-0 right-0 mt-2 bg-[var(--color-base-200)] border border-[var(--color-base-300)] rounded-lg shadow-xl z-50 overflow-hidden",
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if logging_frequency == 1000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.logging_frequency = 1000;
                                    let _ = write_app_settings(&settings());
                                    show_logging_freq_dropdown.set(false);
                                },
                                "1 second"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if logging_frequency == 5000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.logging_frequency = 5000;
                                    let _ = write_app_settings(&settings());
                                    show_logging_freq_dropdown.set(false);
                                },
                                "5 seconds"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if logging_frequency == 10000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.logging_frequency = 10000;
                                    let _ = write_app_settings(&settings());
                                    show_logging_freq_dropdown.set(false);
                                },
                                "10 seconds"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if logging_frequency == 30000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.logging_frequency = 30000;
                                    let _ = write_app_settings(&settings());
                                    show_logging_freq_dropdown.set(false);
                                },
                                "30 seconds"
                            }
                            div { class: "border-t border-[var(--color-base-300)]" }
                            button {
                                class: if logging_frequency == 60000 { "w-full text-left px-4 py-3 bg-[var(--color-primary)] text-[var(--color-neutral)] font-medium transition-all duration-150" } else { "w-full text-left px-4 py-3 hover:bg-[var(--color-base-300)] text-[var(--color-base-content)] transition-all duration-150" },
                                onclick: move |_| {
                                    settings.write().app.logging_frequency = 60000;
                                    let _ = write_app_settings(&settings());
                                    show_logging_freq_dropdown.set(false);
                                },
                                "1 minute"
                            }
                        }
                    }
                }
            }

            // Action Buttons
            // div { class: "flex gap-4 mt-8",
            //     button { class: "px-6 py-3 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
            //         "Save Settings"
            //     }
            //     button { class: "px-6 py-3 bg-[var(--color-base-300)] text-[var(--color-base-content)] rounded-lg font-semibold hover:bg-[var(--color-neutral)] transition-colors",
            //         "Reset to Defaults"
            //     }
            // }
        }
    }
}