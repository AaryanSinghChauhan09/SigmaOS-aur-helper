//! Command-line interface
//!
//! CLI for the AUR helper using clap

use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;

use crate::aur::AurClient;
use crate::package::PackageManager;

/// SigmaOS AUR Helper - AUR package management for Arch Linux
#[derive(Parser)]
#[command(name = "sigmaos-aur")]
#[command(about = "AUR helper for Arch Linux package management", long_about = None)]
struct Cli {
    /// Build directory for AUR packages
    #[arg(short, long, default_value = "/tmp/sigmaos-aur")]
    build_dir: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for packages in AUR
    Search {
        /// Search query
        query: String,
    },
    /// Get package information
    Info {
        /// Package name
        name: String,
    },
    /// Install package from AUR
    Install {
        /// Package name
        name: String,
    },
    /// Remove package
    Remove {
        /// Package name
        name: String,
    },
    /// Update AUR packages
    Update,
    /// Clone package without building
    Clone {
        /// Package name
        name: String,
    },
}

/// Run the CLI
pub async fn run() -> Result<()> {
    let cli = Cli::parse();

    let aur_client = AurClient::new();
    let pkg_manager = PackageManager::new(&cli.build_dir);

    match cli.command {
        Commands::Search { query } => {
            cmd_search(&aur_client, &query).await?;
        }
        Commands::Info { name } => {
            cmd_info(&aur_client, &name).await?;
        }
        Commands::Install { name } => {
            cmd_install(&aur_client, &pkg_manager, &name).await?;
        }
        Commands::Remove { name } => {
            cmd_remove(&pkg_manager, &name)?;
        }
        Commands::Update => {
            cmd_update(&aur_client, &pkg_manager).await?;
        }
        Commands::Clone { name } => {
            cmd_clone(&pkg_manager, &name)?;
        }
    }

    Ok(())
}

/// Search command
async fn cmd_search(client: &AurClient, query: &str) -> Result<()> {
    println!("{} Searching for '{}'...", "[INFO]".blue(), query);

    let results = client.search(query).await?;

    if results.is_empty() {
        println!("{} No results found", "[WARN]".yellow());
        return Ok(());
    }

    println!("\n{} Results:", "[SUCCESS]".green());
    for pkg in results {
        println!("  {} {} - {}", 
            pkg.name.cyan(),
            pkg.version.yellow(),
            pkg.description
        );
    }

    Ok(())
}

/// Info command
async fn cmd_info(client: &AurClient, name: &str) -> Result<()> {
    println!("{} Getting info for '{}'...", "[INFO]".blue(), name);

    if let Some(pkg) = client.info(name).await? {
        println!("\n{} {}", "[SUCCESS]".green(), pkg.name.cyan());
        println!("  Version: {}", pkg.version.yellow());
        println!("  Description: {}", pkg.description);
        println!("  Maintainer: {}", pkg.maintainer.as_deref().unwrap_or("Orphan"));
        println!("  Popularity: {:.2}", pkg.popularity);
        
        if let Some(url) = &pkg.url {
            println!("  URL: {}", url);
        }
        
        if let Some(deps) = &pkg.depends {
            println!("  Depends: {}", deps.join(", "));
        }
        
        if let Some(makedeps) = &pkg.makedepends {
            println!("  Make Depends: {}", makedeps.join(", "));
        }
    } else {
        println!("{} Package not found", "[ERROR]".red());
    }

    Ok(())
}

/// Install command
async fn cmd_install(client: &AurClient, pkg_manager: &PackageManager, name: &str) -> Result<()> {
    println!("{} Installing '{}' from AUR...", "[INFO]".blue(), name);

    // Check if package exists in AUR
    if client.info(name).await?.is_none() {
        println!("{} Package not found in AUR", "[ERROR]".red());
        return Ok(());
    }

    // Clone package
    pkg_manager.clone_package(name)?;

    // Build package
    pkg_manager.build_package(name)?;

    Ok(())
}

/// Remove command
fn cmd_remove(pkg_manager: &PackageManager, name: &str) -> Result<()> {
    pkg_manager.remove_package(name)?;
    Ok(())
}

/// Update command
async fn cmd_update(client: &AurClient, pkg_manager: &PackageManager) -> Result<()> {
    println!("{} Checking for AUR updates...", "[INFO]".blue());
    
    // This would check installed AUR packages and update them
    // For now, just a placeholder
    println!("{} Update functionality coming soon", "[WARN]".yellow());
    
    Ok(())
}

/// Clone command
fn cmd_clone(pkg_manager: &PackageManager, name: &str) -> Result<()> {
    pkg_manager.clone_package(name)?;
    println!("{} Package cloned to build directory", "[SUCCESS]".green());
    Ok(())
}
