// Use these conversion functions to always display the true values for the units
// These conversions also have the built in limits for the units
// Limits are:
// - Watt: 0 - 100
// - Milliwatt: 0 - 100,000
// - Celsius: 0 - 100
// - Fahrenheit: 0 - 212

pub fn power_conversion(value: i32, from: &str, to: &str) -> i32 {
    match from {
        "watt" => match to {
            "milliwatt" => value * 1000,
            "watt" => value,
            _ => value,
        },
        "milliwatt" => match to {
            "watt" => value / 1000,
            "milliwatt" => value,
            _ => value,
        },
        _ => value,
    }
}

pub fn temp_conversion(value: f32, from: &str, to: &str) -> f32 {
    let result = match from {
        "celsius" => match to {
            "fahrenheit" => (value * 1.8) + 32.0,
            _ => value,
        },
        "fahrenheit" => match to {
            "celsius" => (value - 32.0) / 1.8,
            _ => value,
        },
        _ => value,
    };
    (result * 10.0).round() / 10.0
}

pub fn power_unit_label(unit: &str) -> &'static str {
    match unit {
        "milliwatt" => "mW",
        _ => "W",
    }
}

pub fn temp_unit_label(unit: &str) -> &'static str {
    match unit {
        "fahrenheit" => "°F",
        _ => "°C",
    }
}

pub fn minutes_to_hm_text(minutes: i32) -> String {
    if minutes <= 0 {
        return "N/A".to_string();
    }
    let hours = minutes / 60;
    let mins = minutes % 60;
    format!("{hours}h {mins}m")
}

pub fn format_mwh(value: i32) -> String {
    format!("{value} mWh")
}

pub fn format_power_mw(value_mw: i32) -> String {
    if value_mw <= 0 {
        return "0 W".to_string();
    }
    format!("{:.1} W", value_mw as f32 / 1000.0)
}

pub fn battery_health_label(health_percent: i32) -> &'static str {
    match health_percent {
        90..=100 => "Excellent",
        80..=89 => "Good",
        70..=79 => "Fair",
        50..=69 => "Poor",
        1..=49 => "Degraded",
        _ => "N/A",
    }
}

pub fn battery_status_text(status: &str) -> &'static str {
    match status {
        "charging" => "Charging",
        "discharging" => "Discharging",
        "full" => "Full",
        "empty" => "Empty",
        _ => "N/A",
    }
}