//! Case Studies page component
//!
//! This page showcases CraftData's client success stories and case studies using
//! modern UI components with Tailwind CSS styling.

use crate::domain::{CaseStudy, CaseStudyAnalytics};
use crate::ui::components::{
    ContentSection, CtaSection, FeatureCard, HeroSection, StatItem, StatsSection, TestimonialCard,
};
use crate::ui::{Footer, Header};
use leptos::*;

#[component]
pub fn CaseStudiesPage() -> impl IntoView {
    let case_studies = CaseStudy::get_all_case_studies();
    let portfolio_stats = CaseStudyAnalytics::get_portfolio_stats();

    view! {
        <div class="min-h-screen bg-white">
            <Header/>

            // Hero Section
            <HeroSection
                title="Nos réalisations".to_string()
                subtitle="Success stories de nos clients".to_string()
                description="Découvrez comment nous avons aidé des PME à transformer leurs données en avantage concurrentiel".to_string()
                cta_text="Discuter de votre projet".to_string()
                cta_href="/contact".to_string()
                centered=true
                background_variant="bg-gradient-to-br from-secondary-600 to-secondary-800".to_string()
            >
                <div></div>
            </HeroSection>

            // Portfolio Stats Section
            <StatsSection
                title="Résultats de notre portfolio".to_string()
                subtitle="Performance mesurée sur l'ensemble de nos projets clients".to_string()
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                    <StatItem
                        value=portfolio_stats.total_clients.to_string()
                        label="Projets livrés".to_string()
                        icon="🚀".to_string()
                    />
                    <StatItem
                        value=portfolio_stats.format_average_roi()
                        label="ROI moyen".to_string()
                        icon="📈".to_string()
                    />
                    <StatItem
                        value=portfolio_stats.format_payback()
                        label="Retour sur investissement".to_string()
                        icon="💰".to_string()
                    />
                    <StatItem
                        value=format!("{}m", portfolio_stats.average_implementation_months)
                        label="Durée moyenne de mise en œuvre".to_string()
                        icon="⏱️".to_string()
                    />
                </div>
            </StatsSection>

            // Case Studies Section
            <ContentSection
                title="Études de cas détaillées".to_string()
                subtitle="Résultats concrets obtenus chez nos clients".to_string()
                background="bg-gray-50".to_string()
                centered=true
            >
                <div class="flex overflow-x-auto snap-x snap-mandatory space-x-6 pb-4 mt-12 scrollbar-hide">
                    {case_studies.into_iter().map(|case| {
                        let solution_icon = case.solution_type.get_icon();
                        let solution_label = case.solution_type.get_label();
                        let is_featured = case.is_featured;

                        view! {
                            <div class=format!("flex-none w-full md:w-full lg:w-full xl:w-11/12 relative p-8 lg:p-12 rounded-3xl border-2 shadow-medium transition-all duration-300 hover:shadow-large hover:-translate-y-1 snap-center {}",
                                if is_featured {
                                    "border-primary-200 bg-gradient-to-br from-primary-50 to-primary-100"
                                } else {
                                    "border-gray-200 bg-white hover:border-primary-200"
                                })>

                                {if is_featured {
                                    view! {
                                        <div class="absolute -top-4 left-8 px-6 py-2 bg-gradient-to-r from-primary-500 to-primary-600 text-white font-semibold rounded-full shadow-lg">
                                            "⭐ Cas d'étude phare"
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}

                                // Case Header
                                <div class="mb-8 mt-4">
                                    <div class="flex items-center mb-4">
                                        <span class="text-3xl mr-3">{solution_icon}</span>
                                        <span class="px-3 py-1 bg-secondary-100 text-secondary-700 rounded-full text-sm font-medium">
                                            {solution_label}
                                        </span>
                                    </div>
                                    <h3 class="text-2xl lg:text-3xl font-bold text-secondary-900 mb-4">
                                        {&case.title}
                                    </h3>
                                    <div class="flex flex-wrap items-center gap-4 text-secondary-600">
                                        <span class="flex items-center">
                                            <span class="w-2 h-2 bg-primary-500 rounded-full mr-2"></span>
                                            {&case.client_name}
                                        </span>
                                        <span class="flex items-center">
                                            <span class="w-2 h-2 bg-secondary-400 rounded-full mr-2"></span>
                                            {case.client_size_employees} " employés"
                                        </span>
                                        <span class="flex items-center">
                                            <span class="w-2 h-2 bg-accent-500 rounded-full mr-2"></span>
                                            {&case.completion_date}
                                        </span>
                                    </div>
                                </div>

                                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                                    // Left Column
                                    <div class="space-y-6">
                                        // Challenge
                                        <div class="bg-red-50 p-6 rounded-2xl border border-red-200">
                                            <h4 class="text-lg font-bold text-red-800 mb-3 flex items-center">
                                                <span class="text-2xl mr-3">"🎯"</span>
                                                "Défi"
                                            </h4>
                                            <p class="text-red-700 leading-relaxed">{&case.challenge_description}</p>
                                        </div>

                                        // Solution
                                        <div class="bg-blue-50 p-6 rounded-2xl border border-blue-200">
                                            <h4 class="text-lg font-bold text-blue-800 mb-3 flex items-center">
                                                <span class="text-2xl mr-3">"💡"</span>
                                                "Solution"
                                            </h4>
                                            <p class="text-blue-700 leading-relaxed mb-4">{&case.solution_description}</p>

                                            {if !case.technologies_used.is_empty() {
                                                view! {
                                                    <div>
                                                        <h5 class="font-semibold text-blue-800 mb-2">"Technologies utilisées :"</h5>
                                                        <div class="flex flex-wrap gap-2">
                                                            {case.technologies_used.iter().map(|tech| {
                                                                view! {
                                                                    <span class="px-3 py-1 bg-blue-200 text-blue-800 rounded-full text-sm font-medium">
                                                                        {tech}
                                                                    </span>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </div>
                                                    </div>
                                                }.into_view()
                                            } else {
                                                view! { <div></div> }.into_view()
                                            }}
                                        </div>

                                        // Duration
                                        <div class="bg-purple-50 p-4 rounded-xl border border-purple-200 flex items-center justify-between">
                                            <span class="text-purple-700 font-medium">"Durée du projet"</span>
                                            <span class="text-purple-800 font-bold">{case.formatted_duration()}</span>
                                        </div>
                                    </div>

                                    // Right Column
                                    <div class="space-y-6">
                                        // Results
                                        <div class="bg-green-50 p-6 rounded-2xl border border-green-200">
                                            <h4 class="text-lg font-bold text-green-800 mb-4 flex items-center">
                                                <span class="text-2xl mr-3">"📈"</span>
                                                "Résultats"
                                            </h4>
                                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                                {case.business_metrics.iter().map(|metric| {
                                                    view! {
                                                        <div class="text-center p-4 bg-white rounded-xl shadow-sm border border-green-200">
                                                            <div class="text-2xl font-bold text-green-700 mb-1">
                                                                {metric.format_improvement()}
                                                            </div>
                                                            <div class="text-sm font-semibold text-green-800 mb-1">
                                                                {&metric.name}
                                                            </div>
                                                            <div class="text-xs text-green-600">
                                                                {&metric.description}
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </div>
                                        </div>

                                        // ROI & Financial Impact
                                        <div class="bg-yellow-50 p-6 rounded-2xl border border-yellow-200">
                                            <h4 class="text-lg font-bold text-yellow-800 mb-4 flex items-center">
                                                <span class="text-2xl mr-3">"💰"</span>
                                                "ROI & Impact financier"
                                            </h4>
                                            <div class="grid grid-cols-2 gap-4">
                                                <div class="text-center p-3 bg-white rounded-xl border border-yellow-200">
                                                    <div class="text-sm text-yellow-700 mb-1">"Investissement"</div>
                                                    <div class="font-bold text-yellow-800">{case.roi_metrics.format_investment()}</div>
                                                </div>
                                                <div class="text-center p-3 bg-white rounded-xl border border-yellow-200">
                                                    <div class="text-sm text-yellow-700 mb-1">"Économies/an"</div>
                                                    <div class="font-bold text-yellow-800">{case.roi_metrics.format_annual_savings()}</div>
                                                </div>
                                                <div class="text-center p-3 bg-gradient-to-br from-green-100 to-green-200 rounded-xl border border-green-300">
                                                    <div class="text-sm text-green-700 mb-1">"ROI 12 mois"</div>
                                                    <div class="font-bold text-green-800 text-lg">{case.roi_metrics.roi_percentage_12months}"%"</div>
                                                </div>
                                                <div class="text-center p-3 bg-white rounded-xl border border-yellow-200">
                                                    <div class="text-sm text-yellow-700 mb-1">"Retour"</div>
                                                    <div class="font-bold text-yellow-800">{case.roi_metrics.payback_months}" mois"</div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // Testimonial
                                {if let Some(testimonial) = case.testimonial.clone() {
                                    view! {
                                        <div class="mt-8 pt-8 border-t border-gray-200">
                                            <TestimonialCard
                                                quote=testimonial
                                                author=case.client_name.clone()
                                                company=format!("{} employés", case.client_size_employees)
                                            />
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </ContentSection>

            // Solutions Overview Section
            <ContentSection
                title="Types de solutions déployées".to_string()
                subtitle="Nos domaines d'expertise couvrent l'ensemble des besoins data/IA des PME".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12">
                    <FeatureCard
                        title="⚙️ Optimisation de production".to_string()
                        description="Réduction des arrêts, optimisation des paramètres, prédiction des pannes".to_string()
                    />
                    <FeatureCard
                        title="📈 Prédiction de la demande".to_string()
                        description="Optimisation des stocks, réduction des ruptures, amélioration des marges".to_string()
                    />
                    <FeatureCard
                        title="🔍 Contrôle qualité intelligent".to_string()
                        description="Détection automatique des défauts, amélioration des processus".to_string()
                    />
                    <FeatureCard
                        title="📦 Gestion intelligente des stocks".to_string()
                        description="Optimisation des niveaux, automatisation des commandes".to_string()
                    />
                    <FeatureCard
                        title="🤖 Automatisation de processus".to_string()
                        description="RPA avec IA, workflows intelligents, gain de productivité".to_string()
                    />
                    <FeatureCard
                        title="🧠 Assistant IA".to_string()
                        description="Accès instantané à l'information, veille réglementaire, capitalisation des savoirs".to_string()
                        highlighted=true
                    />
                    <FeatureCard
                        title="👥 Analytics clients".to_string()
                        description="Segmentation, prédiction de churn, personnalisation".to_string()
                    />
                </div>
            </ContentSection>

            // Success Factors Section
            <ContentSection
                title="Facteurs clés de succès".to_string()
                background="bg-gradient-to-br from-gray-50 to-gray-100".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mt-12">
                    <FeatureCard
                        title="🎯 Cas d'usage bien définis".to_string()
                        description="Nous commençons toujours par identifier les cas d'usage à fort impact business avant de choisir les technologies.".to_string()
                    />
                    <FeatureCard
                        title="📊 Données de qualité".to_string()
                        description="L'audit initial permet d'identifier et de corriger les problèmes de qualité des données dès le départ.".to_string()
                    />
                    <FeatureCard
                        title="👥 Implication des équipes".to_string()
                        description="La réussite dépend de l'adoption par les utilisateurs finaux. Nous les impliquons dans tout le processus.".to_string()
                    />
                    <FeatureCard
                        title="🚀 Approche progressive".to_string()
                        description="Nous privilégions une approche par étapes avec des résultats rapides plutôt qu'un big bang.".to_string()
                        highlighted=true
                    />
                </div>
            </ContentSection>

            // Final CTA
            <CtaSection
                title="Votre entreprise pourrait être la prochaine success story".to_string()
                description="Discutons de votre contexte et identifions ensemble votre potentiel de transformation".to_string()
                cta_text="Réserver un audit gratuit".to_string()
                cta_href="/contact".to_string()
                variant="gradient".to_string()
            />

            // Additional info section
            <div class="bg-white py-8">
                <div class="container-soft">
                    <p class="text-center text-secondary-600 text-sm">
                        "Audit de 30 minutes • Sans engagement • Recommandations personnalisées"
                    </p>
                </div>
            </div>

            <Footer/>
        </div>
    }
}
