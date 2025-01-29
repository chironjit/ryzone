use std::collections::VecDeque;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::model::state::HistoricalBattStat;

fn find_battery_paths() -> Vec<String> {
    let mut battery_paths = Vec::new();

    // Check common battery paths
    for i in 0..3 {
        // Check BAT0, BAT1, BAT2
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
fn get_batt_stat(path: &str, stat: &str) -> String {
    let path_buf: PathBuf = PathBuf::from(path);
    fs::read_to_string(path_buf.join(stat))
        .unwrap_or_default() // Return an empty string "" if error
        .trim()
        .to_string()
}

fn determine_charge_and_capacity(battery_path: &String) -> (u32, u32) {
    let charge_now = get_batt_stat(&battery_path, "charge_now")
        .parse::<u32>()
        .unwrap_or_default();
    let capacity = get_batt_stat(&battery_path, "capacity")
        .parse::<u32>()
        .unwrap_or_default();

    return (charge_now, capacity);
}

fn calc_discharge_rate_per_min(
    curr_charge: u32,
    prev_charge: u32,
    curr_time: SystemTime,
    prev_time: SystemTime,
) -> f64 {
    let charge_diff = prev_charge.checked_sub(curr_charge).unwrap_or_default();
    let time_diff = curr_time
        .duration_since(prev_time)
        .unwrap_or_default()
        .as_secs();
    let discharge_rate_per_sec = (charge_diff as f64) / (time_diff as f64);
    discharge_rate_per_sec * 60.
}

pub fn get_battery_metrics(batt_stat: &mut VecDeque<HistoricalBattStat>) -> (u32, u32, String, u32) {
    let battery_paths = find_battery_paths();

    match battery_paths.len() {
        0 => (0, 0, "No batt detected".to_string(), 0),
        1 => {
            // Get status
            let status = get_batt_stat(&battery_paths[0], "status");

            // Get voltage and amperage,
            // ( and if available ) calculate power
            // & store data
            let mut power_now: u32 = 0;
            let voltage_now = (get_batt_stat(&battery_paths[0], "voltage_now"))
                .parse::<u32>()
                .unwrap_or(0);
            let current_now = (get_batt_stat(&battery_paths[0], "current_now"))
                .parse::<u32>()
                .unwrap_or(0);

            if voltage_now != 0 && current_now != 0 {
                power_now = ((voltage_now as u64 * current_now as u64) / 1_000_000_000_00) as u32;
            }

            let batt_stat_list_length = batt_stat.len();

            let timestamp = SystemTime::now();
            let (charge, capacity) = determine_charge_and_capacity(&battery_paths[0]);

            let mut discharge_rate: f64 = 0.;
            let mut batt_time: u32 = 0;
            let mut status_count: u32 = 0;

            if status == "Discharging" {
                if batt_stat_list_length != 0 {
                    let last_reading = &batt_stat[batt_stat_list_length - 1];

                    if last_reading.status != "Discharging" {
                        batt_stat.clear();

                        status_count = 1;
                    } else {
                        let earliest_available_reading = &batt_stat[0];

                        if charge != 0 {
                            discharge_rate = calc_discharge_rate_per_min(
                                charge,
                                earliest_available_reading.charge,
                                timestamp,
                                earliest_available_reading.timestamp,
                            );
                            batt_time = (charge as f64 / discharge_rate) as u32;
                        } else {
                            discharge_rate = calc_discharge_rate_per_min(
                                capacity,
                                earliest_available_reading.capacity,
                                timestamp,
                                earliest_available_reading.timestamp,
                            );
                            batt_time = (capacity as f64 / discharge_rate) as u32;
                        }

                        status_count = last_reading.status_count + 1;
                    }
                } else {
                }
            }

            println!("Batt time:{}, Status: {}", batt_time, status);

            // Add new readings to batt_stat
            let new_batt_stat = HistoricalBattStat {
                timestamp,
                charge,
                capacity,
                discharge_rate,
                status: status.clone(),
                status_count,
            };

            batt_stat.push_back(new_batt_stat);

            // Clear old readings oldes than 300
            if batt_stat_list_length > 300 {
                batt_stat.pop_front();
            }

            // // Return results
            (power_now, batt_time, status, capacity)
        }
        _ => (0, 0, "Multiple batteries detected".to_string(), 0),
    }
}
