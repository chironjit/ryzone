use glob::glob;
use std::fs;
use std::io;
use std::collections::VecDeque;
use std::time::SystemTime;
use iced::time::Duration;

use crate::utils;
use crate::model::model::{
    Tab,
    State,
    HistoricalFreq,
    HistoricalGpuFreq,
    FAST_LIMIT_MIN,
    FAST_LIMIT_MAX,
    SLOW_LIMIT_MIN,
    SLOW_LIMIT_MAX,
    STAPM_LIMIT_MIN,
    STAPM_LIMIT_MAX,
    TCTL_LIMIT_MIN,
    TCTL_LIMIT_MAX,
    THRESHOLD_MIN,
    THRESHOLD_MAX
};

use libryzenadj::RyzenAdj;

#[derive(Debug, Clone)]
pub enum Message {
    SetFastLimit(u32),
    SetSlowLimit(u32),
    SetStapmLimit(u32),
    SetTctlLimit(u32),
    FastLimitInputChanged(String),
    SlowLimitInputChanged(String),
    StapmLimitInputChanged(String),
    TctlLimitInputChanged(String),
    UpdateStateValues,
    TabSelected(Tab),
    SetBattProfile(u32, u32, u32, u32),
    SetSaverProfile(u32, u32, u32, u32, u32),
    ToggleSaverProfile,
    ToggleTurboOption,
    EnableTurbo,
    DisableTurbo,
    BattFastInputChanged(String),
    BattSlowInputChanged(String),
    BattStapmInputChanged(String),
    BattTctlInputChanged(String),
    SaverThresholdInputChanged(String),
    SaverFastInputChanged(String),
    SaverSlowInputChanged(String),
    SaverStapmInputChanged(String),
    SaverTctlInputChanged(String),
    PowerFastInputChanged(String),
    PowerSlowInputChanged(String),
    PowerStapmInputChanged(String),
    PowerTctlInputChanged(String),
}

// Standalone update function for the state
pub fn update(
    state: &mut State,
    message: Message
) {
    match message {
        Message::SetFastLimit(value) => {
            if value >= FAST_LIMIT_MIN && value <= FAST_LIMIT_MAX {
                state.manual_fast_limit = value.into();
            }
        }
        Message::SetSlowLimit(value) => {
            if value >= SLOW_LIMIT_MIN && value <= SLOW_LIMIT_MAX {
                state.manual_slow_limit = value.into();
            }
        }
        Message::SetStapmLimit(value) => {
            if value >= STAPM_LIMIT_MIN && value <= STAPM_LIMIT_MAX {
                state.manual_stapm_limit = value.into();
            }
        }
        Message::SetTctlLimit(value) => {
            if value >= TCTL_LIMIT_MIN && value <= TCTL_LIMIT_MAX {
                state.manual_tctl_limit = value.into();
            }
        }
        Message::FastLimitInputChanged(value) => {
            if value.is_empty() {
                state.fast_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= FAST_LIMIT_MAX {
                    state.fast_input = value;
                }
            }
        }
        Message::SlowLimitInputChanged(value) => {
            if value.is_empty() {
                state.slow_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= SLOW_LIMIT_MAX {
                    state.slow_input = value;
                }
            }
        }
        Message::StapmLimitInputChanged(value) => {
            if value.is_empty() {
                state.stapm_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= STAPM_LIMIT_MAX {
                    state.stapm_input = value;
                }
            }
        }
        Message::TctlLimitInputChanged(value) => {
            if value.is_empty() {
                state.tctl_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= TCTL_LIMIT_MAX {
                    state.tctl_input = value;
                }
            }
        }
        Message::UpdateStateValues => {
            // Every second, check what the actual values are
            // If values don't match the override
            // (and the manual values are not 0), 
            // try to apply the override value
            // Then check the latest figures and then update the state

            let ryzen = RyzenAdj::new().unwrap();


            state.curr_fast_limit = (ryzen.get_fast_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_fast_value = (ryzen.get_fast_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_slow_limit = (ryzen.get_slow_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_slow_value = (ryzen.get_slow_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_stapm_limit = (ryzen.get_stapm_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_stapm_value = (ryzen.get_stapm_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_tctl_limit = ryzen.get_tctl_temp().unwrap_or_default().round() as u32;
            state.curr_tctl_value = ryzen.get_tctl_temp_value().unwrap_or_default().round() as u32;

            // CPU frequency updates
            let (current, min_5min, max_5min) = get_cpu_metrics(&mut state.freq_history);
            state.current_max_freq = current;
            state.min_freq_5min = min_5min;
            state.max_freq_5min = max_5min;

            // GPU frequency updates
            let (current, min_5min, max_5min) = get_gpu_metrics(&mut state.gpu_history);
            state.current_gpu_freq = current;
            state.min_gpu_freq_5min = min_5min;
            state.max_gpu_freq_5min = max_5min;


            // Power stats updates
            let (power, time, status) = utils::battery::get_battery_metrics(&mut state.batt_history);

            state.batt_power = power;
            state.batt_time = time;
            state.batt_status = status;


            if state.manual_fast_limit != 0 && state.curr_fast_limit != state.manual_fast_limit {
                let _ = ryzen.set_fast_limit(state.manual_fast_limit);
            }

            if state.manual_slow_limit != 0 && state.curr_slow_limit != state.manual_slow_limit {
                let _ = ryzen.set_slow_limit(state.manual_slow_limit);
            }

            if state.manual_stapm_limit != 0 && state.curr_stapm_limit != state.manual_stapm_limit {
                let _ = ryzen.set_stapm_limit(state.manual_stapm_limit);
            }

            if state.manual_tctl_limit != 0 && state.curr_tctl_limit != state.manual_tctl_limit {
                let _ = ryzen.set_tctl_temp(state.manual_tctl_limit);
            }
        }

        Message::TabSelected(tab) => {
            state.active_tab = tab;
        }

        Message::ToggleSaverProfile => {
            state.enable_saver_profile = !state.enable_saver_profile;
        }

        Message::ToggleTurboOption => {
            state.enable_turbo = !state.enable_turbo;
        }

        Message::SetBattProfile(fast, slow, stapm, tctl) => {
            state.batt_fast_limit = fast;
            state.batt_slow_limit = slow;
            state.batt_stapm_limit = stapm;
            state.batt_tctl_limit = tctl;
        }

        Message::SetSaverProfile(fast, slow, stapm, tctl, threshold) => {
            state.saver_fast_limit = fast;
            state.saver_slow_limit = slow;
            state.saver_stapm_limit = stapm;
            state.saver_tctl_limit = tctl;
            state.saver_threshold = threshold;
        }

        Message::EnableTurbo => {
            state.enable_turbo = true;
        }

        Message::DisableTurbo => {
            state.enable_turbo = false;
        }

        Message::SaverThresholdInputChanged(input) => {
            if input.is_empty() {
                state.saver_threshold_input = input;
            } else if let Ok(value) = input.parse::<u32>() {
                if value <= THRESHOLD_MAX {
                    state.saver_threshold_input = input;
                }
            }
        }

        Message::BattFastInputChanged(input) => {
            if input.is_empty() {
                state.batt_fast_input = input;
            } else if let Ok(value) = input.parse::<u32>() {
                if value <= FAST_LIMIT_MAX {
                    state.batt_fast_input = input;
                }
            }
        }

        Message::BattSlowInputChanged(input) => {
            if input.is_empty() {
                state.batt_slow_input = input;
            } else if let Ok(value) = input.parse::<u32>() {
                if value <= SLOW_LIMIT_MAX {
                    state.batt_slow_input = input;
                }
            }
        }

        Message::BattStapmInputChanged(input) => {
            if input.is_empty() {
                state.batt_stapm_input = input;
            } else if let Ok(value) = input.parse::<u32>() {
                if value <= STAPM_LIMIT_MAX {
                    state.batt_stapm_input = input;
                }
            }
        }

        Message::BattTctlInputChanged(input) => {
            if input.is_empty() {
                state.batt_tctl_input = input;
            } else if let Ok(value) = input.parse::<u32>() {
                if value <= TCTL_LIMIT_MAX {
                    state.batt_tctl_input = input;
                }
            }
        }

        Message::SaverFastInputChanged(value) => {
            if value.is_empty() {
                state.saver_fast_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= FAST_LIMIT_MAX {
                    state.saver_fast_input = value;
                }
            }
        }

        Message::SaverSlowInputChanged(value) => {
            if value.is_empty() {
                state.saver_slow_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= SLOW_LIMIT_MAX {
                    state.saver_slow_input = value;
                }
            }
        }

        Message::SaverStapmInputChanged(value) => {
            if value.is_empty() {
                state.saver_stapm_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= STAPM_LIMIT_MAX {
                    state.saver_stapm_input = value;
                }
            }
        }

        Message::SaverTctlInputChanged(value) => {
            if value.is_empty() {
                state.saver_tctl_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= TCTL_LIMIT_MAX {
                    state.saver_tctl_input = value;
                }
            }
        }

        Message::PowerFastInputChanged(value) => {
            if value.is_empty() {
                state.power_fast_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= FAST_LIMIT_MAX {
                    state.power_fast_input = value;
                }
            }
        }

        Message::PowerSlowInputChanged(value) => {
            if value.is_empty() {
                state.power_slow_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= SLOW_LIMIT_MAX {
                    state.power_slow_input = value;
                }
            }
        }

        Message::PowerStapmInputChanged(value) => {
            if value.is_empty() {
                state.power_stapm_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= STAPM_LIMIT_MAX {
                    state.power_stapm_input = value;
                }
            }
        }

        Message::PowerTctlInputChanged(value) => {
            if value.is_empty() {
                state.power_tctl_input = value;
            } else if let Ok(num) = value.parse::<u32>() {
                if num <= TCTL_LIMIT_MAX {
                    state.power_tctl_input = value;
                }
            }
        }
        
    }
}

fn get_cpu_frequency() -> Result<Vec<u32>, io::Error> {
    let mut frequencies = Vec::new();
    let cpu_count = fs::read_dir("/sys/devices/system/cpu")
        .unwrap()
        .filter(|entry| {
            entry.as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("cpu")
        })
        .count();

    for cpu in 0..cpu_count {
        let freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", cpu);
        if let Ok(freq_str) = fs::read_to_string(freq_path) {
            let freq_khz: u32 = freq_str.trim().parse().unwrap_or(0);
            frequencies.push(freq_khz); // Keep as kHz
        }
    }

    Ok(frequencies)
}

fn get_cpu_metrics(history: &mut VecDeque<HistoricalFreq>) -> (u32, u32, u32) {
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    // Read current frequencies (converting kHz to MHz)
    let mut current_max = 0;
    if let Ok(freqs) = get_cpu_frequency() {
        if !freqs.is_empty() {
            current_max = freqs.iter().max().unwrap_or(&0) / 1000; // Convert kHz to MHz
        }
    }
    
    // Add to history
    history.push_back(HistoricalFreq {
        timestamp: now,
        freq: current_max,
    });
    
    // Remove old entries
    while history.front().map_or(false, |h| h.timestamp < five_mins_ago) {
        history.pop_front();
    }
    
    // Calculate min and max from history
    let mut min_5min = current_max;
    let mut max_5min = current_max;
    
    for hist in history.iter() {
        min_5min = min_5min.min(hist.freq);
        max_5min = max_5min.max(hist.freq);
    }
    
    (current_max, min_5min, max_5min)
}

fn get_gpu_frequency() -> Result<Vec<u32>, io::Error> {
    let mut frequencies = Vec::new();
    
    // Look for AMD GPU sclk files
    for entry in glob("/sys/class/drm/card*/device/pp_dpm_sclk").unwrap() {
        if let Ok(path) = entry {
            if let Ok(content) = fs::read_to_string(path) {
                // Each line looks like: "0: 200Mhz *"
                // The * indicates which state is active
                for line in content.lines() {
                    if line.contains('*') {  // This is the active frequency
                        if let Some(freq_str) = line.split_whitespace().nth(1) {
                            if let Some(freq_num) = freq_str.trim_end_matches("Mhz").parse::<u32>().ok() {
                                frequencies.push(freq_num);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(frequencies)
}

fn get_gpu_metrics(history: &mut VecDeque<HistoricalGpuFreq>) -> (u32, u32, u32) {
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    // Read current frequencies
    let mut current = 0;
    if let Ok(freqs) = get_gpu_frequency() {
        if !freqs.is_empty() {
            current = freqs[0];  // Usually there's only one GPU
        }
    }
    
    // Add to history
    history.push_back(HistoricalGpuFreq {
        timestamp: now,
        freq: current,
    });
    
    // Remove old entries
    while history.front().map_or(false, |h| h.timestamp < five_mins_ago) {
        history.pop_front();
    }
    
    // Calculate min and max from history
    let mut min_5min = current;
    let mut max_5min = current;
    
    for hist in history.iter() {
        min_5min = min_5min.min(hist.freq);
        max_5min = max_5min.max(hist.freq);
    }
    
    (current, min_5min, max_5min)
}