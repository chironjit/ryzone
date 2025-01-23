use std::collections::VecDeque;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct HistoricalFreq {
    pub timestamp: SystemTime,
    pub freq: u32,  // MHz
}

impl Default for HistoricalFreq {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            freq: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoricalGpuFreq {
    pub timestamp: SystemTime,
    pub freq: u32,  // MHz
}

impl Default for HistoricalGpuFreq {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            freq: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoricalBattStat {
    pub timestamp: SystemTime,
    pub charge: u32, // charge
    pub capacity: u32, // capacity (0 - 100)
    pub discharge_rate: f64, // Rate of discharge per minute
    pub status: String, // Charging / Discharging (Other states stored for debugging and / or status update)
    pub status_count: u32,
}

impl Default for HistoricalBattStat {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            charge: 0,
            capacity: 0,
            discharge_rate: 0.,
            status: "".to_string(),
            status_count: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
   Profiles,
   Overrides,
   Settings
}

// Constants to limit input values
pub const FAST_LIMIT_MIN: u32 = 4000;
pub const FAST_LIMIT_MAX: u32 = 65000;
pub const SLOW_LIMIT_MIN: u32 = 4000;
pub const SLOW_LIMIT_MAX: u32 = 65000;
pub const STAPM_LIMIT_MIN: u32 = 4000;
pub const STAPM_LIMIT_MAX: u32 = 65000;
pub const TCTL_LIMIT_MIN: u32 = 40;
pub const TCTL_LIMIT_MAX: u32 = 100;



// All the states managed by the app
#[derive(Default, Debug, Clone)]
pub struct State {
    // App states
    active_tab: Tab,

    // Current APU power status via libryzenadj
    pub curr_fast_value: u32,
    pub curr_slow_value: u32,
    pub curr_stapm_value: u32,
    pub curr_tctl_value: u32,
    pub curr_fast_limit: u32,
    pub curr_slow_limit: u32,
    pub curr_stapm_limit: u32,
    pub curr_tctl_limit: u32,

    // CPU frequency status (in MHz)
    pub current_max_freq: u32,
    pub min_freq_5min: u32,
    pub max_freq_5min: u32,
    pub freq_history: VecDeque<HistoricalFreq>,

    // GPU frequency status (in MHz)
    pub current_gpu_freq: u32,
    pub min_gpu_freq_5min: u32,
    pub max_gpu_freq_5min: u32,
    pub gpu_history: VecDeque<HistoricalGpuFreq>,

    // System power status
    pub batt_power: u32,
    pub batt_time: u32,
    pub batt_status: String,
    pub batt_history: VecDeque<HistoricalBattStat>,

    // Custom values input tracking
    pub fast_input: String,
    pub slow_input: String,
    pub stapm_input: String,
    pub tctl_input: String,

    // Custom override values store
    pub manual_fast_limit: u32,
    pub manual_slow_limit: u32,
    pub manual_stapm_limit: u32,
    pub manual_tctl_limit: u32,
}