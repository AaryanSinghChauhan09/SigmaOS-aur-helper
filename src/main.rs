//! SigmaOS AUR Helper - Main entry point
//!
//! AUR helper for Arch Linux package management

use sigmaos_aur_helper::cli;

#[tokio::main]
async fn main() {
    if let Err(e) = cli::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
