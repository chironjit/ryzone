use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AppSettings {
    pub units: Units,
    pub style: Style,
    pub app: App,
}

#[derive(Deserialize, Serialize)]
pub struct Units {
    pub power: String,
    pub temp: String,
}

#[derive(Deserialize, Serialize)]
pub struct Style {
    pub theme_mode: String,
    pub theme_light_palette: String,
    pub theme_dark_palette: String,
}

#[derive(Deserialize, Serialize)]
pub struct App {
    pub start_on_login: bool,
    pub minimize_to_tray: bool,
    pub enable_logging: bool,
    pub update_frequency: i32,
    pub logging_frequency: i32,
}

#[derive(Deserialize, Serialize)]
pub struct ProfileSettings {
    pub profile: String,
    pub low_batt_threshold: i32,
    pub system: SystemProfiles,
    pub custom: CustomProfiles,
    pub turbo: TurboProfile,
    pub fixed: FixedProfile,
}

#[derive(Deserialize, Serialize)]
pub struct SystemProfiles {
    pub performance: PowerLimits,
    pub balanced: PowerLimits,
    pub power_saver: PowerLimits,
}

#[derive(Deserialize, Serialize)]
pub struct CustomProfiles {
    pub ac: PowerLimits,
    pub batt: PowerLimits,
    pub low_batt: PowerLimits,
}

#[derive(Deserialize, Serialize)]
pub struct TurboProfile {
    pub turbo: PowerLimits,
}

#[derive(Deserialize, Serialize)]
pub struct FixedProfile {
    pub fixed: PowerLimits,
}

#[derive(Deserialize, Serialize)]
pub struct PowerLimits {
    pub fast: i32,
    pub slow: i32,
    pub stapm: i32,
    pub temp: i32,
}



pub static APP_SETTINGS_TEMPLATE: &str = 
r#"[units]
power = "watt"                      # watt | mwatt
temp = "celcius"                    # celcius | fahrenheit

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
r#"profile = "system"                  # system | custom | turbo | fixed
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

