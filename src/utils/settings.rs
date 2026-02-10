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
update_frequency = 1                # 10 | 5 | 1
logging_frequency = 1               # 60 | 30 | 10 | 5 | 1
"#;

pub static PROFILE_SETTINGS_TEMPLATE: &str = 
r#"active_profile = "system"                  # system | custom | turbo | fixed
low_batt_threshold = 20

[system.performance]
fast = 0
slow = 0
stapm = 0
temp = 0

[system.balanced]
fast = 0
slow = 0
stapm = 0
temp = 0

[system.power_saver]
fast = 0
slow = 0
stapm = 0
temp = 0

[custom.ac]
fast = 0
slow = 0
stapm = 0
temp = 0

[custom.batt]
fast = 0
slow = 0
stapm = 0
temp = 0

[custom.low_batt]
fast = 0
slow = 0
stapm = 0
temp = 0

[turbo.turbo]
fast = 0
slow = 0
stapm = 0
temp = 0

[fixed.fixed]
fast = 0
slow = 0
stapm = 0
temp = 0
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

