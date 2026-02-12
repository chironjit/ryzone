use serde::{Deserialize, Serialize};

// App Settings
#[derive(Deserialize, Serialize, Clone)]
pub struct AppSettings {
    pub units: Units,
    pub style: Style,
    pub app: App,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Units {
    pub power: String,
    pub temp: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Style {
    pub theme_mode: String,
    pub theme_light_palette: String,
    pub theme_dark_palette: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct App {
    pub start_on_login: bool,
    pub minimize_to_tray: bool,
    pub enable_logging: bool,
    pub update_frequency_ms: i32, // milliseconds
    pub logging_frequency_ms: i32, // milliseconds
}

// Profile Settings
#[derive(Deserialize, Serialize, Clone)]
pub struct ProfileSettings {
    pub active_profile: String,
    pub low_batt_threshold_percent: i32,
    pub system: SystemProfiles,
    pub custom: CustomProfiles,
    pub turbo: TurboProfile,
    pub fixed: FixedProfile,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct SystemProfiles {
    pub performance: PowerLimits,
    pub balanced: PowerLimits,
    pub power_saver: PowerLimits,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct CustomProfiles {
    pub ac: PowerLimits,
    pub batt: PowerLimits,
    pub low_batt: PowerLimits,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TurboProfile {
    pub turbo: PowerLimits,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct FixedProfile {
    pub fixed: PowerLimits,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PowerLimits {
    pub fast_mw: i32, // milliwatts
    pub slow_mw: i32, // milliwatts
    pub stapm_mw: i32, // milliwatts
    pub temp_c: i32, // celsius
}

// Current stats type
#[derive(Default)]
pub struct CurrentStats {
    // CPU
    pub cpu_frequency_mhz: i32, // megahertz
    pub cpu_temperature_c: i32, // celsius
    pub cpu_load_percent: i32, // percentage
    
    // GPU
    pub gpu_frequency_mhz: i32, // megahertz
    pub gpu_temperature_c: i32, // celsius
    pub gpu_load_percent: i32, // percentage

    // Power
    pub power_draw_mw: i32, // milliwatts
    pub profile: String, // profile name
    pub sub_profile: String, // sub-profile name

    // Battery
    pub batt_charge_status: String, // charging | discharging | full | empty | na
    pub batt_charge_percent: i32, // percentage
    pub batt_design_capacity_mwh: i32, // milliwatt-hours
    pub batt_full_charge_capacity_mwh: i32, // milliwatt-hours
    pub batt_current_capacity_mwh: i32, // milliwatt-hours
    pub batt_health_percent: i32, // percentage
    pub batt_voltage_volt: i32, // volts
    pub batt_cycle_count_cycles: i32, // cycles
    pub batt_temperature_c: i32, // celsius

    // Runtime estimates
    pub current_load_min: i32, // minutes
    pub light_usage_min: i32, // minutes
    pub heavy_usage_min: i32, // minutes
    pub avg_discharge_rate_mw: i32, // milliwatts
    
    // Power Limits
    // Current Limits
    pub curr_fast_limit_mw: i32, // milliwatts
    pub curr_slow_limit_mw: i32, // milliwatts
    pub curr_stapm_limit_mw: i32, // milliwatts
    pub curr_tctl_limit_c: i32, // celsius
    // Current Values
    pub curr_fast_value_mw: i32, // milliwatts
    pub curr_slow_value_mw: i32, // milliwatts
    pub curr_stapm_value_mw: i32, // milliwatts
    pub curr_tctl_value_c: i32, // celsius
    // Current percentages (of limits)
    pub curr_fast_percent: i32, // percentage
    pub curr_slow_percent: i32, // percentage
    pub curr_stapm_percent: i32, // percentage
    pub curr_tctl_percent: i32, // percentage
}