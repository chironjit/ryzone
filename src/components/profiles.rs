use dioxus::prelude::*;
use crate::utils::types::{AppSettings, ProfileSettings};
use crate::utils::settings::write_profile_settings;
use crate::utils::conversions::{power_unit_label, temp_unit_label, power_conversion, temp_conversion};

#[component]
fn PowerInput(label: String, value_mw: Signal<i32>, power_unit: String) -> Element {
    let display_val = power_conversion(value_mw(), "milliwatt", &power_unit);
    let unit_label = power_unit_label(&power_unit);
    let pu = power_unit.clone();

    rsx! {
        div {
            label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                "{label} ({unit_label})"
            }
            input {
                r#type: "number",
                class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                value: "{display_val}",
                oninput: move |evt| {
                    let input_val: i32 = evt.value().parse().unwrap_or(0);
                    value_mw.set(power_conversion(input_val, &pu, "milliwatt"));
                }
            }
        }
    }
}

#[component]
fn TempInput(label: String, value_c: Signal<i32>, temp_unit: String) -> Element {
    let canonical_display = temp_conversion(value_c() as f32, "celsius", &temp_unit).round() as i32;
    let mut input_text = use_signal(|| canonical_display.to_string());
    let mut is_editing = use_signal(|| false);
    let canonical_display_text = canonical_display.to_string();
    let unit_label = temp_unit_label(&temp_unit);
    let tu = temp_unit.clone();
    let canonical_for_effect = canonical_display_text.clone();

    use_effect(move || {
        if !is_editing() {
            input_text.set(canonical_for_effect.clone());
        }
    });

    rsx! {
        div {
            label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                "{label} ({unit_label})"
            }
            input {
                r#type: "number",
                step: "1",
                class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                value: "{input_text()}",
                onfocus: move |_| {
                    is_editing.set(true);
                },
                oninput: move |evt| {
                    input_text.set(evt.value());
                },
                onblur: move |_| {
                    if let Ok(input_val) = input_text().trim().parse::<i32>() {
                        value_c.set(temp_conversion(input_val as f32, &tu, "celsius").round() as i32);
                    } else {
                        input_text.set(canonical_display_text.clone());
                    }
                    is_editing.set(false);
                }
            }
        }
    }
}

pub fn Profiles() -> Element {
    // Import context
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut profile = use_context::<Signal<ProfileSettings>>();

    // State for tracking which profile section is expanded (only one at a time)
    let mut expanded_section = use_signal(|| "".to_string());

    // Active profile
    let active_profile = profile().active_profile.clone();
    let mut low_batt_threshold_percent = use_signal(|| profile().low_batt_threshold_percent.clone());
    let power_unit = settings().units.power.clone();
    let temp_unit = settings().units.temp.clone();

    // System profile signals
    let mut sys_perf_fast_mw = use_signal(|| profile().system.performance.fast_mw.clone());
    let mut sys_perf_slow_mw = use_signal(|| profile().system.performance.slow_mw.clone());
    let mut sys_perf_stapm_mw = use_signal(|| profile().system.performance.stapm_mw.clone());
    let mut sys_perf_temp_c = use_signal(|| profile().system.performance.temp_c.clone());
    let mut sys_bal_fast_mw = use_signal(|| profile().system.balanced.fast_mw.clone());
    let mut sys_bal_slow_mw = use_signal(|| profile().system.balanced.slow_mw.clone());
    let mut sys_bal_stapm_mw = use_signal(|| profile().system.balanced.stapm_mw.clone());
    let mut sys_bal_temp_c = use_signal(|| profile().system.balanced.temp_c.clone());
    let mut sys_power_saver_fast_mw = use_signal(|| profile().system.power_saver.fast_mw.clone());
    let mut sys_power_saver_slow_mw = use_signal(|| profile().system.power_saver.slow_mw.clone());
    let mut sys_power_saver_stapm_mw = use_signal(|| profile().system.power_saver.stapm_mw.clone());
    let mut sys_power_saver_temp_c = use_signal(|| profile().system.power_saver.temp_c.clone());

    // Custom profile signals
    let mut custom_ac_fast_mw = use_signal(|| profile().custom.ac.fast_mw.clone());
    let mut custom_ac_slow_mw = use_signal(|| profile().custom.ac.slow_mw.clone());
    let mut custom_ac_stapm_mw = use_signal(|| profile().custom.ac.stapm_mw.clone());
    let mut custom_ac_temp_c = use_signal(|| profile().custom.ac.temp_c.clone());
    let mut custom_batt_fast_mw = use_signal(|| profile().custom.batt.fast_mw.clone());
    let mut custom_batt_slow_mw = use_signal(|| profile().custom.batt.slow_mw.clone());
    let mut custom_batt_stapm_mw = use_signal(|| profile().custom.batt.stapm_mw.clone());
    let mut custom_batt_temp_c = use_signal(|| profile().custom.batt.temp_c.clone());
    let mut custom_low_batt_fast_mw = use_signal(|| profile().custom.low_batt.fast_mw.clone());
    let mut custom_low_batt_slow_mw = use_signal(|| profile().custom.low_batt.slow_mw.clone());
    let mut custom_low_batt_stapm_mw = use_signal(|| profile().custom.low_batt.stapm_mw.clone());
    let mut custom_low_batt_temp_c = use_signal(|| profile().custom.low_batt.temp_c.clone());

    // Turbo profile signals
    let mut turbo_fast_mw = use_signal(|| profile().turbo.turbo.fast_mw.clone());
    let mut turbo_slow_mw = use_signal(|| profile().turbo.turbo.slow_mw.clone());
    let mut turbo_stapm_mw = use_signal(|| profile().turbo.turbo.stapm_mw.clone());
    let mut turbo_temp_c = use_signal(|| profile().turbo.turbo.temp_c.clone());

    // Fixed profile signals
    let mut fixed_fast_mw = use_signal(|| profile().fixed.fixed.fast_mw.clone());
    let mut fixed_slow_mw = use_signal(|| profile().fixed.fixed.slow_mw.clone());
    let mut fixed_stapm_mw = use_signal(|| profile().fixed.fixed.stapm_mw.clone());
    let mut fixed_temp_c = use_signal(|| profile().fixed.fixed.temp_c.clone());

    let mut save_system_profile = move || {
        let mut p = profile.write();
        p.system.performance.fast_mw = sys_perf_fast_mw();
        p.system.performance.slow_mw = sys_perf_slow_mw();
        p.system.performance.stapm_mw = sys_perf_stapm_mw();
        p.system.performance.temp_c = sys_perf_temp_c();
        p.system.balanced.fast_mw = sys_bal_fast_mw();
        p.system.balanced.slow_mw = sys_bal_slow_mw();
        p.system.balanced.stapm_mw = sys_bal_stapm_mw();
        p.system.balanced.temp_c = sys_bal_temp_c();
        p.system.power_saver.fast_mw = sys_power_saver_fast_mw();
        p.system.power_saver.slow_mw = sys_power_saver_slow_mw();
        p.system.power_saver.stapm_mw = sys_power_saver_stapm_mw();
        p.system.power_saver.temp_c = sys_power_saver_temp_c();
        drop(p);
        let _ = write_profile_settings(&profile());
    };

    let mut save_custom_profile = move || {
        let mut p = profile.write();
        p.custom.ac.fast_mw = custom_ac_fast_mw();
        p.custom.ac.slow_mw = custom_ac_slow_mw();
        p.custom.ac.stapm_mw = custom_ac_stapm_mw();
        p.custom.ac.temp_c = custom_ac_temp_c();
        p.custom.batt.fast_mw = custom_batt_fast_mw();
        p.custom.batt.slow_mw = custom_batt_slow_mw();
        p.custom.batt.stapm_mw = custom_batt_stapm_mw();
        p.custom.batt.temp_c = custom_batt_temp_c();
        p.custom.low_batt.fast_mw = custom_low_batt_fast_mw();
        p.custom.low_batt.slow_mw = custom_low_batt_slow_mw();
        p.custom.low_batt.stapm_mw = custom_low_batt_stapm_mw();
        p.custom.low_batt.temp_c = custom_low_batt_temp_c();
        p.low_batt_threshold_percent = low_batt_threshold_percent();
        drop(p);
        let _ = write_profile_settings(&profile());
    };

    let mut save_turbo_profile = move || {
        let mut p = profile.write();
        p.turbo.turbo.fast_mw = turbo_fast_mw();
        p.turbo.turbo.slow_mw = turbo_slow_mw();
        p.turbo.turbo.stapm_mw = turbo_stapm_mw();
        p.turbo.turbo.temp_c = turbo_temp_c();
        drop(p);
        let _ = write_profile_settings(&profile());
    };

    let mut save_fixed_profile = move || {
        let mut p = profile.write();
        p.fixed.fixed.fast_mw = fixed_fast_mw();
        p.fixed.fixed.slow_mw = fixed_slow_mw();
        p.fixed.fixed.stapm_mw = fixed_stapm_mw();
        p.fixed.fixed.temp_c = fixed_temp_c();
        drop(p);
        let _ = write_profile_settings(&profile());
    };

    let mut set_as_active_profile = move |new_active_profile: String| {
        profile.write().active_profile = new_active_profile;
        let _ = write_profile_settings(&profile());
    };



    rsx! {
        div { class: "p-8 max-w-[1600px] mx-auto",

            p { class: "text-[var(--color-base-content)]/70 mb-8",
                "Configure power profiles here to manage your system's power settings. Each profile allows you to set power limits and temperature thresholds. Select profile and set as active or choose a profile from the navbar"
            }

            // System Profiles Section
            div { class: if active_profile == "system" {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile == "system" {
                    div { class: "absolute top-4 right-16 px-3 py-1 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-full text-xs font-semibold z-10",
                        "Active"
                    }
                }

                button {
                    class: "w-full flex items-center justify-between p-4 hover:bg-[var(--color-base-300)]/50 transition-colors rounded-xl",
                    onclick: move |_| {
                        if expanded_section() == "system" {
                            expanded_section.set("".to_string());
                        } else {
                            expanded_section.set("system".to_string());
                        }
                    },

                    h3 { class: "text-xl font-bold text-[var(--color-base-content)]",
                        "System"
                    }

                    svg {
                        class: if expanded_section() == "system" { "w-6 h-6 transform rotate-180 transition-transform" } else { "w-6 h-6 transition-transform" },
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M19 9l-7 7-7-7" }
                    }
                }

                if expanded_section() == "system" {
                    div { class: "px-4 pb-4 space-y-3",

                        // Performance Sub-profile
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            h4 { class: "text-lg font-bold text-[var(--color-base-content)] mb-2",
                                "Performance"
                            }
                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Set power and temp limits for when the system \"Performance\" profile is selected on your operating system"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: sys_perf_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: sys_perf_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: sys_perf_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: sys_perf_temp_c, temp_unit: temp_unit.clone() }
                            }
                        }

                        // Balanced Sub-profile
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            h4 { class: "text-lg font-bold text-[var(--color-base-content)] mb-2",
                                "Balanced"
                            }
                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Set power and temp limits for when the system \"Balanced\" profile is selected on your operating system"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: sys_bal_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: sys_bal_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: sys_bal_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: sys_bal_temp_c, temp_unit: temp_unit.clone() }
                            }
                        }

                        // Power Saver Sub-profile
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            h4 { class: "text-lg font-bold text-[var(--color-base-content)] mb-2",
                                "Power Saver"
                            }
                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Set power and temp limits for when the system \"Power Saver\" profile is selected on your operating system"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: sys_power_saver_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: sys_power_saver_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: sys_power_saver_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: sys_power_saver_temp_c, temp_unit: temp_unit.clone() }
                            }


                        }

                        div { class: "flex gap-3 mt-4",
                            button {
                                class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| save_system_profile(),
                                "Save Changes"
                            }

                            button {
                                class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| {
                                    save_system_profile();
                                    set_as_active_profile("system".to_string());
                                },
                                "Set as Active"
                            }
                        }
                    }
                }
            }

            // Custom Profiles Section
            div { class: if active_profile == "custom" {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile == "custom" {
                    div { class: "absolute top-4 right-16 px-3 py-1 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-full text-xs font-semibold z-10",
                        "Active"
                    }
                }

                button {
                    class: "w-full flex items-center justify-between p-4 hover:bg-[var(--color-base-300)]/50 transition-colors rounded-xl",
                    onclick: move |_| {
                        if expanded_section() == "custom" {
                            expanded_section.set("".to_string());
                        } else {
                            expanded_section.set("custom".to_string());
                        }
                    },

                    h3 { class: "text-xl font-bold text-[var(--color-base-content)]",
                        "Custom"
                    }

                    svg {
                        class: if expanded_section() == "custom" { "w-6 h-6 transform rotate-180 transition-transform" } else { "w-6 h-6 transition-transform" },
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M19 9l-7 7-7-7" }
                    }
                }

                if expanded_section() == "custom" {
                    div { class: "px-4 pb-4 space-y-3",

                        // AC Sub-profile
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            h4 { class: "text-lg font-bold text-[var(--color-base-content)] mb-2",
                                "AC"
                            }
                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Set power and temp limits for when the Custom profile is active and laptop is in AC mode"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: custom_ac_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: custom_ac_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: custom_ac_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: custom_ac_temp_c, temp_unit: temp_unit.clone() }
                            }

                        }

                        // Battery Sub-profile
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            h4 { class: "text-lg font-bold text-[var(--color-base-content)] mb-2",
                                "Battery"
                            }
                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Set power and temp limits for when the Custom profile is active and laptop is in battery mode"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: custom_batt_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: custom_batt_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: custom_batt_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: custom_batt_temp_c, temp_unit: temp_unit.clone() }
                            }

                        }

                        

                        // Low Battery Sub-profile
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            h4 { class: "text-lg font-bold text-[var(--color-base-content)] mb-2",
                                "Low Battery"
                            }
                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Set power and temp limits for when the Custom profile is active and laptop is in battery mode below a specified thereshold"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4 mb-4",
                                PowerInput { label: "Fast Limit", value_mw: custom_low_batt_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: custom_low_batt_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: custom_low_batt_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: custom_low_batt_temp_c, temp_unit: temp_unit.clone() }
                            }

                            // Battery Level Threshold
                            div { class: "mb-4",
                                label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                    "Battery Level Threshold (%)"
                                }
                                input {
                                    r#type: "number",
                                    min: "0",
                                    max: "100",
                                    class: "w-full md:w-48 px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                    value: "{low_batt_threshold_percent}",
                                    oninput: move |evt| low_batt_threshold_percent.set(evt.value().parse().unwrap_or(0))
                                }
                                p { class: "text-xs text-[var(--color-base-content)]/60 mt-1",
                                    "Profile activates when battery drops below this level"
                                }
                            }

                        }

                        div { class: "flex gap-3 mt-4",
                            button {
                                class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| save_custom_profile(),
                                "Save Changes"
                            }

                            button {
                                class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| {
                                    save_custom_profile();
                                    set_as_active_profile("custom".to_string());
                                },
                                "Set as Active"
                            }
                        }
                    }
                }
            }

            // Turbo Profile
            div { class: if active_profile == "turbo" {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile == "turbo" {
                    div { class: "absolute top-4 right-16 px-3 py-1 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-full text-xs font-semibold z-10",
                        "Active"
                    }
                }

                button {
                    class: "w-full flex items-center justify-between p-4 hover:bg-[var(--color-base-300)]/50 transition-colors rounded-xl",
                    onclick: move |_| {
                        if expanded_section() == "turbo" {
                            expanded_section.set("".to_string());
                        } else {
                            expanded_section.set("turbo".to_string());
                        }
                    },

                    h3 { class: "text-xl font-bold text-[var(--color-base-content)]",
                        "Turbo"
                    }

                    svg {
                        class: if expanded_section() == "turbo" { "w-6 h-6 transform rotate-180 transition-transform" } else { "w-6 h-6 transition-transform" },
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M19 9l-7 7-7-7" }
                    }
                }

                if expanded_section() == "turbo" {
                    div { class: "px-4 pb-4",
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Maximum performance mode with highest power limits"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: turbo_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: turbo_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: turbo_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: turbo_temp_c, temp_unit: temp_unit.clone() }
                            }

                            div { class: "flex gap-3 mt-4",
                                // button {
                                //     class: "px-4 py-2 bg-red-600 text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity flex items-center gap-2",
                                //     // onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                //     svg {
                                //         xmlns: "http://www.w3.org/2000/svg",
                                //         width: "20",
                                //         height: "20",
                                //         view_box: "0 0 24 24",
                                //         fill: "none",
                                //         stroke: "currentColor",
                                //         stroke_width: "2",
                                //         stroke_linecap: "round",
                                //         stroke_linejoin: "round",
                                //         path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" }
                                //         path { d: "M12 9v4" }
                                //         path { d: "M12 17h.01" }
                                //     }
                                //     "Find Max Turbo"
                                // }

                                button {
                                    class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| save_turbo_profile(),
                                    "Save Changes"
                                }

                                button {
                                    class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| {
                                        save_turbo_profile();
                                        set_as_active_profile("turbo".to_string());
                                    },
                                    "Set as Active"
                                }
                            }
                        }
                    }
                }
            }

            // Fixed Profile
            div { class: if active_profile == "fixed" {
                "mb-8 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-8 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile == "fixed" {
                    div { class: "absolute top-4 right-16 px-3 py-1 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-full text-xs font-semibold z-10",
                        "Active"
                    }
                }

                button {
                    class: "w-full flex items-center justify-between p-4 hover:bg-[var(--color-base-300)]/50 transition-colors rounded-xl",
                    onclick: move |_| {
                        if expanded_section() == "fixed" {
                            expanded_section.set("".to_string());
                        } else {
                            expanded_section.set("fixed".to_string());
                        }
                    },

                    h3 { class: "text-xl font-bold text-[var(--color-base-content)]",
                        "Fixed"
                    }

                    svg {
                        class: if expanded_section() == "fixed" { "w-6 h-6 transform rotate-180 transition-transform" } else { "w-6 h-6 transition-transform" },
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        path { d: "M19 9l-7 7-7-7" }
                    }
                }

                if expanded_section() == "fixed" {
                    div { class: "px-4 pb-4",
                        div {
                            class: "bg-[var(--color-base-300)] rounded-lg p-5 border border-[var(--color-base-content)]/10",

                            p { class: "text-sm text-[var(--color-base-content)]/70 mb-4",
                                "Constant power delivery with fixed limits"
                            }

                            div { class: "grid grid-cols-2 md:grid-cols-4 gap-4",
                                PowerInput { label: "Fast Limit", value_mw: fixed_fast_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "Slow Limit", value_mw: fixed_slow_mw, power_unit: power_unit.clone() }
                                PowerInput { label: "STAPM Limit", value_mw: fixed_stapm_mw, power_unit: power_unit.clone() }
                                TempInput { label: "TCTL Temp Limit", value_c: fixed_temp_c, temp_unit: temp_unit.clone() }
                            }

                            div { class: "flex gap-3 mt-4",
                                button {
                                    class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| save_fixed_profile(),
                                    "Save Changes"
                                }

                                button {
                                    class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| {
                                        save_fixed_profile();
                                        set_as_active_profile("fixed".to_string());
                                    },
                                    "Set as Active"
                                }
                            }
                        }
                    }
                }
            }

        }
    }
}
