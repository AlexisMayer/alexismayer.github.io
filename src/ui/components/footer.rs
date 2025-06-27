//! Modern Footer component with Tailwind CSS
//!
//! This component provides a sleek, informative footer with glass morphism effects,
//! organized sections, and responsive design.

use crate::domain::SoftiaCompany;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Footer() -> impl IntoView {
    let company_info = SoftiaCompany::get_info();

    view! {
        <footer class="relative bg-gradient-to-br from-secondary-900 via-secondary-800 to-secondary-900">
            // Background pattern
            <div class="absolute inset-0 bg-hero-pattern opacity-5"></div>

            <div class="relative">
                // Main footer content
                <div class="container-soft section-py-sm">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 lg:gap-12">

                        // Company info section
                        <div class="lg:col-span-2">
                            <div class="flex items-center space-x-3 mb-6">
                                <div class="w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-600 rounded-xl flex items-center justify-center shadow-lg">
                                    <span class="text-white font-bold text-xl">"S"</span>
                                </div>
                                <div>
                                    <h3 class="text-2xl font-bold text-white">"SoftIA"</h3>
                                    <p class="text-secondary-300 text-sm">"Data & IA pour PME"</p>
                                </div>
                            </div>

                            <p class="text-secondary-300 text-lg leading-relaxed mb-6 max-w-md">
                                {company_info.mission}
                            </p>

                            // Contact info
                            <div class="space-y-3">
                                <div class="flex items-center space-x-3 text-secondary-300">
                                    <div class="w-5 h-5 flex-shrink-0">
                                        <svg fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <span>{company_info.target_region}</span>
                                </div>

                                <div class="flex items-center space-x-3 text-secondary-300">
                                    <div class="w-5 h-5 flex-shrink-0">
                                        <svg fill="currentColor" viewBox="0 0 20 20">
                                            <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"/>
                                            <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"/>
                                        </svg>
                                    </div>
                                    <a href="mailto:contact@softia.fr" class="hover:text-primary-400 transition-colors">
                                        "contact@softia.fr"
                                    </a>
                                </div>

                                <div class="flex items-center space-x-3 text-secondary-300">
                                    <div class="w-5 h-5 flex-shrink-0">
                                        <svg fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <span>{company_info.founder.name}" - "{company_info.founder.title}</span>
                                </div>
                            </div>
                        </div>

                        // Navigation links
                        <div>
                            <h4 class="text-white font-semibold text-lg mb-6">"Navigation"</h4>
                            <nav class="space-y-4">
                                <FooterLink href="/">"Accueil"</FooterLink>
                                <FooterLink href="/about">"À propos"</FooterLink>
                                <FooterLink href="/services">"Services"</FooterLink>
                                <FooterLink href="/case-studies">"Réalisations"</FooterLink>
                                <FooterLink href="/contact">"Contact"</FooterLink>
                            </nav>
                        </div>

                        // Services section
                        <div>
                            <h4 class="text-white font-semibold text-lg mb-6">"Nos Services"</h4>
                            <div class="space-y-4">
                                <div class="text-secondary-300">
                                    <p class="font-medium text-primary-400">"Offre Exploration"</p>
                                    <p class="text-sm">"4 000€ HT"</p>
                                </div>
                                <div class="text-secondary-300">
                                    <p class="font-medium text-primary-400">"Offre Bâtisseur"</p>
                                    <p class="text-sm">"15 000€ HT"</p>
                                </div>
                                <div class="text-secondary-300">
                                    <p class="font-medium text-primary-400">"Support & Évolution"</p>
                                    <p class="text-sm">"2 500€/mois"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Bottom bar
                <div class="border-t border-secondary-700/50">
                    <div class="container-soft py-6">
                        <div class="flex flex-col md:flex-row justify-between items-center space-y-4 md:space-y-0">
                            <div class="flex items-center space-x-6 text-secondary-400 text-sm">
                                <span>"© "{company_info.founded_year}" SoftIA. Tous droits réservés."</span>
                                <span class="hidden md:inline">"•"</span>
                                <span class="text-xs">{company_info.legal_form}</span>
                            </div>

                            // Legal links
                            <div class="flex items-center space-x-6 text-sm">
                                <a href="/mentions-legales" class="text-secondary-400 hover:text-primary-400 transition-colors">
                                    "Mentions légales"
                                </a>
                                <a href="/politique-confidentialite" class="text-secondary-400 hover:text-primary-400 transition-colors">
                                    "Confidentialité"
                                </a>
                            </div>
                        </div>
                    </div>
                </div>

                // Subtle call-to-action
                <div class="bg-gradient-to-r from-primary-600 to-primary-700">
                    <div class="container-soft py-8">
                        <div class="text-center">
                            <h3 class="text-white text-xl font-semibold mb-2">
                                "Prêt à transformer vos données ?"
                            </h3>
                            <p class="text-primary-100 mb-4">
                                "Discutons de votre projet lors d'un audit gratuit de 30 minutes"
                            </p>
                            <A href="/contact" class="inline-flex items-center px-6 py-3 bg-white text-primary-600 font-semibold rounded-xl hover:bg-primary-50 transition-colors duration-300 shadow-lg hover:shadow-xl">
                                "Planifier un audit gratuit"
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                                </svg>
                            </A>
                        </div>
                    </div>
                </div>
            </div>
        </footer>
    }
}

#[component]
fn FooterLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <A
            href=href
            class="block text-secondary-300 hover:text-primary-400 transition-colors duration-200 group"
        >
            <span class="flex items-center">
                {children()}
                <svg class="w-3 h-3 ml-1 opacity-0 group-hover:opacity-60 transform translate-x-0 group-hover:translate-x-1 transition-all duration-200" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                </svg>
            </span>
        </A>
    }
}
