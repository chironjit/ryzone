// use std::{process::exit, time::Duration};
mod daemon;

use std::fs::File;

use daemon::daemon_loop;

fn test_dev_mem() {
    println!("\n=== Testing /dev/mem access ===");

    match File::open("/dev/mem") {
        Ok(_) => println!("✅ Can open /dev/mem"),
        Err(e) => println!("❌ Cannot open /dev/mem: {}", e),
    }
}

fn test_pci_access() {
    println!("\n=== Testing PCI access ===");

    // Test access to PCI config space
    let pci_paths = ["/sys/bus/pci/devices", "/proc/bus/pci"];

    for path in &pci_paths {
        match std::fs::read_dir(path) {
            Ok(_) => println!("✅ Can read {}", path),
            Err(e) => println!("❌ Cannot read {}: {}", path, e),
        }
    }
}

fn check_capabilities() {
    use std::fs;

    let status = fs::read_to_string("/proc/self/status").expect("Failed to read process status");

    for line in status.lines() {
        if line.starts_with("Cap") {
            println!("{}", line);
        }
    }

    test_dev_mem();
    test_pci_access();
}

fn main() {
    // Check priviledge levels. Must have access to /lib/mem.
    // Exit if priviledge not available
    check_capabilities();

    // Check settings.toml file in the /home/<user>/.ryzone folder
    // If no settings.toml file, then 

    // Run daemon loop
    daemon_loop(3);
}
