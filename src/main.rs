use clap::{Parser, Subcommand};
use libryzenadj::RyzenAdj;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get current CPU family
    Family,
    /// Get current CPU temperature
    Temp,
    /// Set stapm limit (sustained power limit)
    SetStapm {
        /// Value in mW (milliwatts)
        #[arg(value_parser = clap::value_parser!(u32).range(1000..100000))]
        value: u32,
    },
    /// Get all available system information
    Info,
}

// Helper function to format optional float values
fn format_value<T: std::fmt::Display>(value: Result<T, Box<dyn std::error::Error>>, unit: &str) -> String {
    match value {
        Ok(val) => format!("{}{}", val, unit),
        Err(_) => "N/A".to_string(),
    }
}

fn print_info(adj: &RyzenAdj) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RyzenAdj System Information ===\n");
    
    // System Information
    println!("System Information:");
    if let Ok(family) = adj.get_cpu_family() {
        println!("CPU Family: {:?}", family);
    }
    if let Ok(bios) = adj.get_bios_if_ver() {
        println!("BIOS Interface Version: {}", bios);
    }
    
    // Temperature Information
    println!("\nTemperature Information:");
    println!("Tctl Temperature: {}", format_value(adj.get_tctl_temp().map_err(Box::from), "°C"));
    println!("Tctl Temperature Value: {}", format_value(adj.get_tctl_temp_value().map_err(Box::from), "°C"));
    println!("Graphics Temperature: {}", format_value(adj.get_gfx_temp().map_err(Box::from), "°C"));
    
    // Power Limits
    println!("\nPower Limits:");
    println!("STAPM Limit: {}", format_value(adj.get_stapm_limit().map_err(Box::from), " mW"));
    println!("STAPM Value: {}", format_value(adj.get_stapm_value().map_err(Box::from), " mW"));
    println!("STAPM Time: {}", format_value(adj.get_stapm_time().map_err(Box::from), " s"));
    println!("Fast Limit: {}", format_value(adj.get_fast_limit().map_err(Box::from), " mW"));
    println!("Fast Value: {}", format_value(adj.get_fast_value().map_err(Box::from), " mW"));
    println!("Slow Limit: {}", format_value(adj.get_slow_limit().map_err(Box::from), " mW"));
    println!("Slow Value: {}", format_value(adj.get_slow_value().map_err(Box::from), " mW"));
    println!("Slow Time: {}", format_value(adj.get_slow_time().map_err(Box::from), " s"));
    
    // Power Measurements
    println!("\nPower Measurements:");
    println!("Socket Power: {}", format_value(adj.get_socket_power().map_err(Box::from), " mW"));
    println!("SoC Power: {}", format_value(adj.get_soc_power().map_err(Box::from), " mW"));
    
    // Clock Speeds
    println!("\nClock Speeds:");
    println!("Graphics Clock: {}", format_value(adj.get_gfx_clk().map_err(Box::from), " MHz"));
    println!("Memory Clock: {}", format_value(adj.get_mem_clk().map_err(Box::from), " MHz"));
    println!("FCLK: {}", format_value(adj.get_fclk().map_err(Box::from), " MHz"));
    println!("L3 Cache Clock: {}", format_value(adj.get_l3_clk().map_err(Box::from), " MHz"));
    
    // Core Information
    println!("\nPer-Core Information:");
    for core in 0..8 {
        let clock = adj.get_core_clk(core).map_err(Box::from);
        let temp = adj.get_core_temp(core).map_err(Box::from);
        let volt = adj.get_core_volt(core).map_err(Box::from);
        let power = adj.get_core_power(core).map_err(Box::from);
        
        // Only print core info if at least one value is available
        if clock.is_ok() || temp.is_ok() || volt.is_ok() || power.is_ok() {
            println!("\nCore {}:", core);
            println!("  Clock: {}", format_value(clock, " MHz"));
            println!("  Temperature: {}", format_value(temp, "°C"));
            println!("  Voltage: {}", format_value(volt, " V"));
            println!("  Power: {}", format_value(power, " mW"));
        }
    }
    
    // Voltage Information
    println!("\nVoltage Information:");
    println!("Graphics Voltage: {}", format_value(adj.get_gfx_volt().map_err(Box::from), " V"));
    println!("SoC Voltage: {}", format_value(adj.get_soc_volt().map_err(Box::from), " V"));
    println!("L3 Logic Voltage: {}", format_value(adj.get_l3_logic().map_err(Box::from), " V"));
    println!("L3 VDDM: {}", format_value(adj.get_l3_vddm().map_err(Box::from), " V"));
    
    // Current Information
    println!("\nCurrent Information:");
    println!("VRM Current: {}", format_value(adj.get_vrm_current().map_err(Box::from), " A"));
    println!("VRM Current Value: {}", format_value(adj.get_vrm_current_value().map_err(Box::from), " A"));
    println!("VRM Maximum Current: {}", format_value(adj.get_vrmmax_current().map_err(Box::from), " A"));
    println!("VRM Maximum Current Value: {}", format_value(adj.get_vrmmax_current_value().map_err(Box::from), " A"));
    println!("VRM SoC Current: {}", format_value(adj.get_vrmsoc_current().map_err(Box::from), " A"));
    println!("VRM SoC Current Value: {}", format_value(adj.get_vrmsoc_current_value().map_err(Box::from), " A"));
    println!("VRM SoC Maximum Current: {}", format_value(adj.get_vrmsocmax_current().map_err(Box::from), " A"));
    println!("VRM SoC Maximum Current Value: {}", format_value(adj.get_vrmsocmax_current_value().map_err(Box::from), " A"));
    println!("PSI0 Current: {}", format_value(adj.get_psi0_current().map_err(Box::from), " A"));
    println!("PSI0 SoC Current: {}", format_value(adj.get_psi0soc_current().map_err(Box::from), " A"));
    
    // Additional Metrics
    println!("\nAdditional Metrics:");
    println!("CCLK Busy Value: {}", format_value(adj.get_cclk_busy_value().map_err(Box::from), "%"));
    println!("CCLK Setpoint: {}", format_value(adj.get_cclk_setpoint().map_err(Box::from), ""));
    
    // Skin Temperature Metrics
    println!("\nSkin Temperature Information:");
    println!("APU Skin Temperature Limit: {}", format_value(adj.get_apu_skin_temp_limit().map_err(Box::from), "°C"));
    println!("APU Skin Temperature Value: {}", format_value(adj.get_apu_skin_temp_value().map_err(Box::from), "°C"));
    println!("dGPU Skin Temperature Limit: {}", format_value(adj.get_dgpu_skin_temp_limit().map_err(Box::from), "°C"));
    println!("dGPU Skin Temperature Value: {}", format_value(adj.get_dgpu_skin_temp_value().map_err(Box::from), "°C"));
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let adj = RyzenAdj::new()?;
    
    match cli.command {
        Commands::Family => {
            let family = adj.get_cpu_family()?;
            println!("CPU Family: {:?}", family);
        }
        Commands::Temp => {
            let temp = adj.get_tctl_temp()?;
            println!("CPU Temperature: {:.1}°C", temp);
        }
        Commands::SetStapm { value } => {
            adj.set_stapm_limit(value)?;
            println!("Set STAPM limit to {} mW", value);
        }
        Commands::Info => {
            print_info(&adj)?;
        }
    }
    
    Ok(())
}