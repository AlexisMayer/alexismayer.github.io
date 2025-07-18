//! About page component
//!
//! This page presents CraftData's mission, founder information, and company approach
//! using modern UI components with Tailwind CSS styling.

use crate::domain::CraftDataCompany;
use crate::ui::components::{ContentSection, FeatureCard, HeroSection};
use crate::ui::{Footer, Header};
use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    let company_info = CraftDataCompany::get_info();
    let founder = company_info.founder;

    view! {
        <div class="min-h-screen bg-white">
            <Header/>

            // Hero Section
            <HeroSection
                title="L'expertise data au service des PME".to_string()
                subtitle=format!("Depuis {}, CraftData démocratise l'accès à la data science et à l'IA", company_info.founded_year)

                centered=true

            >
                <div></div>
            </HeroSection>

            // Founder Story Section
            <ContentSection

                background="bg-gray-50".to_string()
            >
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">

                    <div class="flex justify-center">
                        <div class="w-80 h-80 bg-gradient-to-br from-primary-100 to-primary-200 rounded-3xl flex items-center justify-center shadow-lg border border-primary-300">
                            <div class="text-center">
                                <div class="w-32 h-32 bg-gradient-to-br from-primary-500 to-primary-600 rounded-full flex items-center justify-center mx-auto mb-4 shadow-xl">
                                    <span class="text-white text-4xl font-bold">
                                        {founder.name.chars().next().unwrap_or('?')}
                                    </span>
                                </div>
                                <div class="text-primary-800 font-semibold text-lg">
                                    {founder.name}
                                </div>
                                <div class="text-primary-600 text-sm">
                                    {founder.title}
                                </div>
                            </div>
                        </div>
                    </div>
                    <div>
                        <div class="prose prose-lg max-w-none">


                            <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200 mb-8">
                                <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                                    <span class="text-2xl mr-3">"🎓"</span>
                                    "Parcours"
                                </h3>
                                <ul class="space-y-3">
                                    <li class="flex items-start">
                                        <span class="text-primary-500 font-bold mr-3 mt-1">"•"</span>
                                        <span class="text-secondary-700">{founder.title} " diplômé de " {founder.education}</span>
                                    </li>
                                    <li class="flex items-start">
                                        <span class="text-primary-500 font-bold mr-3 mt-1">"•"</span>
                                        <span class="text-secondary-700">{founder.experience_years} "+ années d'expérience en IA et analyse de données"</span>
                                    </li>
                                    <li class="flex items-start">
                                        <span class="text-primary-500 font-bold mr-3 mt-1">"•"</span>
                                        <span class="text-secondary-700">"15+ projets data livrés avec ROI mesurable"</span>
                                    </li>
                                    <li class="flex items-start">
                                        <span class="text-primary-500 font-bold mr-3 mt-1">"•"</span>
                                        <span class="text-secondary-700">"Spécialisé dans l'accompagnement des PME industrielles"</span>
                                    </li>
                                </ul>
                            </div>

                            <div class="bg-gradient-to-br from-primary-50 to-primary-100 p-6 rounded-2xl border border-primary-200">
                                <h3 class="text-xl font-bold text-primary-800 mb-4 flex items-center">
                                    <span class="text-2xl mr-3">"💡"</span>
                                    "Ma philosophie"
                                </h3>
                                <p class="text-primary-700 italic text-lg leading-relaxed">
                                    {founder.philosophy}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </ContentSection>

            // Our Approach Section
            <ContentSection
                title="Notre approche unique".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mt-12">
                    <FeatureCard
                        title="🎯 Pragmatique".to_string()
                        description="Pas de buzzwords. Nous nous concentrons sur des solutions qui génèrent un ROI mesurable rapidement.".to_string()
                    />
                    <FeatureCard
                        title="📚 Pédagogique".to_string()
                        description="Nous formons vos équipes pour qu'elles deviennent autonomes sur les outils déployés.".to_string()
                    />
                    <FeatureCard
                        title="🏠 Proximité".to_string()
                        description=format!("Basés en région {}, nous nous déplaçons chez vous pour comprendre votre contexte.", company_info.target_region)
                        highlighted=true
                    />
                    <FeatureCard
                        title="🔓 Transparence".to_string()
                        description="Code source remis, pas de dépendance. Vous restez propriétaire de vos solutions.".to_string()
                    />
                </div>
            </ContentSection>

            // Company Mission Section
            <ContentSection
                title="Notre mission".to_string()
                background="bg-gradient-to-br from-gray-50 to-gray-100".to_string()
                centered=true
            >
                <div class="max-w-4xl mx-auto">
                    <div class="bg-white p-8 rounded-3xl shadow-medium border border-gray-200 mb-12">
                        <p class="text-xl text-secondary-700 leading-relaxed text-center mb-8">
                            {company_info.mission}
                        </p>

                        <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                            <div class="text-center p-6 bg-gradient-to-br from-primary-50 to-primary-100 rounded-2xl border border-primary-200">
                                <div class="text-3xl font-bold text-gradient-primary mb-2">
                                    {company_info.founded_year}
                                </div>
                                <div class="text-secondary-600 font-medium">
                                    "Année de création"
                                </div>
                            </div>

                            <div class="text-center p-6 bg-gradient-to-br from-secondary-50 to-secondary-100 rounded-2xl border border-secondary-200">
                                <div class="text-lg font-bold text-secondary-800 mb-2">
                                    {company_info.legal_form}
                                </div>
                                <div class="text-secondary-600 font-medium">
                                    "Forme juridique"
                                </div>
                            </div>

                            <div class="text-center p-6 bg-gradient-to-br from-accent-50 to-accent-100 rounded-2xl border border-accent-200">
                                <div class="text-lg font-bold text-accent-800 mb-2">
                                    {company_info.target_region}
                                </div>
                                <div class="text-secondary-600 font-medium">
                                    "Zone d'intervention"
                                </div>
                            </div>
                        </div>
                    </div>

                    // Additional company values
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                        <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                            <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                                <span class="text-2xl mr-3">"🎯"</span>
                                "Notre engagement"
                            </h3>
                            <p class="text-secondary-700 leading-relaxed">
                                "Accompagner les PME dans leur transformation digitale avec des solutions concrètes,
                                mesurables et adaptées à leur réalité opérationnelle."
                            </p>
                        </div>

                        <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                            <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                                <span class="text-2xl mr-3">"🌟"</span>
                                "Notre différence"
                            </h3>
                            <p class="text-secondary-700 leading-relaxed">
                                "Une approche humaine et locale, combinée à une expertise technique de haut niveau
                                pour démocratiser l'accès aux technologies data et IA."
                            </p>
                        </div>
                    </div>
                </div>
            </ContentSection>

            <Footer/>
        </div>
    }
}
