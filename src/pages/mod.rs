//! Pages module - Application pages and views
//!
//! This module contains all the page components that represent the main views
//! of the SoftIA application. Each page is responsible for rendering a complete
//! view and orchestrating the necessary UI components and domain logic.

pub mod about;
pub mod case_studies;
pub mod contact;
pub mod home;
pub mod process;
pub mod services;

// Re-export all page components for convenience
pub use about::AboutPage;
pub use case_studies::CaseStudiesPage;
pub use contact::ContactPage;
pub use home::HomePage;
pub use process::ProcessPage;
pub use services::ServicesPage;
