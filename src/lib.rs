//! SigmaOS AUR Helper
//!
//! AUR helper for Arch Linux package management with yay/parcli equivalent functionality

#![allow(warnings, unused)]

pub mod aur;
pub mod cli;
pub mod package;

pub use aur::*;
pub use cli::*;
pub use package::*;
