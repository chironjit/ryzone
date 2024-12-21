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
#[derive(Default, Debug, Clone)]
struct RyzoneState {
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

// Standalone update function
fn update(
    state: &mut RyzoneState,
    message: Message
) {
    match message {
        Message::SetFastLimit(value) => {
            if value >= FAST_LIMIT_MIN && value <= FAST_LIMIT_MAX {
                state.fast_limit = value;
            }
        }
        Message::SetSlowLimit(value) => {
            if value >= SLOW_LIMIT_MIN && value <= SLOW_LIMIT_MAX {
                state.slow_limit = value;
            }
        }
        Message::SetStapmLimit(value) => {
            if value >= STAPM_LIMIT_MIN && value <= STAPM_LIMIT_MAX {
                state.stapm_limit = value;
            }
        }
        Message::SetTctlLimit(value) => {
            if value >= TCTL_LIMIT_MIN && value <= TCTL_LIMIT_MAX {
                state.tctl_limit = value;
            }
        }
        Message::FastLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u16>() {
                    if num <= FAST_LIMIT_MAX {
                        state.fast_input = value;
                    }
                }
            }
        }
        Message::SlowLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u16>() {
                    if num <= SLOW_LIMIT_MAX {
                        state.slow_input = value;
                    }
                }
            }
        }
        Message::StapmLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u16>() {
                    if num <= STAPM_LIMIT_MAX {
                        state.stapm_input = value;
                    }
                }
            }
        }
        Message::TctlLimitInputChanged(value) => {
            if value.chars().all(|c| c.is_digit(10)) {
                if let Ok(num) = value.parse::<u16>() {
                    if num <= TCTL_LIMIT_MAX {
                        state.tctl_input = value;
                    }
                }
            }
        }
    }
}

// Standalone view function
fn view(state: &RyzoneState) -> Row<Message> {
    row![ 
        // Set Fast Limit
        column![
            text(format!("Current Fast Limit: {} mW", state.fast_limit)).size(30),
            text(format!("Range: {}-{}", FAST_LIMIT_MIN, FAST_LIMIT_MAX)).size(20),
            text_input(
                "Enter new value...",
                &state.fast_input
            )
            .on_input(Message::FastLimitInputChanged)
            .padding(10),
            button("Update").on_press(Message::SetFastLimit(
                state.fast_input.parse().unwrap_or(0)
            )),
        ],

        // Set Slow Limit
        column![
            text(format!("Current Slow Limit: {} mW", state.slow_limit)).size(30),
            text(format!("Range: {}-{}", SLOW_LIMIT_MIN, SLOW_LIMIT_MAX)).size(20),
            text_input(
                "Enter new value...",
                &state.slow_input
            )
            .on_input(Message::SlowLimitInputChanged)
            .padding(10),
            button("Update").on_press(Message::SetSlowLimit(
                state.slow_input.parse().unwrap_or(0)
            )),
        ],

        // Set STAPM Limit
        column![
            text(format!("Current STAPM Limit: {} mW", state.stapm_limit)).size(30),
            text(format!("Range: {}-{}", STAPM_LIMIT_MIN, STAPM_LIMIT_MAX)).size(20),
            text_input(
                "Enter new value...",
                &state.stapm_input
            )
            .on_input(Message::StapmLimitInputChanged)
            .padding(10),
            button("Update").on_press(Message::SetStapmLimit(
                state.stapm_input.parse().unwrap_or(0)
            )),
        ],

        // Set TCTL Limit
        column![
            text(format!("Current TCTL Limit: {}°C", state.tctl_limit)).size(30),
            text(format!("Range: {}-{}", TCTL_LIMIT_MIN, TCTL_LIMIT_MAX)).size(20),
            text_input(
                "Enter new value...",
                &state.tctl_input
            )
            .on_input(Message::TctlLimitInputChanged)
            .padding(10),
            button("Update").on_press(Message::SetTctlLimit(
                state.tctl_input.parse().unwrap_or(0)
            )),
        ],
    ]
    .spacing(20)  
    .padding(20)  
}


fn main() -> iced::Result {
    iced::application("Ryzone", update, view)
        .theme(theme)
        .run()
}

fn theme(_state: &RyzoneState) -> Theme {
    Theme::TokyoNightStorm
}


