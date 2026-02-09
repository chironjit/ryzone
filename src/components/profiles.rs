use dioxus::prelude::*;

#[component]
pub fn Profiles() -> Element {
    // State for tracking which profile section is expanded (only one at a time)
    let mut expanded_section = use_signal(|| "".to_string());

    // State for active profile (this would normally come from app state)
    let mut active_profile = use_signal(|| "system".to_string());

    // State for profile parameters - System profiles
    let mut sys_perf_fast = use_signal(|| 25);
    let mut sys_perf_slow = use_signal(|| 20);
    let mut sys_perf_stapm = use_signal(|| 22);
    let mut sys_perf_tctl = use_signal(|| 95);

    let mut sys_bal_fast = use_signal(|| 18);
    let mut sys_bal_slow = use_signal(|| 15);
    let mut sys_bal_stapm = use_signal(|| 16);
    let mut sys_bal_tctl = use_signal(|| 85);

    let mut sys_power_fast = use_signal(|| 12);
    let mut sys_power_slow = use_signal(|| 10);
    let mut sys_power_stapm = use_signal(|| 11);
    let mut sys_power_tctl = use_signal(|| 75);

    // State for profile parameters - Custom profiles
    let mut custom_bat_fast = use_signal(|| 15);
    let mut custom_bat_slow = use_signal(|| 12);
    let mut custom_bat_stapm = use_signal(|| 13);
    let mut custom_bat_tctl = use_signal(|| 80);

    let mut custom_ac_fast = use_signal(|| 28);
    let mut custom_ac_slow = use_signal(|| 25);
    let mut custom_ac_stapm = use_signal(|| 26);
    let mut custom_ac_tctl = use_signal(|| 90);

    let mut custom_lowbat_fast = use_signal(|| 10);
    let mut custom_lowbat_slow = use_signal(|| 8);
    let mut custom_lowbat_stapm = use_signal(|| 9);
    let mut custom_lowbat_tctl = use_signal(|| 70);
    let mut custom_lowbat_level = use_signal(|| 20);

    // State for profile parameters - Turbo
    let mut turbo_fast = use_signal(|| 35);
    let mut turbo_slow = use_signal(|| 30);
    let mut turbo_stapm = use_signal(|| 32);
    let mut turbo_tctl = use_signal(|| 100);

    // State for profile parameters - Fixed
    let mut fixed_fast = use_signal(|| 20);
    let mut fixed_slow = use_signal(|| 20);
    let mut fixed_stapm = use_signal(|| 20);
    let mut fixed_tctl = use_signal(|| 85);

    rsx! {
        div { class: "p-8 max-w-[1600px] mx-auto",

            p { class: "text-[var(--color-base-content)]/70 mb-8",
                "Configure power profiles here to manage your system's power settings. Each profile allows you to set power limits and temperature thresholds. Select profile and set as active or choose a profile from the navbar"
            }

            // System Profiles Section
            div { class: if active_profile().starts_with("system") {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile().starts_with("system") {
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
                                // Fast Limit
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_perf_fast}",
                                        oninput: move |evt| sys_perf_fast.set(evt.value().parse().unwrap_or(25))
                                    }
                                }

                                // Slow Limit
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_perf_slow}",
                                        oninput: move |evt| sys_perf_slow.set(evt.value().parse().unwrap_or(20))
                                    }
                                }

                                // STAPM Limit
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_perf_stapm}",
                                        oninput: move |evt| sys_perf_stapm.set(evt.value().parse().unwrap_or(22))
                                    }
                                }

                                // TCTL Temp Limit
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_perf_tctl}",
                                        oninput: move |evt| sys_perf_tctl.set(evt.value().parse().unwrap_or(95))
                                    }
                                }
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_bal_fast}",
                                        oninput: move |evt| sys_bal_fast.set(evt.value().parse().unwrap_or(18))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_bal_slow}",
                                        oninput: move |evt| sys_bal_slow.set(evt.value().parse().unwrap_or(15))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_bal_stapm}",
                                        oninput: move |evt| sys_bal_stapm.set(evt.value().parse().unwrap_or(16))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_bal_tctl}",
                                        oninput: move |evt| sys_bal_tctl.set(evt.value().parse().unwrap_or(85))
                                    }
                                }
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_power_fast}",
                                        oninput: move |evt| sys_power_fast.set(evt.value().parse().unwrap_or(12))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_power_slow}",
                                        oninput: move |evt| sys_power_slow.set(evt.value().parse().unwrap_or(10))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_power_stapm}",
                                        oninput: move |evt| sys_power_stapm.set(evt.value().parse().unwrap_or(11))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{sys_power_tctl}",
                                        oninput: move |evt| sys_power_tctl.set(evt.value().parse().unwrap_or(75))
                                    }
                                }
                            }


                        }

                        div { class: "flex gap-3 mt-4",
                            button {
                                class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                "Save Changes"
                            }

                            button {
                                class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                "Set as Active"
                            }
                        }
                    }
                }
            }

            // Custom Profiles Section
            div { class: if active_profile().starts_with("custom") {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile().starts_with("custom") {
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_ac_fast}",
                                        oninput: move |evt| custom_ac_fast.set(evt.value().parse().unwrap_or(28))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_ac_slow}",
                                        oninput: move |evt| custom_ac_slow.set(evt.value().parse().unwrap_or(25))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_ac_stapm}",
                                        oninput: move |evt| custom_ac_stapm.set(evt.value().parse().unwrap_or(26))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_ac_tctl}",
                                        oninput: move |evt| custom_ac_tctl.set(evt.value().parse().unwrap_or(90))
                                    }
                                }
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_bat_fast}",
                                        oninput: move |evt| custom_bat_fast.set(evt.value().parse().unwrap_or(15))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_bat_slow}",
                                        oninput: move |evt| custom_bat_slow.set(evt.value().parse().unwrap_or(12))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_bat_stapm}",
                                        oninput: move |evt| custom_bat_stapm.set(evt.value().parse().unwrap_or(13))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_bat_tctl}",
                                        oninput: move |evt| custom_bat_tctl.set(evt.value().parse().unwrap_or(80))
                                    }
                                }
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_lowbat_fast}",
                                        oninput: move |evt| custom_lowbat_fast.set(evt.value().parse().unwrap_or(10))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_lowbat_slow}",
                                        oninput: move |evt| custom_lowbat_slow.set(evt.value().parse().unwrap_or(8))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_lowbat_stapm}",
                                        oninput: move |evt| custom_lowbat_stapm.set(evt.value().parse().unwrap_or(9))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{custom_lowbat_tctl}",
                                        oninput: move |evt| custom_lowbat_tctl.set(evt.value().parse().unwrap_or(70))
                                    }
                                }
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
                                    value: "{custom_lowbat_level}",
                                    oninput: move |evt| custom_lowbat_level.set(evt.value().parse().unwrap_or(20))
                                }
                                p { class: "text-xs text-[var(--color-base-content)]/60 mt-1",
                                    "Profile activates when battery drops below this level"
                                }
                            }

                        }

                        div { class: "flex gap-3 mt-4",
                            button {
                                class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                "Save Changes"
                            }

                            button {
                                class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                "Set as Active"
                            }
                        }
                    }
                }
            }

            // Turbo Profile
            div { class: if active_profile() == "turbo" {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-4 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile() == "turbo" {
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{turbo_fast}",
                                        oninput: move |evt| turbo_fast.set(evt.value().parse().unwrap_or(35))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{turbo_slow}",
                                        oninput: move |evt| turbo_slow.set(evt.value().parse().unwrap_or(30))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{turbo_stapm}",
                                        oninput: move |evt| turbo_stapm.set(evt.value().parse().unwrap_or(32))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{turbo_tctl}",
                                        oninput: move |evt| turbo_tctl.set(evt.value().parse().unwrap_or(100))
                                    }
                                }
                            }

                            div { class: "flex gap-3 mt-4",
                                button {
                                    class: "px-4 py-2 bg-red-600 text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity flex items-center gap-2",
                                    onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "20",
                                        height: "20",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3" }
                                        path { d: "M12 9v4" }
                                        path { d: "M12 17h.01" }
                                    }
                                    "Find Max Turbo"
                                }

                                button {
                                    class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                    "Save Changes"
                                }

                                button {
                                    class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                    "Set as Active"
                                }
                            }
                        }
                    }
                }
            }

            // Fixed Profile
            div { class: if active_profile() == "fixed" {
                "mb-8 bg-[var(--color-base-200)] rounded-xl border-2 border-[var(--color-primary)] relative"
            } else {
                "mb-8 bg-[var(--color-base-200)] rounded-xl border border-[var(--color-base-300)]"
            },

                if active_profile() == "fixed" {
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
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Fast Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{fixed_fast}",
                                        oninput: move |evt| fixed_fast.set(evt.value().parse().unwrap_or(20))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "Slow Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{fixed_slow}",
                                        oninput: move |evt| fixed_slow.set(evt.value().parse().unwrap_or(20))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "STAPM Limit (W)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{fixed_stapm}",
                                        oninput: move |evt| fixed_stapm.set(evt.value().parse().unwrap_or(20))
                                    }
                                }
                                div {
                                    label { class: "block text-sm font-medium text-[var(--color-base-content)]/70 mb-2",
                                        "TCTL Temp Limit (°C)"
                                    }
                                    input {
                                        r#type: "number",
                                        class: "w-full px-3 py-2 bg-[var(--color-base-300)] border border-[var(--color-base-content)]/20 rounded-lg text-[var(--color-base-content)] focus:outline-none focus:border-[var(--color-primary)]",
                                        value: "{fixed_tctl}",
                                        oninput: move |evt| fixed_tctl.set(evt.value().parse().unwrap_or(85))
                                    }
                                }
                            }

                            div { class: "flex gap-3 mt-4",
                                button {
                                    class: "px-4 py-2 bg-[var(--color-secondary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| active_profile.set("system_powersaver".to_string()),
                                    "Save Changes"
                                }

                                button {
                                    class: "px-4 py-2 bg-[var(--color-primary)] text-[var(--color-primary-content)] rounded-lg font-semibold hover:opacity-90 transition-opacity",
                                    onclick: move |_| active_profile.set("system_powersaver".to_string()),
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
