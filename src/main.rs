use iced::widget::{button, column, row, text, text_input, Row, container, Space, tooltip}; 
use iced::{theme, Subscription, Theme};
use iced::time::{self, Duration};
use libryzenadj::RyzenAdj;
use iced::{Length, Fill, Element, window};
use iced::alignment::{self, Horizontal, Vertical};
use iced::{Size}; 
use iced::widget::container::{Style};
use iced::{Border, Color, Shadow, Background, border, Vector, color};
use std::fs;
use std::io;
use std::collections::VecDeque;
use std::time::{SystemTime};
use glob::glob;

#[derive(Debug, Clone)]
struct HistoricalFreq {
    timestamp: SystemTime,
    freq: u32,
}

impl Default for HistoricalFreq {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            freq: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct HistoricalGpuFreq {
    timestamp: SystemTime,
    freq: u32,  // MHz
}

impl Default for HistoricalGpuFreq {
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            freq: 0,
        }
    }
}

// Constants to limit input values
const FAST_LIMIT_MIN: u32 = 4000;
const FAST_LIMIT_MAX: u32 = 65000;
const SLOW_LIMIT_MIN: u32 = 4000;
const SLOW_LIMIT_MAX: u32 = 65000;
const STAPM_LIMIT_MIN: u32 = 4000;
const STAPM_LIMIT_MAX: u32 = 65000;
const TCTL_LIMIT_MIN: u32 = 40;
const TCTL_LIMIT_MAX: u32 = 100;

#[derive(Debug, Clone)]
pub enum Message {
    SetFastLimit(u32),
    SetSlowLimit(u32),
    SetStapmLimit(u32),
    SetTctlLimit(u32),
    FastLimitInputChanged(String),
    SlowLimitInputChanged(String),
    StapmLimitInputChanged(String),
    TctlLimitInputChanged(String),
    UpdateStateValues,
}

// All the states managed by the app
#[derive(Default, Debug, Clone)]
struct State {
    // Current APU power status via libryzenadj
    curr_fast_value: u32,
    curr_slow_value: u32,
    curr_stapm_value: u32,
    curr_tctl_value: u32,
    curr_fast_limit: u32,
    curr_slow_limit: u32,
    curr_stapm_limit: u32,
    curr_tctl_limit: u32,

    // CPU frequency status (in MHz)
    current_max_freq: u32,
    min_freq_5min: u32,
    max_freq_5min: u32,
    freq_history: VecDeque<HistoricalFreq>,

    // GPU frequency status (in MHz)
    current_gpu_freq: u32,
    min_gpu_freq_5min: u32,
    max_gpu_freq_5min: u32,
    gpu_history: VecDeque<HistoricalGpuFreq>,

    // System power status
    curr_apu_power: u32,
    total_sys_power: u32,
    batt_source_power: u32,
    ext_source_power: u32,

    // Custom values input tracking
    fast_input: String,
    slow_input: String,
    stapm_input: String,
    tctl_input: String,

    // Custom override values store
    manual_fast_limit: u32,
    manual_slow_limit: u32,
    manual_stapm_limit: u32,
    manual_tctl_limit: u32,
}

// Standalone update function
fn update(
    state: &mut State,
    message: Message
) {
    match message {
        Message::SetFastLimit(value) => {
            if value >= FAST_LIMIT_MIN && value <= FAST_LIMIT_MAX {
                state.manual_fast_limit = value.into();
            }
        }
        Message::SetSlowLimit(value) => {
            if value >= SLOW_LIMIT_MIN && value <= SLOW_LIMIT_MAX {
                state.manual_slow_limit = value.into();
            }
        }
        Message::SetStapmLimit(value) => {
            if value >= STAPM_LIMIT_MIN && value <= STAPM_LIMIT_MAX {
                state.manual_stapm_limit = value.into();
            }
        }
        Message::SetTctlLimit(value) => {
            if value >= TCTL_LIMIT_MIN && value <= TCTL_LIMIT_MAX {
                state.manual_tctl_limit = value.into();
            }
        }
        Message::FastLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= FAST_LIMIT_MAX {
                        state.fast_input = value;
                    }
                }
            }
        }
        Message::SlowLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= SLOW_LIMIT_MAX {
                        state.slow_input = value;
                    }
                }
            }
        }
        Message::StapmLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= STAPM_LIMIT_MAX {
                        state.stapm_input = value;
                    }
                }
            }
        }
        Message::TctlLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u32>() {
                    if num <= TCTL_LIMIT_MAX {
                        state.tctl_input = value;
                    }
                }
            }
        }
        Message::UpdateStateValues => {
            // Every second, check what the actual values are
            // If values don't match the override
            // (and the manual values are not 0), 
            // try to apply the override value
            // Then check the latest figures and then update the state

            let ryzen = RyzenAdj::new().unwrap();


            state.curr_fast_limit = (ryzen.get_fast_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_fast_value = (ryzen.get_fast_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_slow_limit = (ryzen.get_slow_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_slow_value = (ryzen.get_slow_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_stapm_limit = (ryzen.get_stapm_limit().unwrap_or_default() * 1000.).round() as u32;
            state.curr_stapm_value = (ryzen.get_stapm_value().unwrap_or_default() * 1000.).round() as u32;
            state.curr_tctl_limit = ryzen.get_tctl_temp().unwrap_or_default().round() as u32;
            state.curr_tctl_value = ryzen.get_tctl_temp_value().unwrap_or_default().round() as u32;

            // CPU frequency updates
            let (current, min_5min, max_5min) = get_cpu_metrics(&mut state.freq_history);
            state.current_max_freq = current;
            state.min_freq_5min = min_5min;
            state.max_freq_5min = max_5min;

            // GPU frequency updates
            let (current, min_5min, max_5min) = get_gpu_metrics(&mut state.gpu_history);
            state.current_gpu_freq = current;
            state.min_gpu_freq_5min = min_5min;
            state.max_gpu_freq_5min = max_5min;


            // state.curr_apu_power = todo!() ;
            // state.total_sys_power = todo!() ;
            // state.batt_source_power = todo!() ;
            // state.ext_source_power = todo!() ;



            if state.manual_fast_limit != 0 && state.curr_fast_limit != state.manual_fast_limit {
                let _ = ryzen.set_fast_limit(state.manual_fast_limit);
            }

            if state.manual_slow_limit != 0 && state.curr_slow_limit != state.manual_slow_limit {
                let _ = ryzen.set_slow_limit(state.manual_slow_limit);
            }

            if state.manual_stapm_limit != 0 && state.curr_stapm_limit != state.manual_stapm_limit {
                let _ = ryzen.set_stapm_limit(state.manual_stapm_limit);
            }

            if state.manual_tctl_limit != 0 && state.curr_tctl_limit != state.manual_tctl_limit {
                let _ = ryzen.set_tctl_temp(state.manual_tctl_limit);
            }
        }
    }
}

fn get_cpu_frequency() -> Result<Vec<u32>, io::Error> {
    let mut frequencies = Vec::new();
    let cpu_count = fs::read_dir("/sys/devices/system/cpu")
        .unwrap()
        .filter(|entry| {
            entry.as_ref()
                .unwrap()
                .file_name()
                .to_string_lossy()
                .starts_with("cpu")
        })
        .count();

    for cpu in 0..cpu_count {
        let freq_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", cpu);
        if let Ok(freq_str) = fs::read_to_string(freq_path) {
            let freq_khz: u32 = freq_str.trim().parse().unwrap_or(0);
            frequencies.push(freq_khz); // Keep as kHz
        }
    }

    Ok(frequencies)
}

fn get_cpu_metrics(history: &mut VecDeque<HistoricalFreq>) -> (u32, u32, u32) {
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    // Read current frequencies (converting kHz to MHz)
    let mut current_max = 0;
    if let Ok(freqs) = get_cpu_frequency() {
        if !freqs.is_empty() {
            current_max = freqs.iter().max().unwrap_or(&0) / 1000; // Convert kHz to MHz
        }
    }
    
    // Add to history
    history.push_back(HistoricalFreq {
        timestamp: now,
        freq: current_max,
    });
    
    // Remove old entries
    while history.front().map_or(false, |h| h.timestamp < five_mins_ago) {
        history.pop_front();
    }
    
    // Calculate min and max from history
    let mut min_5min = current_max;
    let mut max_5min = current_max;
    
    for hist in history.iter() {
        min_5min = min_5min.min(hist.freq);
        max_5min = max_5min.max(hist.freq);
    }
    
    (current_max, min_5min, max_5min)
}

fn get_gpu_frequency() -> Result<Vec<u32>, io::Error> {
    let mut frequencies = Vec::new();
    
    // Look for AMD GPU sclk files
    for entry in glob("/sys/class/drm/card*/device/pp_dpm_sclk").unwrap() {
        if let Ok(path) = entry {
            if let Ok(content) = fs::read_to_string(path) {
                // Each line looks like: "0: 200Mhz *"
                // The * indicates which state is active
                for line in content.lines() {
                    if line.contains('*') {  // This is the active frequency
                        if let Some(freq_str) = line.split_whitespace().nth(1) {
                            if let Some(freq_num) = freq_str.trim_end_matches("Mhz").parse::<u32>().ok() {
                                frequencies.push(freq_num);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(frequencies)
}

fn get_gpu_metrics(history: &mut VecDeque<HistoricalGpuFreq>) -> (u32, u32, u32) {
    let now = SystemTime::now();
    let five_mins_ago = now - Duration::from_secs(300);
    
    // Read current frequencies
    let mut current = 0;
    if let Ok(freqs) = get_gpu_frequency() {
        if !freqs.is_empty() {
            current = freqs[0];  // Usually there's only one GPU
        }
    }
    
    // Add to history
    history.push_back(HistoricalGpuFreq {
        timestamp: now,
        freq: current,
    });
    
    // Remove old entries
    while history.front().map_or(false, |h| h.timestamp < five_mins_ago) {
        history.pop_front();
    }
    
    // Calculate min and max from history
    let mut min_5min = current;
    let mut max_5min = current;
    
    for hist in history.iter() {
        min_5min = min_5min.min(hist.freq);
        max_5min = max_5min.max(hist.freq);
    }
    
    (current, min_5min, max_5min)
}

fn format_frequency(freq: u32) -> String {
    if freq < 1000 {  // Less than 1000 MHz (1 GHz)
        format!("{} MHz", freq)
    } else {
        format!("{:.2} GHz", freq as f32 / 1000.0)
    }
}



fn card_style() -> impl Fn(&Theme) -> Style {
    |theme| Style {
        text_color: None,
        background: Some(Background::Color(theme.extended_palette().background.weak.color)),
        border: Border {
            radius: border::Radius::new(16.0),
            width: 0.0, 
            color: Color::TRANSPARENT,
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 12.0,
        },
    }
}

fn hint_text_style() -> impl Fn(&Theme) -> text::Style {
    |theme| text::Style {
        color: Some(theme.extended_palette().primary.strong.color),
        ..text::Style::default()
    }
}

fn text_input_style() -> impl Fn(&Theme, text_input::Status) -> text_input::Style {
    |theme, status| text_input::Style {
        background: Background::Color(theme.extended_palette().background.base.color),
        border: Border {
            radius: border::Radius::new(8.0),
            width: 1.0,
            color: match status {
                text_input::Status::Focused => theme.extended_palette().primary.base.color,
                _ => theme.extended_palette().secondary.weak.color,
            },
        },
        icon: theme.extended_palette().background.strong.color,
        placeholder: theme.extended_palette().background.strong.color,
        value: theme.extended_palette().primary.base.color,
        selection: theme.extended_palette().primary.weak.color,
    }
}

// Standalone view function
fn view(state: &State) -> Element<Message> {
    column![
        // Top full-width container
        container("Ryzone - Adjust mobile Ryzen APU power limits")
            .align_x(alignment::Horizontal::Center)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fixed(50.0)),

        // Row of three equal containers
        row![
            // Processor container
            container(
                column![
                    // Top section with title and main speed reading
                    row![
                        column![
                            text("CPU").size(20),
                            text("Current").size(10),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text(format_frequency(state.current_max_freq)).size(28),
                        ]
                    ]
                    .align_y(alignment::Vertical::Center)
                    .spacing(2),
            
                    Space::with_height(8),
            
                    // Bottom section with minor details
                    row![
                        column![
                            text("Min (last 5 mins)").size(12),
                            text("Max (last 5 mins)").size(12),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text(format_frequency(state.min_freq_5min)).size(12),
                            text(format_frequency(state.max_freq_5min)).size(12),
                        ]
                        .spacing(2)
                    ]
                ]
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(140.0)),

            // GPU container
            container(
                column![
                    // Top section with title and main speed reading
                    row![
                        column![
                            text("GPU").size(20),
                            text("Current").size(10),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text(format_frequency(state.current_gpu_freq)).size(28),
                        ]
                    ]
                    .align_y(alignment::Vertical::Center)
                    .spacing(2),
            
                    Space::with_height(8),
            
                    // Bottom section with minor details
                    row![
                        column![
                            text("Min (last 5 mins)").size(12),
                            text("Max (last 5 mins)").size(12),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text(format_frequency(state.min_gpu_freq_5min)).size(12),
                            text(format_frequency(state.max_gpu_freq_5min)).size(12),
                        ]
                        .spacing(2)
                    ]
                ]
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(140.0)),

            // Power container
            container(
                column![
                    // Top section with title and main power reading
                    row![
                        column![
                            text("Power").size(20),
                            text("APU Total").size(10),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text("15.1 W").size(28),
                        ]
                    ]
                    .align_y(alignment::Vertical::Center)
                    .spacing(2),
            
                    Space::with_height(8),
            
                    // Bottom section with power details
                    row![
                        column![
                            text("System total").size(12),
                            text("Source (battery)").size(12),
                            text("Source (external)").size(12),
                        ]
                        .spacing(2),
                        Space::with_width(Length::Fill),
                        column![
                            text("20.0 W").size(12),
                            text("20.0 W").size(12),
                            text("00.0 W").size(12),
                        ]
                        .spacing(2)
                    ]
                ]
            )
            .style(card_style())
            .padding(20)
            .width(Length::Fill)
            .height(Length::Fixed(140.0))
            ]
            .spacing(20)
            .width(Length::Fill),

        // Column titles row
        container(
            row![
                container(
                    container(text("").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(4)),
                container(
                    container(text("Current").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("Limit").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("Set New Limit").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(2)),
                container(
                    container(text("").size(14))
                        .align_x(alignment::Horizontal::Center)
                        .width(Length::Fill)
                )
                .width(Length::FillPortion(1)),
            ]
            .spacing(20)
        )
        .style(container::transparent)
        .padding(10)
        .width(Length::Fill),

        // Fast Limit
        container(
            row![
                container(
                    container(text("Fast Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_fast_value as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_fast_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.fast_input
                        )
                            .style(text_input_style())
                            .on_input(Message::FastLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetFastLimit(
                            state.fast_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in miliWatts.\nAccepted value range\nbetween 4000 & 65000.").size(12)
                            )
                            .style(container::bordered_box)
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // Slow Limit
        container(
            row![
                container(
                    container(text("Slow Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_slow_value as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_slow_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.slow_input
                        )
                            .style(text_input_style())
                            .on_input(Message::SlowLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetSlowLimit(
                            state.slow_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in miliWatts.\nAccepted value range\nbetween 4000 & 65000.\nSlow limit must be less\nthan or equal to fast limit.").size(12)
                            )
                            .style(card_style())
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // STAPM Limit
        container(
            row![
                container(
                    container(text("STAPM Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_stapm_value as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{:.1} W", state.curr_stapm_limit as f32 / 1000.0)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.stapm_input
                        )
                            .style(text_input_style())
                            .on_input(Message::StapmLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)  
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetStapmLimit(
                            state.stapm_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in miliWatts.\nAccepted value range\nbetween 4000 & 65000.\nStapm limit must be less\nthan or equal to slow limit.").size(12)
                            )
                            .style(card_style())
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // TCtl Limit
        container(
            row![
                container(
                    container(text("TCTL Limit").size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(4)),
                
                container(
                    container(text(format!("{}°C", state.curr_tctl_value)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
                
                container(
                    container(text(format!("{}°C", state.curr_tctl_limit)).size(16))
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        text_input(
                            "",
                            &state.tctl_input
                        )
                            .style(text_input_style())
                            .on_input(Message::TctlLimitInputChanged)
                            .align_x(Horizontal::Center)
                    )
                        .align_x(Horizontal::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),
        
                container(
                    container(
                        button("Set").on_press(Message::SetTctlLimit(
                            state.tctl_input.parse().unwrap_or(0)
                        ))
                    )
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center)
                        .width(Length::Fill)
                        .height(Length::Fill) 
                )
                .width(Length::FillPortion(2)),

                container(
                    container(
                        tooltip(
                            text("ⓘ").size(16).style(hint_text_style()).shaping(text::Shaping::Advanced),
                            container(
                                text("Enter value in °C.\nAccepted value range\nbetween 40 and 100.").size(12)
                            )
                            .style(card_style())
                            .padding(10),
                            tooltip::Position::Top
                        )
                    )
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
                )
                .width(Length::FillPortion(1))
            ]
            .spacing(20)
        )
        .style(card_style())
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fixed(50.0)),

        // Bottom full-width container
        container("Notes and settings")
            .align_x(alignment::Horizontal::Center)
            .style(card_style())
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fixed(150.0)),

    ]
    .spacing(10)
    .padding(20)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn update_state_values(_: &State) -> Subscription<Message> {
    time::every(Duration::from_secs(1))
        .map(|_| Message::UpdateStateValues)
}


fn main() -> iced::Result {
    iced::application("Ryzone", update, view)
        .window(window::Settings{
            min_size: Some(Size::new(600.0, 450.0)),
            size: Size::new(800.0, 600.0),
            icon: Some(
                window::icon::from_file_data(
                    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.ico")),
                    None,
                )
                .expect("icon file should be reachable and in ICO file format"),
            ),
            ..Default::default()
        })
        .theme(theme)
        .subscription(update_state_values)
        .run()
}

fn theme(_state: &State) -> Theme {
    Theme::TokyoNightStorm
}


