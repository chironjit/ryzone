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
    }

    Ok(())
}
