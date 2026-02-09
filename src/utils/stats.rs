#[derive(Default)]
pub struct CurrentStats {
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
    pub profile: String,
    pub sub_profile: String,

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
    // Current Limits
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



