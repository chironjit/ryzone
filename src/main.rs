use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Grid, Label,
    Orientation, gdk, glib,
};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use libryzenadj::{RyzenAdj, RyzenAdjResult};

// Structure to hold system state
#[derive(Debug, Clone)]
struct SystemState {
    tctl_limit: f32,
    tctl_value: f32,
    fast_limit: f32,
    fast_value: f32,
    slow_limit: f32,
    slow_value: f32,
    stapm_limit: f32,
    stapm_value: f32,
}

impl Default for SystemState {
    fn default() -> Self {
        Self {
            tctl_limit: 0.0,
            tctl_value: 0.0,
            fast_limit: 0.0,
            fast_value: 0.0,
            slow_limit: 0.0,
            slow_value: 0.0,
            stapm_limit: 0.0,
            stapm_value: 0.0,
        }
    }
}

// Structure to hold our parameter entries for validation
struct ParameterEntries {
    fast_limit: Entry,
    slow_limit: Entry,
    stapm_limit: Entry,
    temp: Entry,
}

// Structure to hold UI elements that need updating
struct SystemMonitor {
    state: Arc<Mutex<SystemState>>,
    adj: RyzenAdj,
}

impl SystemMonitor {
    fn new() -> RyzenAdjResult<Self> {
        Ok(Self {
            state: Arc::new(Mutex::new(SystemState::default())),
            adj: RyzenAdj::new()?,
        })
    }

    fn update_state(&self) -> RyzenAdjResult<()> {
        let mut state = self.state.lock().unwrap();
        
        // Update temperature values
        state.tctl_limit = self.adj.get_tctl_temp()?;
        state.tctl_value = self.adj.get_tctl_temp_value()?;
        
        // Update power limits
        state.fast_limit = self.adj.get_fast_limit()? ;
        state.fast_value = self.adj.get_fast_value()? ;
        state.slow_limit = self.adj.get_slow_limit()? ;
        state.slow_value = self.adj.get_slow_value()? ;
        state.stapm_limit = self.adj.get_stapm_limit()?;
        state.stapm_value = self.adj.get_stapm_value()?;

        Ok(())
    }

    fn get_state(&self) -> SystemState {
        self.state.lock().unwrap().clone()
    }

    fn set_fast_limit(&self, value: u32) -> RyzenAdjResult<()> {
        self.adj.set_fast_limit(value * 1000)
    }

    fn set_slow_limit(&self, value: u32) -> RyzenAdjResult<()> {
        self.adj.set_slow_limit(value * 1000)
    }

    fn set_stapm_limit(&self, value: u32) -> RyzenAdjResult<()> {
        self.adj.set_stapm_limit(value * 1000)
    }

    fn set_tctl_temp(&self, value: u32) -> RyzenAdjResult<()> {
        self.adj.set_tctl_temp(value)
    }
}

fn show_error_dialog(parent: &impl IsA<gtk::Window>, message: &str) {
    let dialog = gtk::MessageDialog::new(
        Some(parent),
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Error,
        gtk::ButtonsType::Ok,
        message
    );
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}

fn show_success_dialog(parent: &impl IsA<gtk::Window>, message: &str) {
    let dialog = gtk::MessageDialog::new(
        Some(parent),
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Info,
        gtk::ButtonsType::Ok,
        message
    );
    dialog.connect_response(|dialog, _| dialog.close());
    dialog.show();
}

// Function to create param box
fn create_param_box(name: &str, value: &str) -> (Box, Label) {
    let param_box = Box::new(Orientation::Vertical, 5);
    let param_name = Label::new(Some(name));
    let param_value = Label::new(Some(value));
    
    param_box.append(&param_name);
    param_box.append(&param_value);
    
    param_box.set_widget_name("param-box");
    param_box.add_css_class("param-container");
    param_box.set_hexpand(true);
    param_box.set_vexpand(true);
    
    (param_box, param_value)
}

fn setup_monitor_updates(monitor: Arc<SystemMonitor>, value_labels: Vec<(String, Label)>) {
    glib::timeout_add_local(std::time::Duration::from_millis(1000), move || {
        if let Ok(()) = monitor.update_state() {
            let state = monitor.get_state();
            
            for (id, label) in value_labels.iter() {
                let value = match id.as_str() {
                    "tctl_limit" => format!("{:.1}°C", state.tctl_limit),
                    "tctl_value" => format!("{:.1}°C", state.tctl_value),
                    "fast_limit" => format!("{:.1}W", state.fast_limit),
                    "fast_value" => format!("{:.1}W", state.fast_value),
                    "slow_limit" => format!("{:.1}W", state.slow_limit),
                    "slow_value" => format!("{:.1}W", state.slow_value),
                    "stapm_limit" => format!("{:.1}W", state.stapm_limit),
                    "stapm_value" => format!("{:.1}W", state.stapm_value),
                    _ => String::from("N/A"),
                };
                label.set_text(&value);
            }
        }
        
        glib::ControlFlow::Continue
    });
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("System Parameters Dashboard")
        .default_width(800)
        .default_height(600)
        .build();

    // Initialize the monitor:
    let monitor = match SystemMonitor::new() {
        Ok(monitor) => Arc::new(monitor),
        Err(e) => {
            eprintln!("Failed to initialize system monitor: {}", e);
            return;
        }
    };

    let mut value_labels = Vec::new();

    // Main container
    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);
    main_box.set_vexpand(true);

    // Title
    let title = Label::new(Some("System Parameters Dashboard"));
    title.set_markup("<span size='x-large'><b>System Parameters Dashboard</b></span>");
    main_box.append(&title);

    // Parameters display section wit:
    let params_frame = gtk::Frame::new(Some("Current Parameters"));
    params_frame.set_vexpand(true);

    let params_grid = Grid::new();
    params_grid.set_row_spacing(10);
    params_grid.set_column_spacing(20);
    params_grid.set_vexpand(true);
    params_grid.set_margin_top(10);
    params_grid.set_margin_bottom(10);
    params_grid.set_margin_start(10);
    params_grid.set_margin_end(10);
    
    // Row 1: Temperature
    let temp_label = Label::new(Some("Temperature"));
    temp_label.set_markup("<b>Temperature</b>");
    params_grid.attach(&temp_label, 0, 0, 1, 1);

    let temp_grid = Grid::new();
    temp_grid.set_column_spacing(10);

    // Create temperature boxes with stored labels
    let (tctl_limit_box, tctl_limit_label) = create_param_box("TCtl Limit", "0.0");
    let (tctl_value_box, tctl_value_label) = create_param_box("TCtl Value", "0.0");
    let (gpu_temp1_box, _gpu_temp1_label) = create_param_box("GPU Temp", "0.0");
    let (gpu_temp2_box, _gpu_temp2_label) = create_param_box("GPU Temp", "0.0");

    // Store labels for updating
    value_labels.push(("tctl_limit".to_string(), tctl_limit_label));
    value_labels.push(("tctl_value".to_string(), tctl_value_label));

    // Attach boxes to grid
    temp_grid.attach(&tctl_limit_box, 0, 0, 1, 1);
    temp_grid.attach(&tctl_value_box, 1, 0, 1, 1);
    temp_grid.attach(&gpu_temp1_box, 2, 0, 1, 1);
    temp_grid.attach(&gpu_temp2_box, 3, 0, 1, 1);

    params_grid.attach(&temp_grid, 1, 0, 1, 1);

    // Row 2-4: Limits
    let limits = [
        ("Fast Limit", ["Limit", "Current Value"]),
        ("Slow Limit", ["Limit", "Current Value"]),
        ("STAPM Limit", ["Limit", "Current Value"]),
    ];

    for (row, (title, params)) in limits.iter().enumerate() {
        let label = Label::new(Some(title));
        label.set_markup(&format!("<b>{}</b>", title));
        params_grid.attach(&label, 0, (row + 1) as i32, 1, 1);

        let limit_grid = Grid::new();
        limit_grid.set_column_spacing(10);

        // Create parameter boxes with stored labels
        let (limit_box, limit_label) = create_param_box(params[0], "0.0");
        let (value_box, value_label) = create_param_box(params[1], "0.0");

        // Store labels for updating based on the row
        match row {
            0 => {
                value_labels.push(("fast_limit".to_string(), limit_label));
                value_labels.push(("fast_value".to_string(), value_label));
            },
            1 => {
                value_labels.push(("slow_limit".to_string(), limit_label));
                value_labels.push(("slow_value".to_string(), value_label));
            },
            2 => {
                value_labels.push(("stapm_limit".to_string(), limit_label));
                value_labels.push(("stapm_value".to_string(), value_label));
            },
            _ => {}
        }

        limit_grid.attach(&limit_box, 0, 0, 1, 1);
        limit_grid.attach(&value_box, 1, 0, 1, 1);
        params_grid.attach(&limit_grid, 1, (row + 1) as i32, 1, 1);
    }

    params_frame.set_child(Some(&params_grid));
    main_box.append(&params_frame);


    // Controls Section
    let controls_frame = gtk::Frame::new(Some("Parameter Controls"));
    let controls_box = Box::new(Orientation::Vertical, 10);
    
    controls_box.set_margin_top(10);
    controls_box.set_margin_bottom(10);
    controls_box.set_margin_start(10);
    controls_box.set_margin_end(10);

    // Create control inputs
    let controls_grid = Grid::new();
    controls_grid.set_row_spacing(10);
    controls_grid.set_column_spacing(20);

    // Define the parameters we want to control
    let parameters = [
        "Fast Limit",
        "Slow limit",
        "STAPM Limit",
        "Processor Temp"
    ];

    // Create entries struct to store our entries for validation
    let entries = Rc::new(RefCell::new(ParameterEntries {
        fast_limit: Entry::new(),
        slow_limit: Entry::new(),
        stapm_limit: Entry::new(),
        temp: Entry::new(),
    }));

    // Set up initial values
    let monitor_for_init = Arc::clone(&monitor);
    if let Ok(()) = monitor_for_init.update_state() {
        let state = monitor_for_init.get_state();
        entries.borrow().fast_limit.set_text(&format!("{:.1}", state.fast_limit));
        entries.borrow().slow_limit.set_text(&format!("{:.1}", state.slow_limit));
        entries.borrow().stapm_limit.set_text(&format!("{:.1}", state.stapm_limit));
        entries.borrow().temp.set_text(&format!("{:.1}", state.tctl_limit));
    }

    // Set up entries to only accept numbers
    for entry in [&entries.borrow().fast_limit, &entries.borrow().slow_limit, 
    &entries.borrow().stapm_limit, &entries.borrow().temp] {
    entry.set_width_chars(10);

    // Clear on focus
    let entry_clone = entry.clone();
    entry.connect_activate(move |_| {
    entry_clone.set_text("");
    });

    // Only allow numbers and decimal point
    entry.connect_changed(move |entry| {
    let text = entry.text();
    let is_valid = text.chars().all(|c| c.is_digit(10) || c == '.')
    && text.matches('.').count() <= 1;

    if !is_valid && !text.is_empty() {
    // Remove the last character if it made the input invalid
    let new_text = text.chars().take(text.len() - 1).collect::<String>();
    entry.set_text(&new_text);
    }
    });
    }

    // Create the input fields
    for (i, param_name) in parameters.iter().enumerate() {
        let control_box = Box::new(Orientation::Horizontal, 5);
        let label = Label::new(Some(param_name));
        
        let entry = match i {
            0 => &entries.borrow().fast_limit,
            1 => &entries.borrow().slow_limit,
            2 => &entries.borrow().stapm_limit,
            3 => &entries.borrow().temp,
            _ => unreachable!(),
        };
        
        control_box.append(&label);
        control_box.append(entry);
        
        controls_grid.attach(&control_box, i as i32, 0, 1, 1);
    }

    controls_box.append(&controls_grid);

    

    // Add parameter descriptions
    let descriptions_box = Box::new(Orientation::Vertical, 10);
    descriptions_box.set_margin_top(20);
    descriptions_box.add_css_class("descriptions");

    let descriptions = [
        "<b>Fast Limit:</b> Power limit for short-term boost (valid range: 4W - 50W)\n",
        "<b>Slow Limit:</b> Sustained power limit (must be ≤ Fast Limit)\n",
        "<b>STAPM Limit:</b> Average power limit (must be ≤ Slow Limit)\n",
        "<b>Processor Temp:</b> Maximum processor temperature (valid range: 40°C - 100°C)",
    ];

    for desc in descriptions.iter() {
        let desc_label = Label::new(None);
        desc_label.set_markup(desc);
        desc_label.set_halign(gtk::Align::Start);
        desc_label.set_wrap(true);
        descriptions_box.append(&desc_label);
    }

    controls_box.append(&descriptions_box);

    // Set up validation
    entries.borrow().fast_limit.connect_changed(move |entry| {
        if let Ok(value) = entry.text().parse::<f64>() {
            if value < 4.0 || value > 50.0 {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }
    });

    let entries_for_slow = Rc::clone(&entries);
    entries.borrow().slow_limit.connect_changed(move |entry| {
        let fast_limit: f64 = entries_for_slow.borrow().fast_limit.text()
            .parse().unwrap_or(0.0);
        if let Ok(value) = entry.text().parse::<f64>() {
            if value > fast_limit {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }
    });

    let entries_for_stapm = Rc::clone(&entries);
    entries.borrow().stapm_limit.connect_changed(move |entry| {
        let slow_limit: f64 = entries_for_stapm.borrow().slow_limit.text()
            .parse().unwrap_or(0.0);
        if let Ok(value) = entry.text().parse::<f64>() {
            if value > slow_limit {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }
    });

    entries.borrow().temp.connect_changed(move |entry| {
        if let Ok(value) = entry.text().parse::<f64>() {
            if value < 40.0 || value > 100.0 {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }
    });

    // Apply UI section

    let apply_button = Button::with_label("Apply Changes");
    apply_button.set_halign(gtk::Align::End);
    apply_button.add_css_class("suggested-action");

    let window_clone = window.clone();
    let entries_for_apply = Rc::clone(&entries);
    let monitor_for_apply = Arc::clone(&monitor);

    apply_button.connect_clicked(move |_| {
        let entries = entries_for_apply.borrow();
        
        // Helper function to parse and validate value
        let parse_value = |text: String, min: f64, max: f64| -> Result<u32, String> {
            text.parse::<f64>()
                .map_err(|_| "Invalid number format".to_string())
                .and_then(|v| {
                    if v >= min && v <= max {
                        Ok(v as u32)
                    } else {
                        Err(format!("Value must be between {} and {}", min, max))
                    }
                })
        };

       // Process each entry
        type SetterFn = std::boxed::Box<dyn Fn(u32) -> RyzenAdjResult<()>>;

        // Clone monitor outside the array definition for each closure
        let monitor_for_fast = Arc::clone(&monitor_for_apply);
        let monitor_for_slow = Arc::clone(&monitor_for_apply);
        let monitor_for_stapm = Arc::clone(&monitor_for_apply);
        let monitor_for_temp = Arc::clone(&monitor_for_apply);

        let results: [(String, SetterFn, &str, f64, f64); 4] = [
            (entries.fast_limit.text().to_string(), 
            std::boxed::Box::new(move |v| monitor_for_fast.set_fast_limit(v)), 
            "Fast Limit", 4.0, 50.0),
            (entries.slow_limit.text().to_string(),
            std::boxed::Box::new(move |v| monitor_for_slow.set_slow_limit(v)), 
            "Slow Limit", 4.0, 50.0),
            (entries.stapm_limit.text().to_string(),
            std::boxed::Box::new(move |v| monitor_for_stapm.set_stapm_limit(v)), 
            "STAPM Limit", 4.0, 50.0),
            (entries.temp.text().to_string(),
            std::boxed::Box::new(move |v| monitor_for_temp.set_tctl_temp(v)), 
            "Temperature", 50.0, 100.0),
        ];

        for (value, setter, name, min, max) in results {
            if !value.is_empty() {
                match parse_value(value, min, max) {
                    Ok(parsed_value) => {
                        // Add interdependent validations
                        if name == "Slow Limit" {
                            let fast_limit = entries.fast_limit.text()
                                .parse::<f64>()
                                .unwrap_or(50.0); // default to max if invalid
                            if parsed_value as f64 > fast_limit {
                                show_error_dialog(&window_clone,
                                    "Slow Limit cannot be greater than Fast Limit");
                                continue;
                            }
                        } else if name == "STAPM Limit" {
                            let slow_limit = entries.slow_limit.text()
                                .parse::<f64>()
                                .unwrap_or(50.0);
                            if parsed_value as f64 > slow_limit {
                                show_error_dialog(&window_clone,
                                    "STAPM Limit cannot be greater than Slow Limit");
                                continue;
                            }
                        }
                
                        if let Err(e) = setter(parsed_value) {
                            show_error_dialog(&window_clone, 
                                &format!("Failed to set {}: {}", name, e));
                        } else {
                            show_success_dialog(&window_clone,
                                &format!("{} successfully set to {}", name, parsed_value));
                        }
                    },
                    Err(e) => {
                        show_error_dialog(&window_clone, 
                            &format!("Invalid {} value: {}", name, e));
                    }
                }
            }
        }
    });

    let button_box = Box::new(Orientation::Horizontal, 0);
    button_box.set_halign(gtk::Align::End);
    button_box.append(&apply_button);
    controls_box.append(&button_box);

    controls_frame.set_child(Some(&controls_box));
    main_box.append(&controls_frame);

    // Add CSS styling
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        .param-container {
            background-color: #f8f8f8;
            padding: 10px;
            border-radius: 4px;
            border: 1px solid #ddd;
        }
        .descriptions {
            background-color: #f5f5f5;
            padding: 15px;
            border-radius: 4px;
        }
        .error entry {
            background-color: #ffe6e6;
            border-color: #ff0000;
        }
        "
    );

    // Apply CSS to the window
    let display = gdk::Display::default().expect("Could not get default display");
    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    window.set_child(Some(&main_box));
    setup_monitor_updates(Arc::clone(&monitor), value_labels);
    window.show();

}

fn main() {
    let app = Application::builder()
        .application_id("com.example.system.dashboard")
        .build();

    app.connect_activate(build_ui);
    app.run();
}