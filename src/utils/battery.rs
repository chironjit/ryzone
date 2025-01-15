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

fn get_batt_stat(path: PathBuf) -> HistoricalBattStat {
    let charge_now = fs::read_to_string(&path.join("charge_now"))
                                .ok()  
                                .and_then(|s| s.trim().parse().ok())  
                                .unwrap_or(0);
    let voltage_now = fs::read_to_string(&path.join("voltage_now"))
                                .ok()  
                                .and_then(|s| s.trim().parse().ok())  
                                .unwrap_or(0);
    let current_now = fs::read_to_string(&path.join("current_now"))
                            .ok()  
                            .and_then(|s| s.trim().parse().ok())  
                            .unwrap_or(0);
    let capacity = fs::read_to_string(&path.join("capacity"))
                                .ok()  
                                .and_then(|s| s.trim().parse().ok())  
                                .unwrap_or(0);
    let status = fs::read_to_string(&path.join("status"))
                                .map_or(String::from(""), |s| s.trim().to_string());

    let power_usage = (voltage_now/1_000_000) * (current_now/100_000);

    HistoricalBattStat{
        timestamp: SystemTime::now(),
        power_usage,
        charge_now,
        voltage_now,
        current_now,
        capacity,
        status
    }
}




pub fn get_battery_metrics(batt_stat: &mut VecDeque<HistoricalBattStat>) -> (u32, u32, String) {
    let battery_paths = find_battery_paths();
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    match battery_paths.len() {
        0 => (0, 0, "No batt detected".to_string()),
        1 => {
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


            // Return results 
            (latest_batt_stat.power_usage, batt_time, latest_batt_stat.status)

        },
        _ => (0, 0, "Multiple batteries detected".to_string()),
    }
}

// fn read_battery_info(path: &str) -> io::Result<BatteryInfo> {
//     let status = fs::read_to_string(format!("{}/status", path))?.trim().to_string();
//     let present = fs::read_to_string(format!("{}/present", path))?.trim() == "1";
    
//     let power_path = format!("{}/power_now", path);
//     let current_path = format!("{}/current_now", path);
//     let voltage_path = format!("{}/voltage_now", path);
    
//     // Different systems might use different file names for power measurements
//     let power_now = if Path::new(&power_path).exists() {
//         fs::read_to_string(&power_path)?
//             .trim()
//             .parse::<u32>()
//             .unwrap_or(0)
//     } else if Path::new(&current_path).exists() {
//         // If power_now isn't available, calculate from current and voltage
//         let current = fs::read_to_string(&current_path)?
//             .trim()
//             .parse::<u32>()
//             .unwrap_or(0);
//         let voltage = fs::read_to_string(&voltage_path)?
//             .trim()
//             .parse::<u32>()
//             .unwrap_or(0);
//         (current as u64 * voltage as u64 / 1_000_000) as u32  // Convert to microwatts
//     } else {
//         0
//     };
    
//     let energy_path = format!("{}/energy_now", path);
//     let charge_path = format!("{}/charge_now", path);
    
//     let energy_now = if Path::new(&energy_path).exists() {
//         fs::read_to_string(&energy_path)?
//             .trim()
//             .parse::<u32>()
//             .unwrap_or(0)
//     } else if Path::new(&charge_path).exists() {
//         let charge = fs::read_to_string(&charge_path)?
//             .trim()
//             .parse::<u32>()
//             .unwrap_or(0);
//         let voltage = fs::read_to_string(&voltage_path)?
//             .trim()
//             .parse::<u32>()
//             .unwrap_or(0);
//         (charge as u64 * voltage as u64 / 1_000_000) as u32  // Convert to microwatt-hours
//     } else {
//         0
//     };

//     Ok(BatteryInfo {
//         power_now,
//         energy_now,
//         status,
//         present,
//         timestamp: SystemTime::now(),
//     })
// }

// pub fn get_battery_metrics(power_history: &mut PowerHistory) -> (u32, u32) {
//     let now = SystemTime::now();
//     let five_mins_ago = now - Duration::from_secs(300);
//     let one_min_ago = now - Duration::from_secs(60);
//     let ten_secs_ago = now - Duration::from_secs(10);
    
//     let mut current_power = 0;
//     let mut time_remaining = 0;
//     let mut is_charging = false;
//     let mut total_energy = 0;

//     // Get current battery readings
//     for path in find_battery_paths() {
//         if let Ok(info) = read_battery_info(&path) {
//             if info.present {
//                 if info.status == "Charging" {
//                     is_charging = true;
//                     power_history.last_charging_time = Some(now);
//                     break;
//                 } else if info.status == "Discharging" {
//                     current_power = ((info.power_now as f64 / 100_000.0).round() as u32);
//                     total_energy = info.energy_now / 1_000_000;
                    
//                     // If we were charging within the last 10 seconds, clear history
//                     if let Some(last_charge) = power_history.last_charging_time {
//                         if now.duration_since(last_charge).unwrap() < Duration::from_secs(10) {
//                             power_history.history.clear();
//                             // Return early with just current power, no time estimate
//                             return (current_power, 0);
//                         }
//                     }
//                 }
//             }
//         }
//     }

//     if !is_charging {
//         // Add current reading to history
//         power_history.history.push_back(HistoricalBattStat {
//             timestamp: now,
//             power_usage: current_power,
//         });

//         // Calculate average based on available data
//         if !power_history.history.is_empty() {
//             // Get relevant history window based on data availability
//             let relevant_history: Vec<_> = if power_history.history.iter().any(|h| h.timestamp <= five_mins_ago) {
//                 power_history.history.iter()
//                     .filter(|h| h.timestamp > five_mins_ago)
//                     .collect()
//             } else if power_history.history.iter().any(|h| h.timestamp <= one_min_ago) {
//                 power_history.history.iter()
//                     .filter(|h| h.timestamp > one_min_ago)
//                     .collect()
//             } else {
//                 power_history.history.iter()
//                     .filter(|h| h.timestamp > ten_secs_ago)
//                     .collect()
//             };

//             if !relevant_history.is_empty() {
//                 let avg_power: u32 = relevant_history.iter()
//                     .map(|h| h.power_usage)
//                     .sum::<u32>() / relevant_history.len() as u32;

//                 if avg_power > 0 {
//                     time_remaining = ((total_energy as f64 * 60.0) / avg_power as f64) as u32;
//                 }
//             }
//         }

//         // Clean up old history entries
//         while power_history.history.front().map_or(false, |h| h.timestamp < five_mins_ago) {
//             power_history.history.pop_front();
//         }
//     }

//     (current_power, time_remaining)
// }

pub fn format_time_remaining(minutes: u32) -> String {
    if minutes == 0 {
        String::from("N/A")
    } else if minutes >= 60 {
        format!("{}h:{:02}m", minutes / 60, minutes % 60)
    } else {
        format!("{} min", minutes)
    }
}