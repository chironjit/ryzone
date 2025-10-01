use std::{time::Duration};
use libryzenadj::RyzenAdj;

pub fn daemon_loop(refresh_rate: u64) {
    loop {
        daemon();
        std::thread::sleep(Duration::from_secs(refresh_rate));
    }

}

fn daemon() {
    let ryzen = RyzenAdj::new().unwrap();

    // Power limits (in mW, converted to display units)
    let curr_fast_limit = (ryzen.get_fast_limit().unwrap_or_default() * 1000.).round() as u32;
    let curr_fast_value = (ryzen.get_fast_value().unwrap_or_default() * 1000.).round() as u32;
    let curr_slow_limit = (ryzen.get_slow_limit().unwrap_or_default() * 1000.).round() as u32;
    let curr_slow_value = (ryzen.get_slow_value().unwrap_or_default() * 1000.).round() as u32;
    let curr_stapm_limit = (ryzen.get_stapm_limit().unwrap_or_default() * 1000.).round() as u32;
    let curr_stapm_value = (ryzen.get_stapm_value().unwrap_or_default() * 1000.).round() as u32;

    // Temperature values (in °C)
    let curr_tctl_limit = ryzen.get_tctl_temp().unwrap_or_default().round() as u32;
    let curr_tctl_value = ryzen.get_tctl_temp_value().unwrap_or_default().round() as u32;

    // APU skin temperature
    let apu_skin_temp_limit = ryzen.get_apu_skin_temp_limit().unwrap_or_default();
    let apu_skin_temp_value = ryzen.get_apu_skin_temp_value().unwrap_or_default();
    let apu_slow_limit = (ryzen.get_apu_slow_limit().unwrap_or_default() * 1000.).round() as u32;
    let apu_slow_value = (ryzen.get_apu_slow_value().unwrap_or_default() * 1000.).round() as u32;

    // GPU skin temperature
    let dgpu_skin_temp_limit = ryzen.get_dgpu_skin_temp_limit().unwrap_or_default();
    let dgpu_skin_temp_value = ryzen.get_dgpu_skin_temp_value().unwrap_or_default();

    // Clock frequencies (in MHz)
    let fclk = ryzen.get_fclk().unwrap_or_default();
    let gfx_clk = ryzen.get_gfx_clk().unwrap_or_default();
    let mem_clk = ryzen.get_mem_clk().unwrap_or_default();
    let l3_clk = ryzen.get_l3_clk().unwrap_or_default();
    let cclk_setpoint = ryzen.get_cclk_setpoint().unwrap_or_default();
    let cclk_busy_value = ryzen.get_cclk_busy_value().unwrap_or_default();

    // Voltages
    let gfx_volt = ryzen.get_gfx_volt().unwrap_or_default();
    let soc_volt = ryzen.get_soc_volt().unwrap_or_default();
    let l3_logic = ryzen.get_l3_logic().unwrap_or_default();
    let l3_vddm = ryzen.get_l3_vddm().unwrap_or_default();

    // Temperatures
    let gfx_temp = ryzen.get_gfx_temp().unwrap_or_default();
    let l3_temp = ryzen.get_l3_temp().unwrap_or_default();

    // Power consumption
    let soc_power = ryzen.get_soc_power().unwrap_or_default();
    let socket_power = ryzen.get_socket_power().unwrap_or_default();

    // Current limits and values
    let psi0_current = ryzen.get_psi0_current().unwrap_or_default();
    let psi0soc_current = ryzen.get_psi0soc_current().unwrap_or_default();
    let vrm_current = ryzen.get_vrm_current().unwrap_or_default();
    let vrm_current_value = ryzen.get_vrm_current_value().unwrap_or_default();
    let vrmmax_current = ryzen.get_vrmmax_current().unwrap_or_default();
    let vrmmax_current_value = ryzen.get_vrmmax_current_value().unwrap_or_default();
    let vrmsoc_current = ryzen.get_vrmsoc_current().unwrap_or_default();
    let vrmsoc_current_value = ryzen.get_vrmsoc_current_value().unwrap_or_default();
    let vrmsocmax_current = ryzen.get_vrmsocmax_current().unwrap_or_default();
    let vrmsocmax_current_value = ryzen.get_vrmsocmax_current_value().unwrap_or_default();

    // Time values
    let slow_time = ryzen.get_slow_time().unwrap_or_default();
    let stapm_time = ryzen.get_stapm_time().unwrap_or_default();

    // System info
    let cpu_family = ryzen.get_cpu_family().unwrap_or(libryzenadj::RyzenFamily::Unknow);
    let table_ver = ryzen.get_table_ver().unwrap_or_default();
    let bios_if_ver = ryzen.get_bios_if_ver().unwrap_or_default();

    // Print power limits
    println!("=== Power Limits (mW) ===");
    println!("Fast limit: {} | Fast value: {}", curr_fast_limit, curr_fast_value);
    println!("Slow limit: {} | Slow value: {}", curr_slow_limit, curr_slow_value);
    println!("STAPM limit: {} | STAPM value: {}", curr_stapm_limit, curr_stapm_value);
    println!("APU slow limit: {} | APU slow value: {}", apu_slow_limit, apu_slow_value);

    // Print temperatures
    println!("\n=== Temperatures (°C) ===");
    println!("Tctl limit: {} | Tctl value: {}", curr_tctl_limit, curr_tctl_value);
    println!("APU skin limit: {:.1} | APU skin value: {:.1}", apu_skin_temp_limit, apu_skin_temp_value);
    println!("dGPU skin limit: {:.1} | dGPU skin value: {:.1}", dgpu_skin_temp_limit, dgpu_skin_temp_value);
    println!("GFX temp: {:.1} | L3 temp: {:.1}", gfx_temp, l3_temp);

    // Print frequencies
    println!("\n=== Frequencies (MHz) ===");
    println!("FCLK: {:.0} | GFX CLK: {:.0} | MEM CLK: {:.0}", fclk, gfx_clk, mem_clk);
    println!("L3 CLK: {:.0} | CCLK setpoint: {:.0} | CCLK busy: {:.0}", l3_clk, cclk_setpoint, cclk_busy_value);

    // Print voltages
    println!("\n=== Voltages (V) ===");
    println!("GFX volt: {:.3} | SoC volt: {:.3}", gfx_volt, soc_volt);
    println!("L3 logic: {:.3} | L3 VDDM: {:.3}", l3_logic, l3_vddm);

    // Print power consumption
    println!("\n=== Power Consumption (W) ===");
    println!("SoC power: {:.2} | Socket power: {:.2}", soc_power, socket_power);

    // Print current limits
    println!("\n=== Current Limits (mA) ===");
    println!("PSI0: {:.0} | PSI0 SoC: {:.0}", psi0_current, psi0soc_current);
    println!("VRM: {:.0} ({:.0}) | VRM Max: {:.0} ({:.0})", vrm_current, vrm_current_value, vrmmax_current, vrmmax_current_value);
    println!("VRM SoC: {:.0} ({:.0}) | VRM SoC Max: {:.0} ({:.0})", vrmsoc_current, vrmsoc_current_value, vrmsocmax_current, vrmsocmax_current_value);

    // Print timing
    println!("\n=== Timing (s) ===");
    println!("Slow time: {:.1} | STAPM time: {:.1}", slow_time, stapm_time);

    // Print system info
    println!("\n=== System Info ===");
    println!("CPU Family: {:?} | Table ver: {} | BIOS IF ver: {}", cpu_family, table_ver, bios_if_ver);

    println!("\n{}", "=".repeat(50));

    let random_value = 20000 + (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() % 15001);
    let _ = ryzen.set_fast_limit(random_value);
}