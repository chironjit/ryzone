use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div { class: "p-8 max-w-[1600px] mx-auto",

            // Stats Grid - Row 1
            div { class: "grid grid-cols-3 gap-6 mb-6",
                // CPU Card
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "CPU"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                        "Frequency"
                    }
                    div { class: "text-4xl font-bold text-[var(--color-primary)] mb-4",
                        "4.2 GHz"
                    }
                    div { class: "space-y-2 text-sm text-[var(--color-base-content)]/70",
                        div { "Temperature: 62 °C" }
                        div { "Load: 45%" }
                    }
                    // Progress bar
                    div { class: "mt-4",
                        div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                            div { class: "h-full bg-[var(--color-success)] rounded-full", style: "width: 20%" }
                        }
                    }
                }

                // GPU Card
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "GPU"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                        "Frequency"
                    }
                    div { class: "text-4xl font-bold text-[var(--color-primary)] mb-4",
                        "1.8 GHz"
                    }
                    div { class: "space-y-2 text-sm text-[var(--color-base-content)]/70",
                        div { "Temperature: 58 °C" }
                        div { "Load: 72 %" }
                    }
                    div { class: "mt-4",
                        div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                            div { class: "h-full bg-[var(--color-warning)] rounded-full", style: "width: 50%" }
                        }
                    }
                }

                // SOC Card
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "POWER"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                        "Load"
                    }
                    div { class: "text-4xl font-bold text-[var(--color-primary)] mb-4",
                        "0 W"
                    }
                    div { class: "space-y-2 text-sm text-[var(--color-base-content)]/70",
                        div { "Status: Charging" }
                        div { "Charge: 80 %" }
                    }
                    div { class: "mt-4",
                        div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                            div { class: "h-full bg-[var(--color-error)] rounded-full", style: "width: 90%" }
                        }
                    }
                }
            }

            // Stats Grid - Row 2
            div { class: "grid grid-cols-3 gap-6 mb-8",
                // Fast Limit
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "FAST LIMIT"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                        "Current Value:"
                    }
                    div { class: "text-2xl font-bold text-[var(--color-primary)] mb-2",
                        "15 W"
                    }
                    div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                        "Limit: 25 W"
                    }
                    div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                        div { class: "h-full bg-[var(--color-success)] rounded-full", style: "width: 77%" }
                    }
                }

                // Slow Limit
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "SLOW LIMIT"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                        "Current Value"
                    }
                    div { class: "text-2xl font-bold text-[var(--color-primary)] mb-2",
                        "15 W"
                    }
                    div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                        "Limit: 25 W"
                    }
                    div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                        div { class: "h-full bg-[var(--color-warning)] rounded-full", style: "width: 90%" }
                    }
                }

                // STAPM Limit
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "STAPM LIMIT"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70 mb-1",
                        "Current Value"
                    }
                    div { class: "text-2xl font-bold text-[var(--color-primary)] mb-2",
                        "15 W"
                    }
                    div { class: "text-sm text-[var(--color-base-content)]/70 mb-2",
                        "Limit: 25 W"
                    }
                    div { class: "w-full h-2 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                        div { class: "h-full bg-[var(--color-error)] rounded-full", style: "width: 77%" }
                    }
                }
            }
        }
    }
}
