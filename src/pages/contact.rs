//! Contact page component
//!
//! This page handles contact form and displays contact information using
//! modern UI components with Tailwind CSS styling.

use crate::domain::{
    get_business_sector_options, get_data_challenge_options, get_project_timeline_options,
};
use crate::ui::components::{ContentSection, HeroSection};
use crate::ui::{Footer, Header};
use leptos::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <Header/>

            // Hero Section
            <HeroSection
                title="Parlons de votre projet".to_string()
                subtitle="Audit gratuit de 30 minutes".to_string()
                description="Identifiez votre potentiel data/IA sans engagement".to_string()
                centered=true
                background_variant="bg-gradient-to-br from-primary-600 to-primary-800".to_string()
            >
                <div></div>
            </HeroSection>

            // Contact Content Section
            <ContentSection>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    // Contact Information
                    <div class="space-y-8">
                        <div>
                            <h2 class="text-2xl font-bold text-secondary-900 mb-6">
                                "Discutons de vos défis"
                            </h2>

                            <div class="bg-gradient-to-br from-primary-50 to-primary-100 p-6 rounded-2xl border border-primary-200 mb-8">
                                <h3 class="text-lg font-bold text-primary-800 mb-4 flex items-center">
                                    <span class="text-2xl mr-3">"🎯"</span>
                                    "Durant cet échange gratuit :"
                                </h3>
                                <ul class="space-y-3">
                                    <li class="flex items-start">
                                        <span class="text-green-500 font-bold mr-3 mt-0.5">"✓"</span>
                                        <span class="text-primary-700">"Analyse de votre contexte data actuel"</span>
                                    </li>
                                    <li class="flex items-start">
                                        <span class="text-green-500 font-bold mr-3 mt-0.5">"✓"</span>
                                        <span class="text-primary-700">"Identification de 2-3 cas d'usage prioritaires"</span>
                                    </li>
                                    <li class="flex items-start">
                                        <span class="text-green-500 font-bold mr-3 mt-0.5">"✓"</span>
                                        <span class="text-primary-700">"Estimation du ROI potentiel"</span>
                                    </li>
                                    <li class="flex items-start">
                                        <span class="text-green-500 font-bold mr-3 mt-0.5">"✓"</span>
                                        <span class="text-primary-700">"Recommandations personnalisées"</span>
                                    </li>
                                </ul>
                            </div>
                        </div>

                        // Founder Contact
                        <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                            <div class="flex items-center mb-4">
                                <div class="w-16 h-16 bg-gradient-to-br from-secondary-500 to-secondary-600 rounded-full flex items-center justify-center mr-4">
                                    <span class="text-white text-xl font-bold">"A"</span>
                                </div>
                                <div>
                                    <h3 class="text-xl font-bold text-secondary-900">"Alexis Mayer"</h3>
                                    <p class="text-secondary-600">"Data Scientist & Fondateur"</p>
                                </div>
                            </div>
                            <div class="space-y-3">
                                <div class="flex items-center">
                                    <span class="text-secondary-500 mr-3">"📞"</span>
                                    <span class="text-secondary-700">"Sur rendez-vous uniquement"</span>
                                </div>
                                <div class="flex items-center">
                                    <span class="text-secondary-500 mr-3">"📍"</span>
                                    <span class="text-secondary-700">"Déplacements en région Centre-Val de Loire"</span>
                                </div>
                                <div class="flex items-center">
                                    <span class="text-secondary-500 mr-3">"💻"</span>
                                    <span class="text-secondary-700">"Visioconférence possible"</span>
                                </div>
                            </div>
                        </div>

                        // Guarantees
                        <div class="bg-green-50 p-6 rounded-2xl border border-green-200">
                            <h3 class="text-lg font-bold text-green-800 mb-4 flex items-center">
                                <span class="text-2xl mr-3">"🛡️"</span>
                                "Nos engagements :"
                            </h3>
                            <ul class="space-y-3">
                                <li class="flex items-start">
                                    <span class="text-green-600 font-bold mr-3 mt-0.5">"✅"</span>
                                    <span class="text-green-700">"Réponse sous 24h maximum"</span>
                                </li>
                                <li class="flex items-start">
                                    <span class="text-green-600 font-bold mr-3 mt-0.5">"✅"</span>
                                    <span class="text-green-700">"Confidentialité totale (NDA sur demande)"</span>
                                </li>
                                <li class="flex items-start">
                                    <span class="text-green-600 font-bold mr-3 mt-0.5">"✅"</span>
                                    <span class="text-green-700">"Pas de démarchage commercial"</span>
                                </li>
                                <li class="flex items-start">
                                    <span class="text-green-600 font-bold mr-3 mt-0.5">"✅"</span>
                                    <span class="text-green-700">"Conseils gratuits même sans mission"</span>
                                </li>
                            </ul>
                        </div>
                    </div>

                    // Contact Form
                    <div class="bg-white p-8 rounded-3xl shadow-medium border border-gray-200">
                        <h2 class="text-2xl font-bold text-secondary-900 mb-6 text-center">
                            "Demande d'audit gratuit"
                        </h2>

                        <form class="space-y-6">
                            // Company Name
                            <div>
                                <label for="company" class="block text-sm font-semibold text-secondary-700 mb-2">
                                    "Entreprise *"
                                </label>
                                <input
                                    type="text"
                                    id="company"
                                    name="company"
                                    required
                                    placeholder="Nom de votre entreprise"
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                />
                            </div>

                            // Name and Role
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <label for="name" class="block text-sm font-semibold text-secondary-700 mb-2">
                                        "Prénom Nom *"
                                    </label>
                                    <input
                                        type="text"
                                        id="name"
                                        name="name"
                                        required
                                        placeholder="Jean Dupont"
                                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                    />
                                </div>
                                <div>
                                    <label for="role" class="block text-sm font-semibold text-secondary-700 mb-2">
                                        "Fonction *"
                                    </label>
                                    <input
                                        type="text"
                                        id="role"
                                        name="role"
                                        required
                                        placeholder="Directeur Général"
                                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                    />
                                </div>
                            </div>

                            // Email and Phone
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <label for="email" class="block text-sm font-semibold text-secondary-700 mb-2">
                                        "Email *"
                                    </label>
                                    <input
                                        type="email"
                                        id="email"
                                        name="email"
                                        required
                                        placeholder="jean.dupont@entreprise.com"
                                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                    />
                                </div>
                                <div>
                                    <label for="phone" class="block text-sm font-semibold text-secondary-700 mb-2">
                                        "Téléphone"
                                    </label>
                                    <input
                                        type="tel"
                                        id="phone"
                                        name="phone"
                                        placeholder="02 38 XX XX XX"
                                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                    />
                                </div>
                            </div>

                            // Employees and Sector
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                <div>
                                    <label for="employees" class="block text-sm font-semibold text-secondary-700 mb-2">
                                        "Nombre d'employés *"
                                    </label>
                                    <select
                                        id="employees"
                                        name="employees"
                                        required
                                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                    >
                                        <option value="">"Sélectionnez"</option>
                                        <option value="10-25">"10-25 employés"</option>
                                        <option value="26-50">"26-50 employés"</option>
                                        <option value="51-100">"51-100 employés"</option>
                                        <option value="101-250">"101-250 employés"</option>
                                        <option value="250+">"250+ employés"</option>
                                    </select>
                                </div>
                                <div>
                                    <label for="sector" class="block text-sm font-semibold text-secondary-700 mb-2">
                                        "Secteur d'activité *"
                                    </label>
                                    <select
                                        id="sector"
                                        name="sector"
                                        required
                                        class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                    >
                                        <option value="">"Sélectionnez"</option>
                                        {get_business_sector_options().into_iter().map(|(value, label)| {
                                            view! { <option value={value}>{label}</option> }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                </div>
                            </div>

                            // Challenge
                            <div>
                                <label for="challenge" class="block text-sm font-semibold text-secondary-700 mb-2">
                                    "Principal défi data/IA *"
                                </label>
                                <select
                                    id="challenge"
                                    name="challenge"
                                    required
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                >
                                    <option value="">"Quel est votre principal défi ?"</option>
                                    {get_data_challenge_options().into_iter().map(|(value, label)| {
                                        view! { <option value={value}>{label}</option> }
                                    }).collect::<Vec<_>>()}
                                </select>
                            </div>

                            // Current Tools
                            <div>
                                <label for="current-tools" class="block text-sm font-semibold text-secondary-700 mb-2">
                                    "Outils actuels"
                                </label>
                                <input
                                    type="text"
                                    id="current-tools"
                                    name="current-tools"
                                    placeholder="ERP (SAP, Odoo...), CRM, Excel, autres..."
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                />
                            </div>

                            // Timeline
                            <div>
                                <label for="timeline" class="block text-sm font-semibold text-secondary-700 mb-2">
                                    "Horizon projet"
                                </label>
                                <select
                                    id="timeline"
                                    name="timeline"
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200"
                                >
                                    <option value="">"Quand souhaitez-vous commencer ?"</option>
                                    {get_project_timeline_options().into_iter().map(|(value, label)| {
                                        view! { <option value={value}>{label}</option> }
                                    }).collect::<Vec<_>>()}
                                </select>
                            </div>

                            // Message
                            <div>
                                <label for="message" class="block text-sm font-semibold text-secondary-700 mb-2">
                                    "Décrivez votre contexte (optionnel)"
                                </label>
                                <textarea
                                    id="message"
                                    name="message"
                                    rows="4"
                                    placeholder="Décrivez brièvement votre situation actuelle, vos objectifs, ou toute information utile pour préparer notre échange..."
                                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-primary-500 focus:border-primary-500 transition-colors duration-200 resize-vertical min-h-[100px]"
                                ></textarea>
                            </div>

                            // Submit Button
                            <button
                                type="submit"
                                class="w-full bg-gradient-to-r from-primary-500 to-primary-600 text-white font-bold py-4 px-8 rounded-xl shadow-lg hover:shadow-xl transition-all duration-300 hover:scale-105 active:scale-95 focus:ring-4 focus:ring-primary-200"
                            >
                                "Réserver mon audit gratuit (30 min)"
                            </button>

                            <p class="text-center text-sm text-secondary-600">
                                "* Champs obligatoires • Réponse sous 24h • Audit sans engagement"
                            </p>
                        </form>
                    </div>
                </div>
            </ContentSection>

            <Footer/>
        </div>
    }
}
