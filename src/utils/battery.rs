use std::fs;
use std::io;
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;
use iced::time::Duration;
use std::collections::VecDeque;
use std::path::PathBuf;

use crate::model::HistoricalBattStat;

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
fn get_batt_stat(path: &str, stat: &str) -> String {
    let path_buf:PathBuf = PathBuf::from(path);
    fs::read_to_string(path_buf.join(stat))
        .unwrap_or_default()  // Return an empty string "" if error
        .trim()
        .to_string()
}


fn calc_discharge_rate_per_min(curr_charge: u32, prev_charge: u32, curr_time: SystemTime, prev_time: SystemTime) -> f64 {
    let charge_diff = prev_charge - curr_charge;
    let time_diff = curr_time.duration_since(prev_time)
                    .unwrap_or_default()
                    .as_secs();
    let discharge_rate_per_sec = (charge_diff as f64) / (time_diff as f64);
    discharge_rate_per_sec * 60.
}

pub fn get_battery_metrics(batt_stat: &mut VecDeque<HistoricalBattStat>) -> (u32, u32, String) {
    let battery_paths = find_battery_paths();
    let now = SystemTime::now();
    // let five_mins_ago = now - Duration::from_secs(300);
    
    match battery_paths.len() {
        0 => (0, 0, "No batt detected".to_string()),
        1 => {
            // Get status
            let status = get_batt_stat(&battery_paths[0], "status");

            // Get voltage and amperage, 
            // ( and if available ) calculate power 
            // & store data 
            let mut power_now: u32 = 0;
            let voltage_now = (get_batt_stat(&battery_paths[0], "voltage_now")).parse::<u32>().unwrap_or(0);
            let current_now = (get_batt_stat(&battery_paths[0], "current_now")).parse::<u32>().unwrap_or(0);

            if voltage_now !=0 && current_now !=0 {
                power_now = (voltage_now / 1_000_000) * (current_now / 100_000);
            }

            let batt_stat_list_length = batt_stat.len();

            // If status is "Discharging"
            // Calculate the discharge rate
            let timestamp = SystemTime::now();
            let mut charge_value: u32 = 0;
            let mut discharge_rate:f64 = 0.;
            let mut charge_metric: String = "".to_string();
            let mut batt_time: u32 = 0;
            if &status == "Discharging" {
                // Find last non-discharging state index
                let discharge_start_idx = batt_stat.iter()
                    .rposition(|stat| stat.status != "Discharging")
                    .unwrap_or(0);

                    

                    let recent_discharge_count = &batt_stat_list_length - discharge_start_idx;

                    match recent_discharge_count {
                        0       => {
                            // check which method to use based on available metric
                            // don't calculate anything
                            // just store charge value and metric
                            let charge_now = get_batt_stat(&battery_paths[0], "charge_now").parse::<u32>().unwrap_or_default();

                            if charge_now != 0 {
                                charge_value = charge_now;
                                charge_metric = "charge_now".to_string();
                            } else {
                                let capacity = get_batt_stat(&battery_paths[0], "capacity").parse::<u32>().unwrap_or_default();
                                if capacity != 0 {
                                    charge_value = capacity;
                                    charge_metric = "capacity".to_string();
                                }
                            }
                            
                        },
                        1..=60  => {
                            // get method from stored historical stat
                            // use oldest available data for reading calculating current discharge
                            // average out discharge rate
                            // calculate time remaining

                            let last_batt_stat = &batt_stat[batt_stat_list_length-1];
                            let oldest_discharging_batt_stat = &batt_stat[discharge_start_idx + 1];

                            charge_metric = last_batt_stat.charge_metric.clone();

                            charge_value = get_batt_stat(&battery_paths[0], &charge_metric).parse::<u32>().unwrap_or_default();
                            
                            discharge_rate = calc_discharge_rate_per_min(charge_value, oldest_discharging_batt_stat.charge_value, timestamp, oldest_discharging_batt_stat.timestamp);

                            batt_time = (charge_value as f64 / discharge_rate) as u32; 

                            

                            

                            



                        },
                        _       =>  {
                            // get method from stored historical stat
                            // use data from 60 readings ago reading calculating current discharge
                            // average out discharge rate
                            // calculate time remaining

                        }
                    }

            }

            // Add new readings to batt_stat
            let new_batt_statt = HistoricalBattStat {
                timestamp,
                charge_value,
                charge_metric,
                discharge_rate,
                status: status.clone()
            };

            batt_stat.push_back(new_batt_statt);

            // Clear old readings oldes than 310
            if batt_stat_list_length > 310 {
                
            }

            // // Return results 
            (power_now, batt_time, status)

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