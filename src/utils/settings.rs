use std::fs;
use std::path::Path;


use crate::utils::types::{AppSettings, ProfileSettings};

pub static APP_SETTINGS_TEMPLATE: &str = 
r#"[units]
power = "watt"                      # watt | milliwatt
temp = "celsius"                    # celsius | fahrenheit

[style]
theme_mode = "dark"                 # dark | light
theme_light_palette = "winter"      # winter | black | nord
theme_dark_palette =  "dim"         # dracula | night | dim

[app]
start_on_login = true               # true | false
minimize_to_tray = true             # true | false
enable_logging = true               # true | false
update_frequency_ms = 1000        # 1000 | 500 | 100
logging_frequency_ms = 10000      # 10000 | 5000 | 1000
"#;

pub static PROFILE_SETTINGS_TEMPLATE: &str = 
r#"active_profile = "system"                  # system | custom | turbo | fixed
low_batt_threshold_percent = 20

[system.performance]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[system.balanced]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[system.power_saver]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[custom.ac]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[custom.batt]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[custom.low_batt]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[turbo.turbo]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0

[fixed.fixed]
fast_mw = 0
slow_mw = 0
stapm_mw = 0
temp_c = 0
"#;

pub fn read_app_settings() -> Result<AppSettings, Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let settings_file = format!("{}/.ryzone/app_settings.toml", home_dir);

    if !Path::new(&settings_file).exists() {
        let dir = format!("{}/.ryzone", home_dir);
        fs::create_dir_all(&dir)?;
        fs::write(&settings_file, APP_SETTINGS_TEMPLATE)?;
    }

    let contents = fs::read_to_string(&settings_file)?;
    let settings = toml::from_str(&contents)?;
    Ok(settings)
}

pub fn read_profile_settings() -> Result<ProfileSettings, Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let settings_file = format!("{}/.ryzone/profile_settings.toml", home_dir);

    if !Path::new(&settings_file).exists() {
        let dir = format!("{}/.ryzone", home_dir);
        fs::create_dir_all(&dir)?;
        fs::write(&settings_file, PROFILE_SETTINGS_TEMPLATE)?;
    }

    let contents = fs::read_to_string(&settings_file)?;
    let settings = toml::from_str(&contents)?;
    Ok(settings)
}

pub fn write_app_settings(settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let settings_file = format!("{}/.ryzone/app_settings.toml", home_dir);
    let contents = toml::to_string_pretty(settings)?;
    fs::write(&settings_file, contents)?;
    Ok(())
}

pub fn write_profile_settings(settings: &ProfileSettings) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME")?;
    let settings_file = format!("{}/.ryzone/profile_settings.toml", home_dir);
    let contents = toml::to_string_pretty(settings)?;
    fs::write(&settings_file, contents)?;
    Ok(())
}

