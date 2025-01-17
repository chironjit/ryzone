use std::fs;
use std::io;
use std::path::Path;
use std::time::SystemTime;
use iced::time::Duration;
use std::collections::VecDeque;
use std::path::PathBuf;

use crate::model::HistoricalBattStat;


// pub struct HistoricalBattStat {
//     pub timestamp: SystemTime,
//     pub power_usage: u32,  // Watts
//     pub charge_now: u32, // expected in Ah or µAh
//     pub voltage_now: u32, // volts
//     pub current_now: u32, // Amp
//     pub capacity: u32, // percentage number (0 - 100)
//     pub status: String, // Charging / Discharging (Other states stored for debugging and / or status update)
// }

fn find_battery_paths() -> Vec<String> {
    let mut battery_paths = Vec::new();
    
    // Check common battery paths
    for i in 0..3 {  // Check BAT0, BAT1, BAT2
        let path = format!("/sys/class/power_supply/BAT{}", i);
        if Path::new(&path).exists() {
            println!("Found battery path: {}", path);
            battery_paths.push(path);
        }
    }
    
    // Some systems might use CMB0, CMB1, etc.
    for i in 0..3 {
        let path = format!("/sys/class/power_supply/CMB{}", i);
        if Path::new(&path).exists() {
            println!("Found battery path: {}", path);
            battery_paths.push(path);
        }
    }
    
    if battery_paths.is_empty() {
        println!("No battery paths found!");
    }
    
    battery_paths
}

// Generic function to call and read called stat and return it as a string 
fn get_batt_stat(path: PathBuf, stat: &str) -> String {
    fs::read_to_string(path.join(stat))
        .unwrap_or_default()  // Return an empty string "" if error
        .trim()
        .to_string()
}


// fn get_batt_stat(path: PathBuf, history: HistoricalBattStat) -> HistoricalBattStat {
//     let charge_now = fs::read_to_string(&path.join("charge_now"))
//                                 .ok()  
//                                 .and_then(|s| s.trim().parse().ok())  
//                                 .unwrap_or(0);
//     let voltage_now = fs::read_to_string(&path.join("voltage_now"))
//                                 .ok()  
//                                 .and_then(|s| s.trim().parse().ok())  
//                                 .unwrap_or(0);
//     let current_now = fs::read_to_string(&path.join("current_now"))
//                             .ok()  
//                             .and_then(|s| s.trim().parse().ok())  
//                             .unwrap_or(0);
//     let capacity = fs::read_to_string(&path.join("capacity"))
//                                 .ok()  
//                                 .and_then(|s| s.trim().parse().ok())  
//                                 .unwrap_or(0);
//     let status = fs::read_to_string(&path.join("status"))
//                                 .map_or(String::from(""), |s| s.trim().to_string());

//     let power_usage = (voltage_now/1_000_000) * (current_now/100_000);

//     HistoricalBattStat{
//         timestamp: SystemTime::now(),
//         power_usage,
//         charge_now,
//         voltage_now,
//         current_now,
//         capacity,
//         discharge_rate,
//         discharge_metric,
//         status
//     }
// }

pub fn get_battery_metrics(batt_stat: &mut VecDeque<HistoricalBattStat>) -> (u32, u32, String) {
    let battery_paths = find_battery_paths();
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    match battery_paths.len() {
        0 => (0, 0, "No batt detected".to_string()),
        1 => {
            // Get history fom 60 readings ago


            // Get batt stat() and add history
            let latest_batt_stat = get_batt_stat(PathBuf::from(&battery_paths[0]));

            // Add to history
            batt_stat.push_back(latest_batt_stat.clone());

            // Remove old entries
            while batt_stat.front().map_or(false, |h| h.timestamp < five_mins_ago) {
                batt_stat.pop_front();
            }

            // Calculate est running time (based on status)
            let mut batt_time: u32 = 0;

            // Find last non-discharging state index
            let discharge_start_idx = batt_stat.iter()
                .rposition(|stat| stat.status != "Discharging")
                .unwrap_or(0);

            let recent_discharge_count = batt_stat.iter()
                .skip(discharge_start_idx)
                .count();

            if recent_discharge_count > 40 {
                let prev_stat = &batt_stat[batt_stat.len() - 30];

                let charge_diff = prev_stat.charge_now - latest_batt_stat.charge_now;
                let time_diff = latest_batt_stat.timestamp.duration_since(prev_stat.timestamp)
                    .unwrap_or_default()
                    .as_secs();

                // Return 0 if no discharge or no time passed
                if charge_diff > 0 && time_diff != 0 {
                    let discharge_rate = (charge_diff as f64) / (time_diff as f64);
                    batt_time = (latest_batt_stat.charge_now as f64 / discharge_rate / 60.) as u32 
                }
            }

            // Return results 
            (latest_batt_stat.power_usage, batt_time, latest_batt_stat.status)

        },
        _ => (0, 0, "Multiple batteries detected".to_string()),
    }
}

pub fn format_time_remaining(minutes: u32) -> String {
    if minutes == 0 {
        String::from("N/A")
    } else if minutes >= 60 {
        format!("{}h:{:02}m", minutes / 60, minutes % 60)
    } else {
        format!("{} min", minutes)
    }
}