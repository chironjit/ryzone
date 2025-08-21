use crate::model::{PowerUnit, TemperatureUnit};

/// Convert power value from milliwatts to the display unit
pub fn format_power(value_mw: u32, unit: PowerUnit) -> String {
    match unit {
        PowerUnit::MilliWatt => format!("{} mW", value_mw),
        PowerUnit::Watt => {
            let watts = value_mw as f64 / 1000.0;
            format!("{:.1} W", watts)
        }
    }
}

/// Parse power value from string in display unit to milliwatts for storage
pub fn parse_power(input: &str, unit: PowerUnit) -> Option<u32> {
    let value: f64 = input.trim().parse().ok()?;
    match unit {
        PowerUnit::MilliWatt => Some(value as u32),
        PowerUnit::Watt => Some((value * 1000.0) as u32),
    }
}

/// Convert temperature value from Celsius to the display unit
pub fn format_temperature(value_celsius: u32, unit: TemperatureUnit) -> String {
    match unit {
        TemperatureUnit::Celsius => format!("{}°C", value_celsius),
        TemperatureUnit::Fahrenheit => {
            let fahrenheit = (value_celsius as f64 * 9.0 / 5.0) + 32.0;
            format!("{:.0}°F", fahrenheit)
        }
    }
}

/// Parse temperature value from string in display unit to Celsius for storage
pub fn parse_temperature(input: &str, unit: TemperatureUnit) -> Option<u32> {
    let value: f64 = input.trim().parse().ok()?;
    match unit {
        TemperatureUnit::Celsius => Some(value as u32),
        TemperatureUnit::Fahrenheit => {
            let celsius = (value - 32.0) * 5.0 / 9.0;
            Some(celsius as u32)
        }
    }
}

/// Get power unit hint text for input fields
pub fn power_unit_hint(unit: PowerUnit) -> &'static str {
    match unit {
        PowerUnit::MilliWatt => "Enter value in mW",
        PowerUnit::Watt => "Enter value in W",
    }
}

/// Get temperature unit hint text for input fields  
pub fn temperature_unit_hint(unit: TemperatureUnit) -> &'static str {
    match unit {
        TemperatureUnit::Celsius => "Enter value in °C",
        TemperatureUnit::Fahrenheit => "Enter value in °F",
    }
}