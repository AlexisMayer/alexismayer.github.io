//! Main layout component for the Softia application
//!
//! This component provides the primary layout structure used across
//! most pages of the application with modern Tailwind CSS styling.

use crate::ui::{Footer, Header};
use leptos::*;

/// Main layout component that wraps page content with modern styling
#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white flex flex-col">
            <Header/>

            <main class="flex-1">
                {children()}
            </main>

            <Footer/>
        </div>
    }
}

/// Alternative layout for landing pages with full-width hero sections
#[component]
pub fn LandingLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            {children()}
        </div>
    }
}

/// Minimal layout for error pages or simple content
#[component]
pub fn MinimalLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 flex items-center justify-center">
            <div class="max-w-md w-full mx-auto p-6">
                {children()}
            </div>
        </div>
    }
}
