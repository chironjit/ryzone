use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use dioxus::prelude::*;
use libryzenadj::RyzenAdj;

use crate::utils::battery::read_battery_snapshot;
use crate::utils::types::{CurrentStats, PowerLimits, ProfileSettings};

// ─── Dioxus hook: entry point ────────────────────────────────────────────────
// Spawns the background polling thread. Creates RyzenAdj once, then loops.
pub fn use_current_stats_signal(
    update_frequency_ms: i32,
    profile_settings: SyncSignal<ProfileSettings>,
) -> SyncSignal<CurrentStats> {
    let stats_signal = use_signal_sync(CurrentStats::default);
    let mut worker_signal = stats_signal;
    let interval_ms = update_frequency_ms.max(250) as u64;

    use_hook(move || {
        std::thread::spawn(move || {
            // Create RyzenAdj once at thread startup
            let adj = match RyzenAdj::new() {
                Ok(adj) => Some(adj),
                Err(e) => {
                    eprintln!(
                        "[ryzenadj] Failed to initialize: {:?}. CPU/power stats unavailable.",
                        e
                    );
                    None
                }
            };

            loop {
                let ps = profile_settings.peek().clone();
                if let Ok(next_stats) = read_current_stats(adj.as_ref(), &ps) {
                    worker_signal.set(next_stats);
                }
                std::thread::sleep(Duration::from_millis(interval_ms));
            }
        });
    });

    stats_signal
}

// ─── Per-cycle orchestrator ──────────────────────────────────────────────────
// Called once per tick. Fills all CurrentStats fields, resolves profile, enforces limits.
fn read_current_stats(
    adj: Option<&RyzenAdj>,
    profile_settings: &ProfileSettings,
) -> Result<CurrentStats, String> {
    let mut stats = CurrentStats::default();

    // 1. Fill battery stats
    fill_battery_stats(&mut stats)?;

    // 2. Fill ryzenadj stats (CPU, GPU, power limits)
    if let Some(adj) = adj {
        if let Err(e) = fill_ryzenadj_stats(adj, &mut stats) {
            eprintln!("[ryzenadj] stats read error: {}", e);
        }
    }

    // 3. Fill runtime estimates
    fill_runtime_estimates(&mut stats);

    // 4. Resolve target profile
    let (target_limits, profile_name, sub_profile_name) = resolve_target_profile(
        profile_settings,
        &stats.batt_charge_status,
        stats.batt_charge_percent,
    );
    stats.profile = profile_name;
    stats.sub_profile = sub_profile_name;

    // 5. Enforce profile limits if they differ from target
    if let Some(adj) = adj {
        if let Err(e) = enforce_profile_limits(adj, &stats, &target_limits) {
            eprintln!("[ryzenadj] enforce limits error: {}", e);
        }
    }

    Ok(stats)
}

// ─── Battery ─────────────────────────────────────────────────────────────────
// Reads battery snapshot and fills battery + power_draw fields in stats.
fn fill_battery_stats(stats: &mut CurrentStats) -> Result<(), String> {
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
    } else {
        stats.batt_charge_status = "na".to_string();
    }
    Ok(())
}

// Computes runtime estimates from battery fields already in stats.
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

// ─── RyzenAdj data ──────────────────────────────────────────────────────────
// Refreshes SMU table and fills CPU, GPU, power, and limit fields.
fn fill_ryzenadj_stats(adj: &RyzenAdj, stats: &mut CurrentStats) -> Result<(), String> {
    adj.refresh()
        .map_err(|e| format!("refresh failed: {:?}", e))?;

    // CPU frequency: max across all cores (MHz)
    // ryzenadj returns MHz as float
    let mut max_core_clk: f32 = 0.0;
    for core in 0..32u32 {
        match adj.get_core_clk(core) {
            Ok(clk) if clk > 0.0 => max_core_clk = max_core_clk.max(clk),
            _ => break,
        }
    }
    stats.cpu_frequency_mhz = max_core_clk.round() as i32;

    // CPU temperature (tctl value, celsius)
    stats.cpu_temperature_c = adj.get_tctl_temp_value().unwrap_or(0.0).round() as i32;

    // CPU load (cclk busy percentage)
    stats.cpu_load_percent = adj.get_cclk_busy_value().unwrap_or(0.0).round() as i32;

    // GPU
    stats.gpu_frequency_mhz = adj.get_gfx_clk().unwrap_or(0.0).round() as i32;
    stats.gpu_temperature_c = adj.get_gfx_temp().unwrap_or(0.0).round() as i32;
    stats.gpu_load_percent = read_gpu_load_percent();

    // Power limits - ryzenadj returns watts (float), we store milliwatts (i32)
    stats.curr_fast_limit_mw = watts_to_mw(adj.get_fast_limit().unwrap_or(0.0));
    stats.curr_fast_value_mw = watts_to_mw(adj.get_fast_value().unwrap_or(0.0));
    stats.curr_slow_limit_mw = watts_to_mw(adj.get_slow_limit().unwrap_or(0.0));
    stats.curr_slow_value_mw = watts_to_mw(adj.get_slow_value().unwrap_or(0.0));
    stats.curr_stapm_limit_mw = watts_to_mw(adj.get_stapm_limit().unwrap_or(0.0));
    stats.curr_stapm_value_mw = watts_to_mw(adj.get_stapm_value().unwrap_or(0.0));
    stats.curr_tctl_limit_c = adj.get_tctl_temp().unwrap_or(0.0).round() as i32;
    stats.curr_tctl_value_c = adj.get_tctl_temp_value().unwrap_or(0.0).round() as i32;

    // Percentages (value / limit * 100), clamped 0..100
    stats.curr_fast_percent = safe_percent(stats.curr_fast_value_mw, stats.curr_fast_limit_mw);
    stats.curr_slow_percent = safe_percent(stats.curr_slow_value_mw, stats.curr_slow_limit_mw);
    stats.curr_stapm_percent = safe_percent(stats.curr_stapm_value_mw, stats.curr_stapm_limit_mw);
    stats.curr_tctl_percent = safe_percent(stats.curr_tctl_value_c, stats.curr_tctl_limit_c);

    Ok(())
}

// Reads iGPU load from sysfs (not available via ryzenadj).
fn read_gpu_load_percent() -> i32 {
    for card in &["card0", "card1", "card2"] {
        let path = format!("/sys/class/drm/{}/device/gpu_busy_percent", card);
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(val) = content.trim().parse::<i32>() {
                return val.clamp(0, 100);
            }
        }
    }
    0
}

// ─── Profile resolution ─────────────────────────────────────────────────────
// Determines which PowerLimits should be active based on profile settings,
// battery status, and (for "system" mode) the OS power profile.
// Returns (target_limits, profile_name, sub_profile_name).
fn resolve_target_profile(
    profile_settings: &ProfileSettings,
    battery_status: &str,
    battery_percent: i32,
) -> (PowerLimits, String, String) {
    match profile_settings.active_profile.as_str() {
        "system" => {
            let os_profile = read_system_power_profile();
            let (limits, sub) = match os_profile.as_str() {
                "performance" => (profile_settings.system.performance.clone(), "performance"),
                "power_saver" => (profile_settings.system.power_saver.clone(), "power_saver"),
                _ => (profile_settings.system.balanced.clone(), "balanced"),
            };
            (limits, "system".to_string(), sub.to_string())
        }
        "custom" => {
            let is_on_battery = battery_status == "discharging";
            let is_low_battery =
                is_on_battery && battery_percent < profile_settings.low_batt_threshold_percent;

            if is_low_battery {
                (
                    profile_settings.custom.low_batt.clone(),
                    "custom".to_string(),
                    "low_batt".to_string(),
                )
            } else if is_on_battery {
                (
                    profile_settings.custom.batt.clone(),
                    "custom".to_string(),
                    "batt".to_string(),
                )
            } else {
                (
                    profile_settings.custom.ac.clone(),
                    "custom".to_string(),
                    "ac".to_string(),
                )
            }
        }
        "turbo" => (
            profile_settings.turbo.turbo.clone(),
            "turbo".to_string(),
            "turbo".to_string(),
        ),
        "fixed" => (
            profile_settings.fixed.fixed.clone(),
            "fixed".to_string(),
            "fixed".to_string(),
        ),
        _ => (
            profile_settings.system.balanced.clone(),
            "system".to_string(),
            "balanced".to_string(),
        ),
    }
}

// Reads the OS-level power profile. Tries sysfs first, falls back to
// powerprofilesctl, defaults to "balanced".
fn read_system_power_profile() -> String {
    // Try sysfs platform_profile first (works on most AMD laptops)
    if let Ok(content) = fs::read_to_string("/sys/firmware/acpi/platform_profile") {
        let profile = content.trim().to_lowercase();
        return match profile.as_str() {
            "low-power" => "power_saver".to_string(),
            "balanced" => "balanced".to_string(),
            "performance" => "performance".to_string(),
            _ => "balanced".to_string(),
        };
    }

    // Fallback: query power-profiles-daemon via CLI
    if let Ok(output) = Command::new("powerprofilesctl").arg("get").output() {
        if output.status.success() {
            let profile = String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_lowercase();
            return match profile.as_str() {
                "power-saver" => "power_saver".to_string(),
                "balanced" => "balanced".to_string(),
                "performance" => "performance".to_string(),
                _ => "balanced".to_string(),
            };
        }
    }

    // Default
    "balanced".to_string()
}

// ─── Profile enforcement ────────────────────────────────────────────────────
// Compares current ryzenadj limits against target. Applies via set_* if they differ.
fn enforce_profile_limits(
    adj: &RyzenAdj,
    stats: &CurrentStats,
    target: &PowerLimits,
) -> Result<(), String> {
    let tolerance_mw = 500; // 500 mW tolerance for power limits
    let tolerance_c = 1; // 1 °C tolerance for temperature

    let fast_diff = (stats.curr_fast_limit_mw - target.fast_mw).abs() > tolerance_mw;
    let slow_diff = (stats.curr_slow_limit_mw - target.slow_mw).abs() > tolerance_mw;
    let stapm_diff = (stats.curr_stapm_limit_mw - target.stapm_mw).abs() > tolerance_mw;
    let tctl_diff = (stats.curr_tctl_limit_c - target.temp_c).abs() > tolerance_c;

    if !(fast_diff || slow_diff || stapm_diff || tctl_diff) {
        return Ok(()); // All within tolerance, nothing to do
    }

    if fast_diff {
        adj.set_fast_limit(target.fast_mw as u32)
            .map_err(|e| format!("set_fast_limit: {:?}", e))?;
    }
    if slow_diff {
        adj.set_slow_limit(target.slow_mw as u32)
            .map_err(|e| format!("set_slow_limit: {:?}", e))?;
    }
    if stapm_diff {
        adj.set_stapm_limit(target.stapm_mw as u32)
            .map_err(|e| format!("set_stapm_limit: {:?}", e))?;
    }
    if tctl_diff {
        adj.set_tctl_temp(target.temp_c as u32)
            .map_err(|e| format!("set_tctl_temp: {:?}", e))?;
    }

    Ok(())
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn watts_to_mw(watts: f32) -> i32 {
    (watts * 1000.0).round() as i32
}

fn safe_percent(value: i32, limit: i32) -> i32 {
    if limit <= 0 {
        0
    } else {
        ((value as i64 * 100) / limit as i64).clamp(0, 100) as i32
    }
}
