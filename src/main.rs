use iced::widget::{button, column, row, text, text_input, Row}; 
use iced::{Theme, Subscription};
use iced::time::{self, Duration};
use libryzenadj::RyzenAdj;


// Constants to limit input values
const FAST_LIMIT_MIN: u32 = 4000;
const FAST_LIMIT_MAX: u32 = 50000;
const SLOW_LIMIT_MIN: u32 = 4000;
const SLOW_LIMIT_MAX: u32 = 50000;
const STAPM_LIMIT_MIN: u32 = 4000;
const STAPM_LIMIT_MAX: u32 = 50000;
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
struct RyzoneState {
    curr_fast_value: u32,
    curr_slow_value: u32,
    curr_stapm_value: u32,
    curr_tctl_value: u32,
    curr_fast_limit: u32,
    curr_slow_limit: u32,
    curr_stapm_limit: u32,
    curr_tctl_limit: u32,
    fast_input: String,
    slow_input: String,
    stapm_input: String,
    tctl_input: String,
    manual_fast_limit: u32,
    manual_slow_limit: u32,
    manual_stapm_limit: u32,
    manual_tctl_limit: u32,
}

// Standalone update function
fn update(
    state: &mut RyzoneState,
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

// Standalone view function
fn view(state: &RyzoneState) -> Row<Message> {
    row![ 
        // Set Fast Limit
        column![
            text(format!("Current Fast Value: {} mW", state.curr_fast_value)).size(30),
            text(format!("Current Fast Limit: {} mW", state.curr_fast_limit)).size(30),
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
            text(format!("Current Slow Value: {} mW", state.curr_slow_value)).size(30),
            text(format!("Current Slow Limit: {} mW", state.curr_slow_limit)).size(30),
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
            text(format!("Current STAPM Value: {} mW", state.curr_stapm_value)).size(30),
            text(format!("Current STAPM Limit: {} mW", state.curr_stapm_limit)).size(30),
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
            text(format!("Current TCTL Value: {}°C", state.curr_tctl_value)).size(30),
            text(format!("Current TCTL Limit: {}°C", state.curr_tctl_limit)).size(30),
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

fn update_state_values(_: &RyzoneState) -> Subscription<Message> {
    time::every(Duration::from_secs(1))
        .map(|_| Message::UpdateStateValues)
}


fn main() -> iced::Result {
    iced::application("Ryzone", update, view)
        .theme(theme)
        .subscription(update_state_values)
        .run()
}

fn theme(_state: &RyzoneState) -> Theme {
    Theme::TokyoNightStorm
}


