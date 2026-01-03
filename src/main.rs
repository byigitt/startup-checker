use startup_checker::permissions::{admin_warning, is_elevated};
use startup_checker::sources::scan_all_sources;
use startup_checker::ui::run_app;

fn main() {
    // Check for admin privileges
    if !is_elevated() {
        if let Some(warning) = admin_warning() {
            eprintln!("Warning: {}", warning);
            eprintln!("Some startup items may not be modifiable.");
            eprintln!();
        }
    }

    // Scan all startup sources
    eprintln!("Scanning startup items...");
    let items = scan_all_sources();
    eprintln!("Found {} startup items.", items.len());

    // Run the TUI
    if let Err(e) = run_app(items) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
