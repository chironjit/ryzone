use dioxus::prelude::*;

#[component]
pub fn Battery() -> Element {
    // Sample data for the chart (in a real app, this would come from your daemon)
    let battery_history = vec![
        (0, 92),
        (1, 88),
        (2, 85),
        (3, 82),
        (4, 78),
        (5, 75),
        (6, 72),
        (7, 68),
        (8, 65),
        (9, 62),
        (10, 58),
        (11, 55),
        (12, 52),
    ];

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

            // Battery Charts Section
            div { class: "mb-6",
                div { class: "bg-[var(--color-base-200)] rounded-xl p-6 border border-[var(--color-base-300)]",
                    div { class: "text-sm font-semibold text-[var(--color-secondary)] mb-4",
                        "BATTERY HISTORY"
                    }

                    // Chart tabs
                    div { class: "flex gap-2 mb-4",
                        button { class: "px-3 py-1 rounded bg-[var(--color-primary)] text-white text-sm",
                            "Percentage"
                        }
                        button { class: "px-3 py-1 rounded bg-[var(--color-base-300)] text-[var(--color-base-content)] text-sm hover:bg-[var(--color-base-300)]/80",
                            "Runtime"
                        }
                        button { class: "px-3 py-1 rounded bg-[var(--color-base-300)] text-[var(--color-base-content)] text-sm hover:bg-[var(--color-base-300)]/80",
                            "Power Draw"
                        }
                    }

                    // Chart
                    BatteryChart { data: battery_history.clone() }
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

#[derive(Clone, PartialEq, Props)]
struct BatteryChartProps {
    data: Vec<(i32, i32)>,
    #[props(default = "Battery %".to_string())]
    y_label: String,
    #[props(default = "Time (hours ago)".to_string())]
    x_label: String,
    #[props(default = "%".to_string())]
    y_unit: String,
    #[props(default = "h".to_string())]
    x_unit: String,
}

#[component]
fn BatteryChart(props: BatteryChartProps) -> Element {
    let BatteryChartProps { data, y_label, x_label, y_unit, x_unit } = props;

    // Chart dimensions
    let width = 800.0;
    let height = 300.0;
    let padding = 40.0;

    let chart_width = width - 2.0 * padding;
    let chart_height = height - 2.0 * padding;

    // Calculate min/max values
    let min_val = data.iter().map(|(_, v)| v).min().unwrap_or(&0);
    let max_val = data.iter().map(|(_, v)| v).max().unwrap_or(&100);
    let min_time = data.first().map(|(t, _)| t).unwrap_or(&0);
    let max_time = data.last().map(|(t, _)| t).unwrap_or(&12);

    // Generate path for the line chart
    let points: Vec<String> = data.iter().map(|(time, value)| {
        let x = padding + ((*time - min_time) as f64 / (*max_time - min_time) as f64) * chart_width;
        let y = height - padding - ((value - min_val) as f64 / (max_val - min_val) as f64) * chart_height;
        format!("{},{}", x, y)
    }).collect();

    let path_data = format!("M {}", points.join(" L "));

    // Generate gradient area path
    let first_point = points.first().unwrap().split(',').collect::<Vec<_>>();
    let last_point = points.last().unwrap().split(',').collect::<Vec<_>>();
    let area_path = format!(
        "M {} L {},{} L {},{} Z",
        points.join(" L "),
        last_point[0], height - padding,
        first_point[0], height - padding
    );

    rsx! {
        div { class: "w-full overflow-x-auto",
            svg {
                width: "{width}",
                height: "{height}",
                view_box: "0 0 {width} {height}",
                class: "w-full h-auto",

                // Gradient definition
                defs {
                    linearGradient {
                        id: "batteryGradient",
                        x1: "0%",
                        y1: "0%",
                        x2: "0%",
                        y2: "100%",
                        stop { offset: "0%", stop_color: "rgba(59, 130, 246, 0.3)" }
                        stop { offset: "100%", stop_color: "rgba(59, 130, 246, 0.05)" }
                    }
                }

                // Grid lines (horizontal)
                for i in 0..5 {
                    {
                        let y = padding + (i as f64 * chart_height / 4.0);
                        let val = max_val - ((i as f64 * (*max_val - min_val) as f64 / 4.0) as i32);
                        rsx! {
                            line {
                                x1: "{padding}",
                                y1: "{y}",
                                x2: "{width - padding}",
                                y2: "{y}",
                                stroke: "rgba(128, 128, 128, 0.2)",
                                stroke_width: "1"
                            }
                            text {
                                x: "{padding - 5.0}",
                                y: "{y + 4.0}",
                                text_anchor: "end",
                                fill: "var(--color-base-content)",
                                opacity: "0.7",
                                font_size: "12",
                                "{val}{y_unit}"
                            }
                        }
                    }
                }

                // Grid lines (vertical)
                for i in 0..(max_time - min_time + 1) {
                    {
                        let x = padding + (i as f64 * chart_width / (*max_time - min_time) as f64);
                        rsx! {
                            line {
                                x1: "{x}",
                                y1: "{padding}",
                                x2: "{x}",
                                y2: "{height - padding}",
                                stroke: "rgba(128, 128, 128, 0.2)",
                                stroke_width: "1"
                            }
                            text {
                                x: "{x}",
                                y: "{height - padding + 20.0}",
                                text_anchor: "middle",
                                fill: "var(--color-base-content)",
                                opacity: "0.7",
                                font_size: "12",
                                "{i}{x_unit}"
                            }
                        }
                    }
                }

                // Area under the line
                path {
                    d: "{area_path}",
                    fill: "url(#batteryGradient)"
                }

                // Line
                path {
                    d: "{path_data}",
                    stroke: "rgb(59, 130, 246)",
                    stroke_width: "2",
                    fill: "none",
                    stroke_linejoin: "round",
                    stroke_linecap: "round"
                }

                // Data points
                for (time, value) in &data {
                    {
                        let x = padding + ((time - min_time) as f64 / (*max_time - min_time) as f64) * chart_width;
                        let y = height - padding - ((value - min_val) as f64 / (max_val - min_val) as f64) * chart_height;
                        rsx! {
                            circle {
                                cx: "{x}",
                                cy: "{y}",
                                r: "4",
                                fill: "rgb(59, 130, 246)",
                                stroke: "white",
                                stroke_width: "2"
                            }
                        }
                    }
                }

                // Axes labels
                text {
                    x: "{width / 2.0}",
                    y: "{height - 2.0}",
                    text_anchor: "middle",
                    fill: "var(--color-base-content)",
                    opacity: "0.6",
                    font_size: "11",
                    "{x_label}"
                }

                text {
                    x: "{8.0}",
                    y: "{height / 2.0}",
                    text_anchor: "middle",
                    fill: "var(--color-base-content)",
                    opacity: "0.6",
                    font_size: "11",
                    transform: "rotate(-90, 8, {height / 2.0})",
                    "{y_label}"
                }
            }
        }
    }
}
