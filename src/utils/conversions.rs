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