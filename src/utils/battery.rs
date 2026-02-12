use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct BatterySnapshot {
    pub charge_status: String,
    pub charge_percent: i32,
    pub design_capacity_mwh: i32,
    pub full_charge_capacity_mwh: i32,
    pub current_capacity_mwh: i32,
    pub health_percent: i32,
    pub voltage_millivolt: i32,
    pub cycle_count_cycles: i32,
    pub temperature_c: i32,
    pub power_draw_mw: i32,
}

pub fn read_battery_snapshot() -> io::Result<Option<BatterySnapshot>> {
    let Some(battery_dir) = find_battery_dir("/sys/class/power_supply")? else {
        return Ok(None);
    };

    let status = normalize_status(read_trimmed(&battery_dir.join("status")).as_deref());
    let charge_percent = read_i32(&battery_dir.join("capacity")).unwrap_or(0).clamp(0, 100);

    let voltage_uv = read_i64(&battery_dir.join("voltage_now")).unwrap_or(0);
    let design_voltage_uv = read_i64(&battery_dir.join("voltage_min_design"))
        .or_else(|| read_i64(&battery_dir.join("voltage_max_design")))
        .unwrap_or(0);
    let voltage_millivolt = if voltage_uv > 0 {
        (voltage_uv / 1_000) as i32
    } else {
        0
    };

    // Design/full capacities should come from BMS capacity files.
    // If only charge_* is available, convert with design voltage (not voltage_now).
    let design_capacity_mwh = read_energy_mwh(&battery_dir, "energy_full_design")
        .or_else(|| {
            charge_to_mwh(
                read_i64(&battery_dir.join("charge_full_design"))?,
                design_voltage_uv,
            )
        })
        .unwrap_or(-1);
    let full_charge_capacity_mwh = read_energy_mwh(&battery_dir, "energy_full")
        .or_else(|| {
            charge_to_mwh(
                read_i64(&battery_dir.join("charge_full"))?,
                design_voltage_uv,
            )
        })
        .unwrap_or(-1);
    let current_capacity_mwh = read_energy_mwh(&battery_dir, "energy_now")
        .or_else(|| charge_to_mwh(read_i64(&battery_dir.join("charge_now"))?, voltage_uv))
        .unwrap_or(0);

    let health_percent = if design_capacity_mwh > 0 && full_charge_capacity_mwh > 0 {
        (((full_charge_capacity_mwh as i64 * 100) / design_capacity_mwh as i64) as i32).clamp(0, 100)
    } else {
        -1
    };

    let cycle_count_cycles = read_i32(&battery_dir.join("cycle_count")).unwrap_or(0).max(0);

    let temperature_c = read_temperature_c(&battery_dir).unwrap_or(-1);

    let power_draw_mw = read_power_mw(&battery_dir, voltage_uv).unwrap_or(0).max(0);

    Ok(Some(BatterySnapshot {
        charge_status: status,
        charge_percent,
        design_capacity_mwh,
        full_charge_capacity_mwh,
        current_capacity_mwh,
        health_percent,
        voltage_millivolt,
        cycle_count_cycles,
        temperature_c,
        power_draw_mw,
    }))
}

fn find_battery_dir(base: &str) -> io::Result<Option<PathBuf>> {
    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else {
            continue;
        };
        if name.starts_with("BAT") && entry.path().is_dir() {
            return Ok(Some(entry.path()));
        }
    }
    Ok(None)
}

fn read_trimmed(path: &Path) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn read_i32(path: &Path) -> Option<i32> {
    read_trimmed(path)?.parse::<i32>().ok()
}

fn read_i64(path: &Path) -> Option<i64> {
    read_trimmed(path)?.parse::<i64>().ok()
}

fn read_energy_mwh(battery_dir: &Path, file_name: &str) -> Option<i32> {
    let value_uwh = read_i64(&battery_dir.join(file_name))?;
    Some((value_uwh / 1000) as i32)
}

fn charge_to_mwh(charge_uah: i64, voltage_uv: i64) -> Option<i32> {
    if charge_uah <= 0 || voltage_uv <= 0 {
        return None;
    }
    let mwh = (charge_uah as i128 * voltage_uv as i128) / 1_000_000_000;
    Some(mwh as i32)
}

fn read_temperature_c(battery_dir: &Path) -> Option<i32> {
    if let Some(temp_raw) = read_i64(&battery_dir.join("temp")) {
        return Some(scale_temperature_c(temp_raw));
    }
    if let Some(temp_raw) = read_i64(&battery_dir.join("temperature")) {
        return Some(scale_temperature_c(temp_raw));
    }
    None
}

fn scale_temperature_c(raw: i64) -> i32 {
    if raw > 1000 {
        (raw / 1000) as i32
    } else if raw > 200 {
        (raw / 10) as i32
    } else {
        raw as i32
    }
}

fn read_power_mw(battery_dir: &Path, voltage_uv: i64) -> Option<i32> {
    if let Some(power_uw) = read_i64(&battery_dir.join("power_now")) {
        return Some((power_uw / 1000) as i32);
    }

    let current_ua = read_i64(&battery_dir.join("current_now"))?;
    if current_ua <= 0 || voltage_uv <= 0 {
        return None;
    }
    let power_mw = (current_ua as i128 * voltage_uv as i128) / 1_000_000_000;
    Some(power_mw as i32)
}

fn normalize_status(status: Option<&str>) -> String {
    match status.unwrap_or("").trim().to_lowercase().as_str() {
        "charging" => "charging".to_string(),
        "discharging" => "discharging".to_string(),
        "full" => "full".to_string(),
        "not charging" => "full".to_string(),
        "empty" => "empty".to_string(),
        _ => "na".to_string(),
    }
}
