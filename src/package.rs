//! Package management operations
//!
//! Handles package installation, building, and dependency resolution

use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use std::process::Command;

/// Package manager for AUR packages
pub struct PackageManager {
    build_dir: String,
}

impl PackageManager {
    /// Create a new package manager
    pub fn new(build_dir: &str) -> Self {
        Self {
            build_dir: build_dir.to_string(),
        }
    }

    /// Clone package from AUR
    pub fn clone_package(&self, name: &str) -> Result<()> {
        let url = format!("https://aur.archlinux.org/{}.git", name);
        let path = format!("{}/{}", self.build_dir, name);

        println!("{} Cloning {} from AUR...", "[INFO]".blue(), name);

        let status = Command::new("git")
            .args(["clone", &url, &path])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to clone package: {}", name);
        }

        Ok(())
    }

    /// Build package from PKGBUILD
    pub fn build_package(&self, name: &str) -> Result<()> {
        let path = format!("{}/{}", self.build_dir, name);

        println!("{} Building {}...", "[INFO]".blue(), name);

        let status = Command::new("makepkg")
            .args(["-si", "--noconfirm"])
            .current_dir(&path)
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to build package: {}", name);
        }

        println!("{} Successfully built {}", "[SUCCESS]".green(), name);
        Ok(())
    }

    /// Install package
    pub fn install_package(&self, name: &str) -> Result<()> {
        println!("{} Installing {}...", "[INFO]".blue(), name);

        let status = Command::new("pacman")
            .args(["-S", "--noconfirm", name])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to install package: {}", name);
        }

        println!("{} Successfully installed {}", "[SUCCESS]".green(), name);
        Ok(())
    }

    /// Remove package
    pub fn remove_package(&self, name: &str) -> Result<()> {
        println!("{} Removing {}...", "[INFO]".blue(), name);

        let status = Command::new("pacman")
            .args(["-Rns", "--noconfirm", name])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to remove package: {}", name);
        }

        println!("{} Successfully removed {}", "[SUCCESS]".green(), name);
        Ok(())
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        Command::new("pacman")
            .args(["-Q", name])
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

impl Default for PackageManager {
    fn default() -> Self {
        Self::new("/tmp/sigmaos-aur")
    }
}
