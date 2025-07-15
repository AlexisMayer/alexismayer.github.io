//! Domain layer - Core business logic and entities
//!
//! This module contains the business domain models and logic for CraftData.
//! It follows Domain-Driven Design principles with clear separation of concerns.

pub mod case_studies;
pub mod company;
pub mod services;

// Re-export main domain types for convenience
pub use case_studies::*;
pub use company::*;
pub use services::*;
