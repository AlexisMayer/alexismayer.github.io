//! Services page component
//!
//! This page presents all service offerings in detail using domain entities
//! and modern UI components with Tailwind CSS styling.

use crate::domain::{get_services_by_category, ServiceCategory};
use crate::ui::components::{ContentSection, CtaSection, HeroSection};
use crate::ui::{Footer, Header};
use leptos::*;

#[component]
pub fn ServicesPage() -> impl IntoView {
    let main_services = get_services_by_category(ServiceCategory::CoreOfferings);
    let additional_services = get_services_by_category(ServiceCategory::AdditionalServices);

    view! {
        <div class="min-h-screen bg-white">
            <Header/>

            // Hero Section
            <HeroSection
                title="Services adaptés à votre maturité data".to_string()
                subtitle="De l'audit initial à la solution complète".to_string()
                description="".to_string()
                cta_text="Discuter de votre projet".to_string()
                cta_href="/contact".to_string()
                centered=true
                background_variant="neural-network".to_string()
            >
                <div></div>
            </HeroSection>

            // Main Services Section
            <ContentSection
                title="Nos offres principales".to_string()
                subtitle="Solutions complètes pour transformer vos données en avantage concurrentiel".to_string()
                centered=true
            >
                <div class="flex overflow-x-auto snap-x snap-mandatory space-x-6 pb-4 mt-6 scrollbar-hide">
                    {main_services.into_iter().map(|service| {
                        let is_exploration = service.service_type == crate::domain::ServiceType::Exploration;

                        view! {
                            <div class=format!("flex-none w-full md:w-full lg:w-full xl:w-11/12 relative p-8 lg:p-12 mt-4 rounded-3xl border-2 shadow-medium transition-all duration-300 hover:shadow-large hover:-translate-y-1 snap-center {}",
                                if is_exploration {
                                    "border-primary-200 bg-gradient-to-br from-primary-50 to-primary-100"
                                } else {
                                    "border-gray-200 bg-white hover:border-primary-200"
                                })>

                                {if is_exploration {
                                    view! {
                                        <div class="absolute -top-4 left-8 px-6 py-2 bg-gradient-to-r from-primary-500 to-primary-600 text-white font-semibold rounded-full shadow-lg">
                                            "⚡ Offre phare"
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}

                                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start mt-4">
                                    // Service Header
                                    <div class="lg:col-span-1">
                                        <h3 class="text-3xl lg:text-3xl font-bold text-secondary-900 mb-4">
                                            {&service.name}
                                        </h3>
                                        <div class="space-y-2 mb-6">
                                            <div class="text-3xl font-bold text-gradient-primary">
                                                {service.formatted_price()}
                                            </div>
                                            <div class="text-secondary-600 font-medium">
                                                {service.formatted_duration()}
                                            </div>
                                        </div>
                                        <p class="text-secondary-700 leading-relaxed mb-6">
                                            {&service.description}
                                        </p>
                                        <a href="/contact" class=format!("inline-flex items-center px-6 py-3 font-semibold rounded-xl transition-all duration-300 {}",
                                            if is_exploration {
                                                "bg-gradient-to-r from-primary-500 to-primary-600 text-white shadow-lg hover:shadow-xl hover:scale-105"
                                            } else {
                                                "border-2 border-primary-500 text-primary-600 hover:bg-primary-500 hover:text-white"
                                            })>
                                            "Discuter de ce service"
                                            <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                                            </svg>
                                        </a>
                                    </div>

                                    // Service Details
                                    <div class="lg:col-span-2 space-y-6">
                                        // Benefits
                                        <div class="bg-white p-6 rounded-2xl shadow-soft border border-gray-200">
                                            <h4 class="text-lg font-bold text-secondary-900 mb-4 flex items-center">
                                                <span class="text-green-500 mr-2">"✓"</span>
                                                "Ce que vous obtenez"
                                            </h4>
                                            <ul class="grid grid-cols-1 md:grid-cols-2 gap-3">
                                                {service.benefits.iter().map(|benefit| {
                                                    view! {
                                                        <li class="flex items-start">
                                                            <span class="text-green-500 font-bold mr-2 mt-0.5">"•"</span>
                                                            <span class="text-secondary-700">{benefit}</span>
                                                        </li>
                                                    }
                                                }).collect::<Vec<_>>()}
                                            </ul>
                                        </div>

                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                            // Deliverables
                                            {if !service.deliverables.is_empty() {
                                                view! {
                                                    <div class="bg-blue-50 p-6 rounded-2xl border border-blue-200">
                                                        <h4 class="text-lg font-bold text-blue-800 mb-4 flex items-center">
                                                            <span class="text-blue-600 mr-2">"📄"</span>
                                                            "Livrables"
                                                        </h4>
                                                        <ul class="space-y-2">
                                                            {service.deliverables.iter().map(|deliverable| {
                                                                view! {
                                                                    <li class="flex items-start">
                                                                        <span class="text-blue-600 font-bold mr-2 mt-0.5">"•"</span>
                                                                        <span class="text-blue-700">{deliverable}</span>
                                                                    </li>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </ul>
                                                    </div>
                                                }.into_view()
                                            } else {
                                                view! { <div></div> }.into_view()
                                            }}

                                            // Target Audience
                                            {if !service.target_audience.is_empty() {
                                                view! {
                                                    <div class="bg-purple-50 p-6 rounded-2xl border border-purple-200">
                                                        <h4 class="text-lg font-bold text-purple-800 mb-4 flex items-center">
                                                            <span class="text-purple-600 mr-2">"👤"</span>
                                                            "Idéal pour"
                                                        </h4>
                                                        <ul class="space-y-2">
                                                            {service.target_audience.iter().map(|target| {
                                                                view! {
                                                                    <li class="flex items-start">
                                                                        <span class="text-purple-600 font-bold mr-2 mt-0.5">"•"</span>
                                                                        <span class="text-purple-700">{target}</span>
                                                                    </li>
                                                                }
                                                            }).collect::<Vec<_>>()}
                                                        </ul>
                                                    </div>
                                                }.into_view()
                                            } else {
                                                view! { <div></div> }.into_view()
                                            }}
                                        </div>

                                        // Guarantee or ROI
                                        {if let Some(guarantee) = &service.guarantee {
                                            view! {
                                                <div class="bg-yellow-50 p-6 rounded-2xl border border-yellow-200">
                                                    <h4 class="text-lg font-bold text-yellow-800 mb-3 flex items-center">
                                                        <span class="text-yellow-600 mr-2">"💰"</span>
                                                        "Garantie"
                                                    </h4>
                                                    <p class="text-yellow-700 font-medium">{guarantee}</p>
                                                </div>
                                            }.into_view()
                                        } else if let Some(roi) = service.roi_percentage {
                                            view! {
                                                <div class="bg-green-50 p-6 rounded-2xl border border-green-200">
                                                    <h4 class="text-lg font-bold text-green-800 mb-3 flex items-center">
                                                        <span class="text-green-600 mr-2">"📈"</span>
                                                        "ROI attendu"
                                                    </h4>
                                                    <p class="text-green-700 font-medium">{roi} "% en moyenne sur 12 mois"</p>
                                                </div>
                                            }.into_view()
                                        } else {
                                            view! { <div></div> }.into_view()
                                        }}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </ContentSection>

            // Additional Services Section
            <ContentSection
                title="Services d'accompagnement".to_string()
                subtitle="Pour garantir le succès à long terme de votre transformation data".to_string()
                background="bg-gray-50".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12">
                    {additional_services.into_iter().map(|service| {
                        view! {
                            <div class="card-hover p-6">
                                <h3 class="text-2xl font-bold text-secondary-900 mb-3">
                                    {&service.name}
                                </h3>
                                <div class="text-2xl font-bold text-gradient-primary mb-3">
                                    {service.formatted_price()}
                                </div>
                                <p class="text-secondary-700 leading-relaxed mb-4">
                                    {&service.description}
                                </p>

                                {if !service.benefits.is_empty() {
                                    view! {
                                        <ul class="space-y-2 text-center">
                                            {service.benefits.iter().map(|benefit| {
                                                view! {
                                                    <li class="flex items-start justify-center text-sm">
                                                        <span class="text-primary-500 font-bold mr-2 mt-0.5">"•"</span>
                                                        <span class="text-secondary-600">{benefit}</span>
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }}
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </ContentSection>

            // Process Overview Section
            <ContentSection
                title="Comment nous procédons".to_string()
                background="bg-gradient-to-br from-gray-50 to-gray-100".to_string()
                centered=true
            >
                <div class="max-w-4xl mx-auto mt-12">
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
                        <div class="text-center group">
                            <div class="w-16 h-16 bg-gradient-to-br from-primary-500 to-primary-600 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg group-hover:scale-110 transition-transform duration-300">
                                <span class="text-white text-2xl font-bold">"1"</span>
                            </div>
                            <h3 class="text-lg font-bold text-secondary-900 mb-2">"Audit & Diagnostic"</h3>
                            <p class="text-secondary-600 text-sm leading-relaxed">"Analyse de votre écosystème data et identification des opportunités"</p>
                        </div>

                        <div class="text-center group">
                            <div class="w-16 h-16 bg-gradient-to-br from-secondary-500 to-secondary-600 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg group-hover:scale-110 transition-transform duration-300">
                                <span class="text-white text-2xl font-bold">"2"</span>
                            </div>
                            <h3 class="text-lg font-bold text-secondary-900 mb-2">"Preuve de concept"</h3>
                            <p class="text-secondary-600 text-sm leading-relaxed">"Développement d'un démonstrateur sur votre cas d'usage prioritaire"</p>
                        </div>

                        <div class="text-center group">
                            <div class="w-16 h-16 bg-gradient-to-br from-accent-500 to-accent-600 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg group-hover:scale-110 transition-transform duration-300">
                                <span class="text-white text-2xl font-bold">"3"</span>
                            </div>
                            <h3 class="text-lg font-bold text-secondary-900 mb-2">"Déploiement"</h3>
                            <p class="text-secondary-600 text-sm leading-relaxed">"Mise en production et formation de vos équipes"</p>
                        </div>

                        <div class="text-center group">
                            <div class="w-16 h-16 bg-gradient-to-br from-green-500 to-green-600 rounded-2xl flex items-center justify-center mx-auto mb-4 shadow-lg group-hover:scale-110 transition-transform duration-300">
                                <span class="text-white text-2xl font-bold">"4"</span>
                            </div>
                            <h3 class="text-lg font-bold text-secondary-900 mb-2">"Accompagnement"</h3>
                            <p class="text-secondary-600 text-sm leading-relaxed">"Support continu et évolutions selon vos besoins"</p>
                        </div>
                    </div>

                    <div class="text-center mt-12">
                        <a href="/process" class="btn-outline inline-flex items-center px-8 py-3 border-2 border-primary-500 text-primary-600 font-semibold rounded-xl hover:bg-primary-500 hover:text-white transition-all duration-300">
                            "Découvrir notre méthodologie complète"
                            <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                            </svg>
                        </a>
                    </div>
                </div>
            </ContentSection>

            // Final CTA
            <CtaSection
                title="Prêt à commencer votre transformation data ?".to_string()
                description="Discutons de vos besoins lors d'un audit gratuit de 30 minutes".to_string()
                cta_text="Planifier mon audit gratuit".to_string()
                cta_href="/contact".to_string()
                secondary_cta_text="Voir nos réalisations".to_string()
                secondary_cta_href="/case-studies".to_string()
                variant="gradient".to_string()
            />

            <Footer/>
        </div>
    }
}
