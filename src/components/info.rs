use dioxus::prelude::*;

#[component]
pub fn Info() -> Element {
    let mut notice_expanded = use_signal(|| true);

    rsx! {
        div { class: "p-8 max-w-[1600px] mx-auto",

            // Note Section
            div { class: "bg-[var(--color-warning)]/10 border-2 border-[var(--color-warning)] rounded-xl mb-6 overflow-hidden",
            // Header - clickable
            button {
                class: "w-full flex items-center justify-between p-8 hover:bg-[var(--color-warning)]/20 transition-colors",
                onclick: move |_| notice_expanded.set(!notice_expanded()),

                div { class: "text-2xl font-bold text-[var(--color-warning)]",
                    "Important Notice"
                }

                // Chevron icon
                svg {
                    class: if notice_expanded() { "w-6 h-6 stroke-[var(--color-warning)] transform rotate-180 transition-transform" } else { "w-6 h-6 stroke-[var(--color-warning)] transition-transform" },
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke_width: "2",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }

            // Content - collapsible
            if notice_expanded() {
                div { class: "px-8 pb-8",
                    div { class: "space-y-4",
                        div { class: "flex gap-3",
                            div { class: "flex-shrink-0 mt-1",
                                svg {
                                    class: "w-5 h-5 stroke-[var(--color-warning)]",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                                    line { x1: "12", y1: "9", x2: "12", y2: "13" }
                                    line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
                                }
                            }
                            div { class: "text-sm text-[var(--color-base-content)]/80",
                                "Please note that your use of this software is at your own risk. Neither the owner of this app nor the providers of the underlying software can guarantee that this will work or that it will not harm your system."
                            }
                        }

                        div { class: "flex gap-3",
                            div { class: "flex-shrink-0 mt-1",
                                svg {
                                    class: "w-5 h-5 stroke-[var(--color-warning)]",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                                    line { x1: "12", y1: "9", x2: "12", y2: "13" }
                                    line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
                                }
                            }
                            div { class: "text-sm text-[var(--color-base-content)]/80",
                                "While this has been tested on some AMD systems, it may not work on every system as there may be locks in place from your own hardware vendor."
                            }
                        }

                        div { class: "flex gap-3",
                            div { class: "flex-shrink-0 mt-1",
                                svg {
                                    class: "w-5 h-5 stroke-[var(--color-warning)]",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                                    line { x1: "12", y1: "9", x2: "12", y2: "13" }
                                    line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
                                }
                            }
                            div { class: "text-sm text-[var(--color-base-content)]/80",
                                "Most systems in general have limits to prevent damage, but these limits may not be available on your system or may fail to prevent damage."
                            }
                        }

                        div { class: "flex gap-3",
                            div { class: "flex-shrink-0 mt-1",
                                svg {
                                    class: "w-5 h-5 stroke-[var(--color-warning)]",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke_width: "2",
                                    path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                                    line { x1: "12", y1: "9", x2: "12", y2: "13" }
                                    line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
                                }
                            }
                            div { class: "text-sm text-[var(--color-base-content)]/80",
                                "AMD is the copyright owner of AMD, Ryzen and associated brands. AMD was not involved in any form in the production of this software, and no part of this software implies any form of support or approval of use of this software on their systems."
                            }
                        }
                    }
                }
            }
        }
           

            // Guide Section
            div { class: "bg-[var(--color-base-200)] rounded-xl p-8 border border-[var(--color-base-300)] mb-6",
                div { class: "text-2xl font-bold text-[var(--color-primary)] mb-6",
                    "Guide"
                }

                div { class: "space-y-2",
                    GuideItem {
                        title: "Ryzone App",
                        content: rsx! {
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "Use this app to monitor your Ryzen processor / SOC power draw, battery and change power profiles."
                            }
                            div { class: "text-sm text-[var(--color-primary)]/70",
                                "The app is only used to view / change settings. Once settings are applied, this app can be saftely closed"
                            }
                        }
                    }

                    GuideItem {
                        title: "Daemon",
                        content: rsx! {
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "All implementation and tracking of the settings are done via the Ryzoned systemd background daemon. The Ryzoned daemon works in the background and is set to start on log in. You do not need to keep this app open to have the settings work"
                            }
                        }
                    }

                    GuideItem {
                        title: "Profiles",
                        content: rsx! {
                            div { class: "text-sm text-[var(--color-base-content)]/70",
                                "Use the profiles section to add custom power and temperature limits to each profile / sub-profile. You can quickly select your active profile from the dropdown in the nav bar"
                            }
                        }
                    }

                    GuideItem {
                        title: "Slow and Fast Limit",
                        content: rsx! {
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "The Fast Limit is the maximum instantaneous power the processor can draw, allowing the processor to ramp up to complete the task as fast as possible. The Slow Limit is the maximum power the processor can draw over a specific longer duration(this duration is controllled my the AMD driver / processor). The Slow Limit should be less than the Fast Limit"
                            }
                            div { class: "text-sm text-[var(--color-warning)]/70",
                                "The Slow Limit should be less than the Fast Limit"
                            }
                        }
                    }

                    GuideItem {
                        title: "STAPM Limit",
                        content: rsx! {
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "The Skin Temperature Aware Power Management (STAPM) Limit is a limit designed to manage the power output of the laptop over a long duration to prevent the external temperature of the laptop exceeding a point where it is too hot to comfortable use (for example on your lap)."
                            }
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "This is usually controlled by the processor / driver and cannot be changed. In some systems, the starting limit is usually the Fast Limit, with the system throtlling this limit over time. In systems where this change is allowed, the system may still override this this value. Forcing a reset of the STAPM time and limit can force this value to be constantly high, but it is not recommended to do so."
                            }
                            div { class: "text-sm text-[var(--color-warning)]/70",
                                "Ryzone will only allow attempting to set the STAPM as high as the Fast Limit, where the STAPM Limit is not locked."
                            }
                        }
                    }

                    GuideItem {
                        title: "Temperature Limit",
                        content: rsx! {
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "The temperature limit is the maximum processor temperature allowed. If this threshold is hit, the processor will throttle even if the power limits are not hit"
                            }
                            div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                                "The minimum limit value allowed by Ryzone is 30 and the maximum limit value allowed is 100"
                            }
                            div { class: "text-sm text-[var(--color-warning)]/70",
                                "Please note that the temp limit may be temporarily exceeded during burstly workloads, and thus may at times be higher than the set limit"
                            }
                        }
                    }
                }
            }

            // Credits Section
            div { class: "bg-[var(--color-base-200)] rounded-xl p-8 border border-[var(--color-base-300)]",
                div { class: "text-2xl font-bold text-[var(--color-primary)] mb-6",
                    "Credits"
                }

                div { class: "text-sm text-[var(--color-base-content)]/80 mb-6",
                    "This application is made possible by the following open-source projects and their contributors:"
                }

                div { class: "space-y-4",
                    // RyzenAdj
                    div { 
                        class: "font-semibold text-base mb-1", 
                        a { 
                            href: "https://github.com/FlyGoat/RyzenAdj",
                            class: "underline hover:text-[var(--color-secondary)]",
                            "RyzenAdj"
                        }
                    }
                    div { class: "text-sm text-[var(--color-base-content)]/70",
                        "The C Library that is used to get and set all the Ryzen processor settings"
                    }

                    // Libryzenadj-rs
                    div { 
                        class: "font-semibold text-base mb-1", 
                        a { 
                            href: "https://gitlab.com/dragonn/libryzenadj-rs/",
                            class: "underline hover:text-[var(--color-secondary)]",
                            "Libryzenadj-rs"
                        }
                    }
                    div { class: "text-sm text-[var(--color-base-content)]/70",
                        "Rust bindings for RyzenAdj"
                    }

                }

                // Footer note
                div { class: "mt-8 pt-6 border-t border-[var(--color-base-300)] text-center text-sm text-[var(--color-base-content)]/60",
                    "Built using Rust, Dioxus & TailwindCSS"
                }
            }
        }
    }
}

#[component]
fn GuideItem(title: String, content: Element) -> Element {
    let mut is_expanded = use_signal(|| false);

    rsx! {
        div { class: "border border-[var(--color-base-300)] rounded-lg overflow-hidden",
            // Header - clickable
            button {
                class: "w-full flex items-center justify-between p-4 hover:bg-[var(--color-base-300)]/30 transition-colors",
                onclick: move |_| is_expanded.set(!is_expanded()),

                div { class: "font-semibold text-base", "{title}" }

                // Chevron icon
                svg {
                    class: if is_expanded() { "w-5 h-5 stroke-[var(--color-base-content)] transform rotate-180 transition-transform" } else { "w-5 h-5 stroke-[var(--color-base-content)] transition-transform" },
                    view_box: "0 0 24 24",
                    fill: "none",
                    stroke_width: "2",
                    polyline { points: "6 9 12 15 18 9" }
                }
            }

            // Content - collapsible
            if is_expanded() {
                div { class: "px-4 pb-4 border-t border-[var(--color-base-300)]",
                    div { class: "pt-4",
                        {content}
                    }
                }
            }
        }
    }
}
