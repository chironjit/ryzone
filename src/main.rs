use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, Entry, Grid, Label,
    Orientation, gdk,
};
use std::rc::Rc;
use std::cell::RefCell;

// Structure to hold our parameter entries for validation
struct ParameterEntries {
    fast_limit: Entry,
    slow_limit: Entry,
    stapm_limit: Entry,
    temp: Entry,
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("System Parameters Dashboard")
        .default_width(800)
        .default_height(600)
        .build();

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

    // Parameters Display Section
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

    // Create parameter display boxes
    for i in 0..15 {
        let param_box = Box::new(Orientation::Vertical, 5);
        let param_name = Label::new(Some(&format!("Parameter {}", i + 1)));
        let param_value = Label::new(Some("0.0"));
        
        param_box.append(&param_name);
        param_box.append(&param_value);
        
        param_box.set_widget_name("param-box");
        param_box.add_css_class("param-container");
        param_box.set_hexpand(true);
        param_box.set_vexpand(true);
        
        params_grid.attach(&param_box, (i % 3) as i32, (i / 3) as i32, 1, 1);
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

    // Set up entries
    entries.borrow().fast_limit.set_width_chars(10);
    entries.borrow().slow_limit.set_width_chars(10);
    entries.borrow().stapm_limit.set_width_chars(10);
    entries.borrow().temp.set_width_chars(10);

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
    let entries_for_fast = Rc::clone(&entries);
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

    let entries_for_temp = Rc::clone(&entries);
    entries.borrow().temp.connect_changed(move |entry| {
        if let Ok(value) = entry.text().parse::<f64>() {
            if value < 40.0 || value > 100.0 {
                entry.add_css_class("error");
            } else {
                entry.remove_css_class("error");
            }
        }
    });

    // Apply button
    let apply_button = Button::with_label("Apply Changes");
    apply_button.set_halign(gtk::Align::End);
    apply_button.add_css_class("suggested-action");
    
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
    window.show();
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.system.dashboard")
        .build();

    app.connect_activate(build_ui);
    app.run();
}