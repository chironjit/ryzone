use dioxus::prelude::*;

#[component]
pub fn Battery() -> Element {
    rsx! {
        div { class: "p-8 max-w-[1600px] mx-auto",

            // Main Battery Status Card
            div { class: "bg-[var(--color-base-200)] rounded-xl p-8 border border-[var(--color-base-300)] mb-6",
                div { class: "flex items-center justify-between mb-6",
                    div {
                        div { class: "text-lg font-semibold text-[var(--color-secondary)] mb-1",
                            "Battery Status"
                        }
                        div { class: "text-sm text-[var(--color-base-content)]/70",
                            "Health: Good"
                        }
                    }
                    div { class: "text-right",
                        div { class: "text-5xl font-bold text-[var(--color-primary)]",
                            "85%"
                        }
                    }
                }

                // Battery charge progress bar
                div { class: "mb-4",
                    div { class: "w-full h-4 bg-[var(--color-base-300)] rounded-full overflow-hidden",
                        div { class: "h-full bg-[var(--color-success)] rounded-full transition-all", style: "width: 85%" }
                    }
                }

                // Status indicators
                div { class: "grid grid-cols-3 gap-4 text-sm",
                    div {
                        div { class: "text-[var(--color-base-content)]/70 mb-1", "Status" }
                        div { class: "font-semibold", "Charging" }
                    }
                    div {
                        div { class: "text-[var(--color-base-content)]/70 mb-1", "Time to Full" }
                        div { class: "font-semibold", "1h 23m" }
                    }
                    div {
                        div { class: "text-[var(--color-base-content)]/70 mb-1", "Power Draw" }
                        div { class: "font-semibold", "24.5 W" }
                    }
                }
            }

            // Stats Grid
            div { class: "grid grid-cols-2 gap-6 mb-6",
                // Capacity Info
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-4",
                        "CAPACITY"
                    }
                    div { class: "space-y-3",
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-[var(--color-base-content)]/70", "Design Capacity" }
                            span { class: "text-sm font-semibold", "54,000 mWh" }
                        }
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-[var(--color-base-content)]/70", "Full Charge Capacity" }
                            span { class: "text-sm font-semibold", "51,300 mWh" }
                        }
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-[var(--color-base-content)]/70", "Current Capacity" }
                            span { class: "text-sm font-semibold", "43,605 mWh" }
                        }
                        div { class: "pt-3 mt-3 border-t border-[var(--color-base-300)]",
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm text-[var(--color-base-content)]/70", "Battery Health" }
                                span { class: "text-sm font-semibold text-[var(--color-success)]", "95%" }
                            }
                        }
                    }
                }

                // Runtime Estimates
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-4",
                        "RUNTIME ESTIMATE"
                    }
                    div { class: "space-y-3",
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-[var(--color-base-content)]/70", "Current Load" }
                            span { class: "text-sm font-semibold", "3h 45m" }
                        }
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-[var(--color-base-content)]/70", "Light Usage" }
                            span { class: "text-sm font-semibold", "6h 20m" }
                        }
                        div { class: "flex justify-between items-center",
                            span { class: "text-sm text-[var(--color-base-content)]/70", "Heavy Usage" }
                            span { class: "text-sm font-semibold", "2h 10m" }
                        }
                        div { class: "pt-3 mt-3 border-t border-[var(--color-base-300)]",
                            div { class: "flex justify-between items-center",
                                span { class: "text-sm text-[var(--color-base-content)]/70", "Avg. Discharge Rate" }
                                span { class: "text-sm font-semibold", "11.6 W" }
                            }
                        }
                    }
                }
            }

            // Additional Battery Details
            div { class: "grid grid-cols-3 gap-6",
                // Voltage
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "VOLTAGE"
                    }
                    div { class: "text-3xl font-bold text-[var(--color-primary)] mb-2",
                        "12.6 V"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70",
                        "Nominal: 11.4 V"
                    }
                }

                // Cycle Count
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "CYCLE COUNT"
                    }
                    div { class: "text-3xl font-bold text-[var(--color-primary)] mb-2",
                        "127"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70",
                        "Target: < 500"
                    }
                }

                // Temperature
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-2",
                        "TEMPERATURE"
                    }
                    div { class: "text-3xl font-bold text-[var(--color-primary)] mb-2",
                        "32 °C"
                    }
                    div { class: "text-xs text-[var(--color-base-content)]/70",
                        "Status: Normal"
                    }
                }
            }
        }
    }
}
