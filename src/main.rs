use iced::widget::{button, column, row, text, text_input, Column, Row}; 
use iced::Theme;


// Constants to limit input values
const FAST_LIMIT_MIN: u16 = 4000;
const FAST_LIMIT_MAX: u16 = 50000;
const SLOW_LIMIT_MIN: u16 = 4000;
const SLOW_LIMIT_MAX: u16 = 50000;
const STAPM_LIMIT_MIN: u16 = 4000;
const STAPM_LIMIT_MAX: u16 = 50000;
const TCTL_LIMIT_MIN: u16 = 40;
const TCTL_LIMIT_MAX: u16 = 100;

#[derive(Debug, Clone)]
pub enum Message {
    SetFastLimit(u16),
    SetSlowLimit(u16),
    SetStapmLimit(u16),
    SetTctlLimit(u16),
    FastLimitInputChanged(String),
    SlowLimitInputChanged(String),
    StapmLimitInputChanged(String),
    TctlLimitInputChanged(String),
}

// All the states managed by the app
#[derive(Default)]
struct Ryzone {
    fast_value: u16,
    slow_value: u16,
    stapm_value: u16,
    tctl_value: u16,
    fast_input: String,
    slow_input: String,
    stapm_input: String,
    tctl_input: String,
    fast_limit: u16,
    slow_limit: u16,
    stapm_limit: u16,
    tctl_limit: u16,
}


impl Ryzone {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetFastLimit(value) => {
                if value >= FAST_LIMIT_MIN && value <= FAST_LIMIT_MAX {
                    self.fast_limit = value;
                }
            }
            Message::SetSlowLimit(value) => {
                if value >= SLOW_LIMIT_MIN && value <= SLOW_LIMIT_MAX {
                    self.slow_limit = value;
                }
            }
            Message::SetStapmLimit(value) => {
                if value >= STAPM_LIMIT_MIN && value <= STAPM_LIMIT_MAX {
                    self.stapm_limit = value;
                }
            }
            Message::SetTctlLimit(value) => {
                if value >= TCTL_LIMIT_MIN && value <= TCTL_LIMIT_MAX {
                    self.tctl_limit = value;
                }
            }
            Message::FastLimitInputChanged(value) => {
                if value.chars().all(|c| c.is_digit(10)) {
                    if let Ok(num) = value.parse::<u16>() {
                        if num <= FAST_LIMIT_MAX {
                            self.fast_input = value;
                        }
                    }
                }
            }
            Message::SlowLimitInputChanged(value) => {
                if value.chars().all(|c| c.is_digit(10)) {
                    if let Ok(num) = value.parse::<u16>() {
                        if num <= SLOW_LIMIT_MAX {
                            self.slow_input = value;
                        }
                    }
                }
            }
            Message::StapmLimitInputChanged(value) => {
                if value.chars().all(|c| c.is_digit(10)) {
                    if let Ok(num) = value.parse::<u16>() {
                        if num <= STAPM_LIMIT_MAX {
                            self.stapm_input = value;
                        }
                    }
                }
            }
            Message::TctlLimitInputChanged(value) => {
                if value.chars().all(|c| c.is_digit(10)) {
                    if let Ok(num) = value.parse::<u16>() {
                        if num <= TCTL_LIMIT_MAX {
                            self.tctl_input = value;
                        }
                    }
                }
            }
        }
    }

    // Separate getter methods
    pub fn get_fast_limit(&self) -> u16 {
        self.fast_limit
    }

    pub fn get_slow_limit(&self) -> u16 {
        self.slow_limit
    }

    pub fn get_stapm_limit(&self) -> u16 {
        self.stapm_limit
    }

    pub fn get_tctl_limit(&self) -> u16 {
        self.tctl_limit
    }

    pub fn view(&self) -> Row<Message> {
        row![ 
            // Set Fast Limit
            column![
                text(format!("Current Fast Limit: {} mW", self.fast_limit)).size(30),
                text(format!("Range: {}-{}", FAST_LIMIT_MIN, FAST_LIMIT_MAX)).size(20),
                text_input(
                    "Enter new value...",
                    &self.fast_input
                )
                .on_input(Message::FastLimitInputChanged)
                .padding(10),
                button("Update").on_press(Message::SetFastLimit(
                    self.fast_input.parse().unwrap_or(0)
                )),
            ],

            // Set Slow Limit
            column![
                text(format!("Current Slow Limit: {} mW", self.slow_limit)).size(30),
                text(format!("Range: {}-{}", SLOW_LIMIT_MIN, SLOW_LIMIT_MAX)).size(20),
                text_input(
                    "Enter new value...",
                    &self.slow_input
                )
                .on_input(Message::SlowLimitInputChanged)
                .padding(10),
                button("Update").on_press(Message::SetSlowLimit(
                    self.slow_input.parse().unwrap_or(0)
                )),
            ],

            // Set STAPM Limit
            column![
                text(format!("Current STAPM Limit: {} mW", self.stapm_limit)).size(30),
                text(format!("Range: {}-{}", STAPM_LIMIT_MIN, STAPM_LIMIT_MAX)).size(20),
                text_input(
                    "Enter new value...",
                    &self.stapm_input
                )
                .on_input(Message::StapmLimitInputChanged)
                .padding(10),
                button("Update").on_press(Message::SetStapmLimit(
                    self.stapm_input.parse().unwrap_or(0)
                )),
            ],

            // Set TCTL Limit
            column![
                text(format!("Current TCTL Limit: {}°C", self.tctl_limit)).size(30),
                text(format!("Range: {}-{}", TCTL_LIMIT_MIN, TCTL_LIMIT_MAX)).size(20),
                text_input(
                    "Enter new value...",
                    &self.tctl_input
                )
                .on_input(Message::TctlLimitInputChanged)
                .padding(10),
                button("Update").on_press(Message::SetTctlLimit(
                    self.tctl_input.parse().unwrap_or(0)
                )),
            ],
        ]
        .spacing(20)  
        .padding(20)  
    }

    
    
}


fn main() -> iced::Result {
    iced::application("Ryzone", Ryzone::update, Ryzone::view)
        .theme(theme)
        .run()
}

fn theme(state: &Ryzone) -> Theme {
    Theme::TokyoNightStorm
}


