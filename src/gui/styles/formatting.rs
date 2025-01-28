pub fn format_frequency(freq: u32) -> String {
    if freq < 1000 {
        // Less than 1000 MHz (1 GHz)
        format!("{} MHz", freq)
    } else {
        format!("{:.2} GHz", freq as f32 / 1000.0)
    }
}

pub fn format_time_remaining(minutes: u32) -> String {
    if minutes == 0 {
        String::from("N/A")
    } else if minutes >= 60 {
        format!("{}h:{:02}m", minutes / 60, minutes % 60)
    } else {
        format!("{} min", minutes)
    }
}
