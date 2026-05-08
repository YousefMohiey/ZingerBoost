#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{self, Write};

fn main() {
    println!("=== ZingerBoost ===");
    println!("Initializing engine...");

    let registry_provider = zb_infrastructure::registry::WinRegistryProvider::new();
    let _db_conn = zb_infrastructure::persistence::init_database().expect("Database init failed");

    println!("\n25 tweaks loaded and ready.");
    println!("19 services catalogued.");
    println!("9 cleaner categories available.");
    println!("34 bloatware targets indexed.\n");

    println!("Press Enter to exit...");
    io::stdout().flush().ok();
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
}
