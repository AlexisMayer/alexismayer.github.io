//! UI Components module
//!
//! This module contains reusable UI components following clean architecture principles.
//! Components are organized by functionality and responsibility.

pub mod footer;
pub mod header;
pub mod sections;

// Re-export components for easier imports
pub use footer::Footer;
pub use header::Header;
pub use sections::*;
