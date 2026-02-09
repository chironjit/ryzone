// All app settings read and loaded into a struct
pub struct AppSettings {
    // App settings
    pub theme_mode: String,
    pub theme_light_palette: String,
    pub theme_dark_palette: String,
    pub power_unit: String,
    pub temp_unit: String,
    pub update_frequency: String,
    pub start_on_boot: bool,
    pub minimize_to_tray: bool,
    pub active_profile: String,
}

pub struct ProfileSettings {
    // System profile
    // Performance
    pub system_performance_fast_limit: i32,
    pub system_performance_slow_limit: i32,
    pub system_performance_stapm_limit: i32,
    pub system_performance_tctl_limit: i32,
    // Balanced
    pub system_balanced_fast_limit: i32,
    pub system_balanced_slow_limit: i32,
    pub system_balanced_stapm_limit: i32,
    pub system_balanced_tctl_limit: i32,
    // Power Saver
    pub system_power_saver_fast_limit: i32,
    pub system_power_saver_slow_limit: i32,
    pub system_power_saver_stapm_limit: i32,
    pub system_power_saver_tctl_limit: i32,

    // Custom profile
    // AC
    pub custom_ac_fast_limit: i32,
    pub custom_ac_slow_limit: i32,
    pub custom_ac_stapm_limit: i32,
    pub custom_ac_tctl_limit: i32,
    // Battery
    pub custom_battery_fast_limit: i32,
    pub custom_battery_slow_limit: i32,
    pub custom_battery_stapm_limit: i32,
    pub custom_battery_tctl_limit: i32,
    // Low Battery
    pub custom_low_battery_fast_limit: i32,
    pub custom_low_battery_slow_limit: i32,
    pub custom_low_battery_stapm_limit: i32,
    pub custom_low_battery_tctl_limit: i32,
    pub custom_low_battery_threshold: i32,

    // Turbo profile
    pub turbo_fast_limit: i32,
    pub turbo_slow_limit: i32,
    pub turbo_stapm_limit: i32,
    pub turbo_tctl_limit: i32,

    // Fixed profile
    pub fixed_fast_limit: i32,
    pub fixed_slow_limit: i32,
    pub fixed_stapm_limit: i32,
    pub fixed_tctl_limit: i32,
}

pub struct CurrentStatus {
    // CPU
    pub cpu_frequency: i32,
    pub cpu_temperature: i32,
    pub cpu_load: i32,
    // GPU
    pub gpu_frequency: i32,
    pub gpu_temperature: i32,
    pub gpu_load: i32,

    // Power
    pub power_draw: i32,

    // Battery
    pub batt_charge_status: String,
    pub batt_charge_percentage: i32,
    pub batt_design_capacity: i32,
    pub batt_full_charge_capacity: i32,
    pub batt_current_capacity: i32,
    pub batt_health: i32,
    pub batt_voltage: i32,
    pub batt_cycle_count: i32,
    pub batt_temperature: i32,

    // Runtime estimates
    pub current_load: i32,
    pub light_usage: i32,
    pub heavy_usage: i32,
    pub avg_discharge_rate: i32,
    
    // Power Limits
    // Curreng Limits
    pub curr_fast_limit: i32,
    pub curr_slow_limit: i32,
    pub curr_stapm_limit: i32,
    pub curr_tctl_limit: i32,
    // Current Values
    pub curr_fast_value: i32,
    pub curr_slow_value: i32,
    pub curr_stapm_value: i32,
    pub curr_tctl_value: i32,
    // Current percentages (of limits)
    pub curr_fast_percentage: i32,
    pub curr_slow_percentage: i32,
    pub curr_stapm_percentage: i32,
    pub curr_tctl_percentage: i32,
}

pub static APP_SETTINGS_TEMPLATE: &str = r#"
[units]
power = "watt"                      # watt | mwatt
temp = "celcius"                    # celcius | fahrenheit | kelvin | rankine | réaumur

[style]
theme_mode = "dark"                 # dark | light
theme_light_palette = "winter"      # winter | black | nord
theme_dark_palette =  "dim"         # dracula | night | dim 
tab = "settings"                    # dashboard | profiles | settings
profile = "system"                  # system | custom | turbo | fixed
"#;

pub static PROFILE_SETTINGS_TEMPLATE: &str = r#"
[system.performance]
fast = 0
slow = 0
stapm = 0
temp = 0

[system.balanced]
fast = 0
slow = 0
stapm = 0
temp = 0

[system.power_saver]
fast = 0
slow = 0
stapm = 0
temp = 0

[custom.ac]
fast = 0
slow = 0
stapm = 0
temp = 0

[custom.batt]
fast = 0
slow = 0
stapm = 0
temp = 0

[custom.low_batt]
fast = 0
slow = 0
stapm = 0
temp = 0
threshold = 0

[turbo]
fast = 0
slow = 0
stapm = 0
temp = 0

[fixed]
fast = 0
slow = 0
stapm = 0
temp = 0
"#;

pub fn read_app_settings() -> AppSettings {

}

pub fn read_profile_settings() -> ProfileSettings {

}

pub fn write_app_settings(settings: AppSettings) {

}

pub fn write_profile_settings(settings: ProfileSettings) {

}

pub fn create_settings_file() {
    // Create new settings file in the /home/<user>/.ryzone folder
    let home_dir = std::env::var("HOME").unwrap();
    let settings_file = format!("{}/{}", home_dir, ".ryzone/app_settings.toml");
    let mut file = File::create(settings_file).unwrap();
    file.write_all(APP_SETTINGS_TEMPLATE.as_bytes()).unwrap();
    file.write_all(PROFILE_SETTINGS_TEMPLATE.as_bytes()).unwrap();
}

