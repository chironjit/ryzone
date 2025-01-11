use std::fs;
use std::io;
use std::path::Path;
use std::collections::VecDeque;
use std::time::SystemTime;
use iced::time::Duration;

use crate::model::HistoricalBattStat;

#[derive(Debug)]
struct BatteryInfo {
    pub power_now: u32,     // Power in microwatts
    pub energy_now: u32,    // Energy in microwatt-hours
    pub status: String,     // Charging, Discharging, Full, Not charging
    pub present: bool,      // Is battery present
}

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

fn read_battery_info(path: &str) -> io::Result<BatteryInfo> {
    println!("Reading battery info from: {}", path);
    
    let status = fs::read_to_string(format!("{}/status", path))?.trim().to_string();
    println!("Battery status: {}", status);
    
    let present = fs::read_to_string(format!("{}/present", path))?.trim() == "1";
    println!("Battery present: {}", present);
    
    let power_path = format!("{}/power_now", path);
    let current_path = format!("{}/current_now", path);
    let voltage_path = format!("{}/voltage_now", path);
    
    // Different systems might use different file names for power measurements
    let power_now = if Path::new(&power_path).exists() {
        let power = fs::read_to_string(&power_path)?
            .trim()
            .parse::<u32>()
            .unwrap_or(0);
        println!("Power from power_now: {} µW", power);
        power
    } else if Path::new(&current_path).exists() {
        // If power_now isn't available, calculate from current and voltage
        let current = fs::read_to_string(&current_path)?
            .trim()
            .parse::<u32>()
            .unwrap_or(0);
        let voltage = fs::read_to_string(&voltage_path)?
            .trim()
            .parse::<u32>()
            .unwrap_or(0);
        let power = (current as u64 * voltage as u64 / 1_000_000) as u32;  // Convert to microwatts
        println!("Power calculated from current ({} µA) * voltage ({} µV): {} µW", 
                current, voltage, power);
        power
    } else {
        println!("No power measurement available!");
        0
    };
    
    let energy_path = format!("{}/energy_now", path);
    let charge_path = format!("{}/charge_now", path);
    
    let energy_now = if Path::new(&energy_path).exists() {
        fs::read_to_string(&energy_path)?
            .trim()
            .parse::<u32>()
            .unwrap_or(0)
    } else if Path::new(&charge_path).exists() {
        let charge = fs::read_to_string(&charge_path)?
            .trim()
            .parse::<u32>()
            .unwrap_or(0);
        let voltage = fs::read_to_string(&voltage_path)?
            .trim()
            .parse::<u32>()
            .unwrap_or(0);
        (charge as u64 * voltage as u64 / 1_000_000) as u32  // Convert to microwatt-hours
    } else {
        0
    };

    Ok(BatteryInfo {
        power_now,
        energy_now,
        status,
        present,
    })
}

pub fn get_battery_metrics(history: &mut VecDeque<HistoricalBattStat>) -> (u32, u32) {
    
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    let mut current_power = 0;
    let mut time_remaining = 0;
    let mut is_charging = false;
    let mut total_energy = 0;

    // Get current battery readings
    for path in find_battery_paths() {
        if let Ok(info) = read_battery_info(&path) {
            if info.present {
                if info.status == "Charging" {
                    is_charging = true;
                    break;
                } else if info.status == "Discharging" {
                    // Convert to watts with one decimal point precision (multiply by 10 for rounding)
                    current_power = (info.power_now as f64 / 100_000.0).round() as u32;
                    total_energy = info.energy_now / 1_000_000; // Convert to watt-hours
                }
            }
        }
    }
    
    if !is_charging {
        // Add current reading to history
        history.push_back(HistoricalBattStat {
            timestamp: now,
            power_usage: current_power,
        });

        // Remove old entries
        while history.front().map_or(false, |h| h.timestamp < five_mins_ago) {
            history.pop_front();
        }
        
        // Calculate average power consumption over the last 5 minutes
        if !history.is_empty() {
            let avg_power: u32 = history.iter()
                .map(|h| h.power_usage)
                .sum::<u32>() / history.len() as u32;
            
            if avg_power > 0 {
                time_remaining = ((total_energy as f64 * 60.0) / avg_power as f64) as u32;
            }
        }
    }

    (current_power, time_remaining)
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