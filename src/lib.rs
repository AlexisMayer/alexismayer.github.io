//! SoftIA - Data & IA pour les PME
//!
//! This is the main library file implementing a light Domain-Driven Design architecture.
//! The application is organized into clear layers with separated concerns.

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Domain layer imports
pub mod domain;

// UI layer imports
pub mod ui;

// Pages layer imports
pub mod pages;
use pages::*;

/// Main application component
///
/// This is the root component that sets up routing and provides the overall application structure.
/// It follows clean architecture principles by separating concerns between domains, UI, and pages.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/softia.css"/>
        <Title text="SoftIA - Transformez vos données en avantage concurrentiel"/>
        <Meta name="description" content="SoftIA accompagne les PME du Centre-Val de Loire dans leur transformation data et IA. Audit, démonstrateur et solutions sur mesure. ROI garanti dès 3 mois."/>
        <Meta name="keywords" content="data science, intelligence artificielle, PME, Centre-Val de Loire, analyse de données, IA, solutions sur mesure, consultation, audit data"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        // Open Graph meta tags
        <Meta property="og:title" content="SoftIA - Data et IA pour les PME du Centre-Val de Loire"/>
        <Meta property="og:description" content="Solutions personnalisées d'analyse et d'implémentation d'outils data/IA pour les PME. Offres Exploration (4000€) et Bâtisseur (15000€)."/>
        <Meta property="og:type" content="website"/>
        <Meta property="og:locale" content="fr_FR"/>

        <Router>
            <main class="app-main">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/about" view=AboutPage/>
                    <Route path="/services" view=ServicesPage/>
                    <Route path="/process" view=ProcessPage/>
                    <Route path="/case-studies" view=CaseStudiesPage/>
                    <Route path="/contact" view=ContactPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Application entry point
///
/// This function initializes the WASM application and mounts it to the DOM.
/// It includes error handling and logging setup for better debugging experience.
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    // Set up panic hook for better error reporting
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Initialize logging
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");

    log::info!("🚀 SoftIA application starting...");
    log::info!("Domain-driven architecture initialized");

    // Mount the application
    leptos::mount_to_body(App);

    log::info!("✅ SoftIA application mounted successfully");
}
