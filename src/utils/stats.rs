use std::time::Duration;

use dioxus::prelude::*;

use crate::utils::battery::read_battery_snapshot;
use crate::utils::types::CurrentStats;

pub fn use_current_stats_signal(update_frequency_ms: i32) -> SyncSignal<CurrentStats> {
    let stats_signal = use_signal_sync(CurrentStats::default);
    let mut worker_signal = stats_signal;
    let interval_ms = update_frequency_ms.max(250) as u64;

    use_hook(move || {
        std::thread::spawn(move || {
            loop {
                if let Ok(next_stats) = read_current_stats() {
                    worker_signal.set(next_stats);
                }
                std::thread::sleep(Duration::from_millis(interval_ms));
            }
        });
    });

    stats_signal
}

pub fn read_current_stats() -> Result<CurrentStats, String> {
    let mut stats = CurrentStats::default();
    update_current_stats(&mut stats)?;
    Ok(stats)
}

pub fn update_current_stats(stats: &mut CurrentStats) -> Result<(), String> {
    let snapshot = read_battery_snapshot().map_err(|e| e.to_string())?;
    if let Some(battery) = snapshot {
        stats.batt_charge_status = battery.charge_status;
        stats.batt_charge_percent = battery.charge_percent;
        stats.batt_design_capacity_mwh = battery.design_capacity_mwh;
        stats.batt_full_charge_capacity_mwh = battery.full_charge_capacity_mwh;
        stats.batt_current_capacity_mwh = battery.current_capacity_mwh;
        stats.batt_health_percent = battery.health_percent;
        stats.batt_voltage_millivolt = battery.voltage_millivolt;
        stats.batt_cycle_count_cycles = battery.cycle_count_cycles;
        stats.batt_temperature_c = battery.temperature_c;
        stats.power_draw_mw = battery.power_draw_mw;

        fill_runtime_estimates(stats);
    } else {
        stats.batt_charge_status = "na".to_string();
    }

    Ok(())
}

fn fill_runtime_estimates(stats: &mut CurrentStats) {
    let current_capacity_mwh = stats.batt_current_capacity_mwh.max(0);
    let full_capacity_mwh = stats.batt_full_charge_capacity_mwh.max(0);
    let power_draw_mw = stats.power_draw_mw.max(0);
    let charging = stats.batt_charge_status == "charging";
    let discharging = stats.batt_charge_status == "discharging";

    if power_draw_mw == 0 {
        stats.current_load_min = 0;
        stats.light_usage_min = 0;
        stats.heavy_usage_min = 0;
        stats.avg_discharge_rate_mw = 0;
        return;
    }

    let remaining_min = ((current_capacity_mwh as i64 * 60) / power_draw_mw as i64) as i32;
    let to_full_capacity_mwh = (full_capacity_mwh - current_capacity_mwh).max(0);
    let to_full_min = ((to_full_capacity_mwh as i64 * 60) / power_draw_mw as i64) as i32;

    stats.current_load_min = if charging { to_full_min } else { remaining_min };
    stats.light_usage_min = ((remaining_min as f32) * 1.35) as i32;
    stats.heavy_usage_min = ((remaining_min as f32) * 0.7) as i32;
    stats.avg_discharge_rate_mw = if discharging { power_draw_mw } else { 0 };
}