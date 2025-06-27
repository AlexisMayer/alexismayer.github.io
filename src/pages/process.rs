//! Process page component
//!
//! This page explains SoftIA's methodology and implementation process using
//! modern UI components with Tailwind CSS styling.

use crate::ui::components::{ContentSection, CtaSection, FeatureCard, HeroSection};
use crate::ui::{Footer, Header};
use leptos::*;

#[component]
pub fn ProcessPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <Header/>

            // Hero Section
            <HeroSection
                title="Notre méthodologie éprouvée".to_string()
                subtitle="Un processus structuré et agile".to_string()
                description="5 étapes clés pour maximiser vos chances de succès dans votre transformation data/IA".to_string()
                cta_text="Découvrir nos services".to_string()
                cta_href="/services".to_string()
                centered=true
                background_variant="bg-gradient-to-br from-accent-600 to-accent-800".to_string()
            >
                <div></div>
            </HeroSection>

            // Process Timeline Section
            <ContentSection
                title="5 étapes pour transformer vos données".to_string()
                subtitle="Un processus itératif avec des résultats mesurables à chaque étape".to_string()
                centered=true
            >
                <div class="max-w-4xl mx-auto mt-12">
                    <div class="space-y-12">
                        // Step 1: Discovery & Audit
                        <div class="relative flex items-start group">
                            <div class="flex-shrink-0 w-16 h-16 bg-gradient-to-br from-primary-500 to-primary-600 rounded-2xl flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300 relative z-10">
                                <span class="text-white text-xl font-bold">"1"</span>
                            </div>
                            <div class="ml-8 flex-grow">
                                <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200 hover:shadow-medium transition-all duration-300">
                                    <h3 class="text-xl font-bold text-secondary-900 mb-3 flex items-center">
                                        "Découverte & Audit"
                                        <span class="ml-3 px-3 py-1 bg-primary-100 text-primary-700 rounded-full text-sm font-medium">
                                            "1-2 semaines"
                                        </span>
                                    </h3>
                                    <p class="text-secondary-700 leading-relaxed mb-4">
                                        "Compréhension de votre contexte métier, cartographie des données existantes, identification des cas d'usage à fort impact."
                                    </p>
                                    <div class="bg-blue-50 p-4 rounded-xl border border-blue-200">
                                        <h4 class="font-semibold text-blue-800 mb-3">"Livrables :"</h4>
                                        <ul class="space-y-2">
                                            <li class="flex items-start">
                                                <span class="text-blue-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-blue-700">"Cartographie des sources de données"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-blue-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-blue-700">"Analyse de la qualité des données"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-blue-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-blue-700">"Benchmark des outils existants"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-blue-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-blue-700">"Identification des cas d'usage prioritaires"</span>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Step 2: Design & Validation
                        <div class="relative flex items-start group">
                            <div class="flex-shrink-0 w-16 h-16 bg-gradient-to-br from-secondary-500 to-secondary-600 rounded-2xl flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300 relative z-10">
                                <span class="text-white text-xl font-bold">"2"</span>
                            </div>
                            <div class="ml-8 flex-grow">
                                <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200 hover:shadow-medium transition-all duration-300">
                                    <h3 class="text-xl font-bold text-secondary-900 mb-3 flex items-center">
                                        "Conception & Validation"
                                        <span class="ml-3 px-3 py-1 bg-secondary-100 text-secondary-700 rounded-full text-sm font-medium">
                                            "1 semaine"
                                        </span>
                                    </h3>
                                    <p class="text-secondary-700 leading-relaxed mb-4">
                                        "Architecture technique, choix des technologies, validation des approches avec vos équipes métier."
                                    </p>
                                    <div class="bg-green-50 p-4 rounded-xl border border-green-200">
                                        <h4 class="font-semibold text-green-800 mb-3">"Livrables :"</h4>
                                        <ul class="space-y-2">
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Architecture technique détaillée"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Spécifications fonctionnelles"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Plan de développement"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Validation des hypothèses métier"</span>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Step 3: Agile Development
                        <div class="relative flex items-start group">
                            <div class="flex-shrink-0 w-16 h-16 bg-gradient-to-br from-accent-500 to-accent-600 rounded-2xl flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300 relative z-10">
                                <span class="text-white text-xl font-bold">"3"</span>
                            </div>
                            <div class="ml-8 flex-grow">
                                <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200 hover:shadow-medium transition-all duration-300">
                                    <h3 class="text-xl font-bold text-secondary-900 mb-3 flex items-center">
                                        "Développement Agile"
                                        <span class="ml-3 px-3 py-1 bg-accent-100 text-accent-700 rounded-full text-sm font-medium">
                                            "2-4 semaines"
                                        </span>
                                    </h3>
                                    <p class="text-secondary-700 leading-relaxed mb-4">
                                        "Développement itératif avec démonstrations régulières, ajustements selon vos retours."
                                    </p>
                                    <div class="bg-purple-50 p-4 rounded-xl border border-purple-200">
                                        <h4 class="font-semibold text-purple-800 mb-3">"Approche :"</h4>
                                        <ul class="space-y-2">
                                            <li class="flex items-start">
                                                <span class="text-purple-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-purple-700">"Sprints de 1-2 semaines"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-purple-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-purple-700">"Démonstrations régulières"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-purple-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-purple-700">"Tests en continu"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-purple-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-purple-700">"Ajustements selon vos retours"</span>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Step 4: Deployment & Training
                        <div class="relative flex items-start group">
                            <div class="flex-shrink-0 w-16 h-16 bg-gradient-to-br from-yellow-500 to-yellow-600 rounded-2xl flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300 relative z-10">
                                <span class="text-white text-xl font-bold">"4"</span>
                            </div>
                            <div class="ml-8 flex-grow">
                                <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200 hover:shadow-medium transition-all duration-300">
                                    <h3 class="text-xl font-bold text-secondary-900 mb-3 flex items-center">
                                        "Déploiement & Formation"
                                        <span class="ml-3 px-3 py-1 bg-yellow-100 text-yellow-700 rounded-full text-sm font-medium">
                                            "1-2 semaines"
                                        </span>
                                    </h3>
                                    <p class="text-secondary-700 leading-relaxed mb-4">
                                        "Mise en production, formation de vos équipes, documentation complète."
                                    </p>
                                    <div class="bg-yellow-50 p-4 rounded-xl border border-yellow-200">
                                        <h4 class="font-semibold text-yellow-800 mb-3">"Activités :"</h4>
                                        <ul class="space-y-2">
                                            <li class="flex items-start">
                                                <span class="text-yellow-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-yellow-700">"Déploiement en production"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-yellow-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-yellow-700">"Formation des utilisateurs"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-yellow-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-yellow-700">"Documentation technique"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-yellow-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-yellow-700">"Tests de performance"</span>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>

                        // Step 5: Follow-up & Optimization
                        <div class="relative flex items-start group">
                            <div class="flex-shrink-0 w-16 h-16 bg-gradient-to-br from-green-500 to-green-600 rounded-2xl flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform duration-300 relative z-10">
                                <span class="text-white text-xl font-bold">"5"</span>
                            </div>
                            <div class="ml-8 flex-grow">
                                <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200 hover:shadow-medium transition-all duration-300">
                                    <h3 class="text-xl font-bold text-secondary-900 mb-3 flex items-center">
                                        "Suivi & Optimisation"
                                        <span class="ml-3 px-3 py-1 bg-green-100 text-green-700 rounded-full text-sm font-medium">
                                            "En continu"
                                        </span>
                                    </h3>
                                    <p class="text-secondary-700 leading-relaxed mb-4">
                                        "Mesure des résultats, optimisations, évolutions selon les nouveaux besoins."
                                    </p>
                                    <div class="bg-green-50 p-4 rounded-xl border border-green-200">
                                        <h4 class="font-semibold text-green-800 mb-3">"Services :"</h4>
                                        <ul class="space-y-2">
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Monitoring des performances"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Support technique"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Évolutions fonctionnelles"</span>
                                            </li>
                                            <li class="flex items-start">
                                                <span class="text-green-600 font-bold mr-2 mt-0.5">"•"</span>
                                                <span class="text-green-700">"Formation continue"</span>
                                            </li>
                                        </ul>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </ContentSection>

            // Methodology Principles Section
            <ContentSection
                title="Nos principes méthodologiques".to_string()
                subtitle="Les valeurs qui guident notre approche pour chaque projet".to_string()
                background="bg-gray-50".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12">
                    <FeatureCard
                        title="🎯 Orienté ROI".to_string()
                        description="Chaque développement est priorisé selon son impact business. Nous commençons par les cas d'usage à plus forte valeur ajoutée.".to_string()
                    />
                    <FeatureCard
                        title="🚀 Approche agile".to_string()
                        description="Développement itératif avec démonstrations fréquentes pour ajuster le cap selon vos retours et l'évolution de vos besoins.".to_string()
                    />
                    <FeatureCard
                        title="📊 Data-driven".to_string()
                        description="Toutes nos décisions techniques sont basées sur l'analyse de vos données réelles, pas sur des hypothèses théoriques.".to_string()
                        highlighted=true
                    />
                    <FeatureCard
                        title="👥 Co-création".to_string()
                        description="Vos équipes métier sont impliquées à chaque étape pour garantir l'adoption et l'adéquation avec vos processus.".to_string()
                    />
                    <FeatureCard
                        title="🔧 Pragmatique".to_string()
                        description="Nous privilégions les solutions simples et robustes aux technologies complexes. L'objectif est l'efficacité, pas la sophistication.".to_string()
                    />
                    <FeatureCard
                        title="📚 Transfert de compétences".to_string()
                        description="Formation approfondie de vos équipes pour qu'elles deviennent autonomes sur les outils et méthodes déployés.".to_string()
                    />
                </div>
            </ContentSection>

            // Quality Assurance Section
            <ContentSection
                title="Garanties qualité".to_string()
                subtitle="Standards techniques et méthodologiques pour assurer le succès de votre projet".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 mt-12">
                    <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                        <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                            <span class="text-2xl mr-3">"📋"</span>
                            "Standards techniques"
                        </h3>
                        <ul class="space-y-3">
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Code source commenté et documenté"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Tests automatisés"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Architecture évolutive"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Bonnes pratiques de sécurité"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                        <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                            <span class="text-2xl mr-3">"🔍"</span>
                            "Validation métier"
                        </h3>
                        <ul class="space-y-3">
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Tests utilisateurs réguliers"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Validation des résultats"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Mesure du ROI"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Ajustements selon feedback"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                        <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                            <span class="text-2xl mr-3">"📖"</span>
                            "Documentation"
                        </h3>
                        <ul class="space-y-3">
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Manuel utilisateur détaillé"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Documentation technique"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Procédures de maintenance"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Formation vidéo"</span>
                            </li>
                        </ul>
                    </div>

                    <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                        <h3 class="text-xl font-bold text-secondary-900 mb-4 flex items-center">
                            <span class="text-2xl mr-3">"🛡️"</span>
                            "Support"
                        </h3>
                        <ul class="space-y-3">
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Garantie 3 mois incluse"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Support téléphonique"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Corrections de bugs"</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-primary-500 font-bold mr-3 mt-0.5">"✓"</span>
                                <span class="text-secondary-700">"Évolutions mineures"</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </ContentSection>

            // Final CTA
            <CtaSection
                title="Prêt à démarrer votre projet ?".to_string()
                description="Discutons de votre contexte et de vos objectifs lors d'un audit gratuit".to_string()
                cta_text="Réserver un audit gratuit".to_string()
                cta_href="/contact".to_string()
                secondary_cta_text="Voir nos réalisations".to_string()
                secondary_cta_href="/case-studies".to_string()
                variant="gradient".to_string()
            />

            // Additional info section
            <div class="bg-white py-8">
                <div class="container-soft">
                    <p class="text-center text-secondary-600 text-sm">
                        "Premier échange de 30 minutes • Sans engagement • Conseils personnalisés"
                    </p>
                </div>
            </div>

            <Footer/>
        </div>
    }
}
