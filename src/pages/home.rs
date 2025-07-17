//! Home page component
//!
//! This is the main landing page for CraftData, showcasing the value proposition,
//! services overview, social proof, and call-to-actions. It uses domain entities
//! and modern UI components with Tailwind CSS styling.

use crate::domain::{ServiceOffering, CraftDataCompany};
use crate::ui::components::{
    ContentSection, CtaSection, HeroSection, StatItem, StatsSection,
};
use crate::ui::{Footer, Header};
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // Get domain data
    let services = ServiceOffering::all_main_services();
    let company_info = CraftDataCompany::get_info();

    view! {
        <div class="min-h-screen bg-white">
            <Header/>

            // Hero Section with value proposition
            <HeroSection
                title="Transformez vos données en avantage concurrentiel".to_string()
                subtitle="CraftData accompagne les entreprises dans leur transformation Data et IA".to_string()
                // description=company_info.mission.to_string()
                cta_text="Audit gratuit (30 min)".to_string()
                cta_href="/contact".to_string()
                secondary_cta_text="Voir nos services".to_string()
                secondary_cta_href="/services".to_string()
                centered=true
                // background_variant="animated-neural-network".to_string()
            >
                // Hero stats
                <div></div>
            </HeroSection>

            // Value Proposition - Problems/Solutions
            <ContentSection
              title="Vos défis Data & IA, nos solutions".to_string()
              background="bg-gray-50".to_string()
              centered={true}
            >
              <div class="grid grid-cols-1 lg:grid-cols-3 gap-8 mt-12">

                {/* Bloc 1 */}
                <div class="card-hover p-6 bg-white">
                  <div class="mb-6">
                    <div class="flex items-center mb-3">
                      <span class="text-2xl mr-3">"❌"</span>
                      <h3 class="text-xl font-bold text-red-600">Vos données sont dispersées</h3>
                    </div>
                    <p class="text-secondary-600">
                      Excel, ERP, CRM... impossible d/'avoir une vision globale
                    </p>
                  </div>
                  <div class="border-t pt-6">
                    <div class="flex items-center mb-3">
                      <span class="text-2xl mr-3">"✅"</span>
                      <h3 class="text-xl font-bold text-green-600">Centralisation intelligente</h3>
                    </div>
                    <p class="text-secondary-600">
                      Dashboard unifié connecté à toutes vos sources
                    </p>
                  </div>
                </div>

                {/* Bloc 2 */}
                <div class="card-hover p-6 bg-white">
                  <div class="mb-6">
                    <div class="flex items-center mb-3">
                      <span class="text-2xl mr-3">"❌"</span>
                      <h3 class="text-xl font-bold text-red-600">"L'IA semble complexe et inaccessible"</h3>
                    </div>
                    <p class="text-secondary-600">
                      "Difficile de savoir par où commencer et comment en tirer des bénéfices concrets."
                    </p>
                  </div>
                  <div class="border-t pt-6">
                    <div class="flex items-center mb-3">
                      <span class="text-2xl mr-3">"✅"</span>
                      <h3 class="text-xl font-bold text-green-600">IA sur-mesure, simple à adopter</h3>
                    </div>
                    <p class="text-secondary-600">
                      Nous concevons des outils adaptés à vos besoins, sans complexité technique.
                    </p>
                  </div>
                </div>

                {/* Bloc 3 */}
                <div class="card-hover p-6 bg-white">
                  <div class="mb-6">
                    <div class="flex items-center mb-3">
                      <span class="text-2xl mr-3">"❌"</span>
                      <h3 class="text-xl font-bold text-red-600">Manque de temps et de ressources internes</h3>
                    </div>
                    <p class="text-secondary-600">
                      "Vos équipes sont déjà mobilisées ; il vous faut des résultats rapides sans surcharge."
                    </p>
                  </div>
                  <div class="border-t pt-6">
                    <div class="flex items-center mb-3">
                      <span class="text-2xl mr-3">"✅"</span>
                      <h3 class="text-xl font-bold text-green-600">Accompagnement clé en main</h3>
                    </div>
                    <p class="text-secondary-600">
                      Nous gérons le projet de bout en bout, pour un impact rapide et mesurable.
                    </p>
                  </div>
                </div>

              </div>
            </ContentSection>

            // Services Preview with domain data
            <ContentSection
                title="Des offres modulables et adaptées à vos besoins".to_string()
                centered=true
            >
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mt-12">
                    {services.into_iter().map(|service| {
                        let is_exploration = service.service_type == crate::domain::ServiceType::Exploration;

                        view! {
                            <div class=format!("relative p-8 rounded-2xl border-2 transition-all duration-300 hover:shadow-xl hover:-translate-y-1 {}",
                                if is_exploration {
                                    "border-primary-200 bg-gradient-to-br from-primary-50 to-primary-100 ring-2 ring-primary-200"
                                } else {
                                    "border-gray-200 bg-white hover:border-primary-200"
                                })>

                                {if is_exploration {
                                    view! {
                                        <div class="absolute -top-3 left-6 px-4 py-1 bg-gradient-to-r from-primary-500 to-primary-600 text-white text-sm font-semibold rounded-full">
                                            "⚡ Démarrage rapide"
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class="absolute -top-3 left-6 px-4 py-1 bg-gradient-to-r from-secondary-500 to-secondary-600 text-white text-sm font-semibold rounded-full">
                                            "🚀 Solution complète"
                                        </div>
                                    }.into_view()
                                }}

                                <div class="mt-4">
                                    <h3 class="text-3xl font-bold text-secondary-900 mb-4">{&service.name}</h3>

                                    <div class="flex items-baseline justify-center mb-2">
                                        <span class="text-3xl font-bold text-gradient-primary">{service.formatted_price()}</span>
                                    </div>
                                    <div class="text-secondary-600 mb-6">{service.formatted_duration()}</div>

                                    <div class="mb-6">
                                        <h4 class="font-semibold text-secondary-800 mb-3">"Ce que vous obtenez :"</h4>
                                        <ul class="space-y-2">
                                            {service.benefits.into_iter().map(|benefit| {
                                                view! {
                                                    <li class="flex justify-center items-start">
                                                        <span class="text-green-500 font-bold mr-2 mt-0.5">"✓"</span>
                                                        <span class="text-secondary-700">{benefit}</span>
                                                    </li>
                                                }
                                            }).collect::<Vec<_>>()}
                                        </ul>
                                    </div>

                                    {if let Some(guarantee) = service.guarantee {
                                        view! {
                                            <div class="flex items-center p-3 bg-yellow-50 border border-yellow-200 rounded-lg mb-6">
                                                <span class="text-yellow-600 mr-2">"💰"</span>
                                                <span class="text-yellow-800 font-medium">"Garantie : " {guarantee}</span>
                                            </div>
                                        }.into_view()
                                    } else {
                                        view! {
                                            <div class="flex items-center p-3 bg-green-50 border border-green-200 rounded-lg mb-6">
                                                <span class="text-green-600 mr-2">"📈"</span>
                                                <span class="text-green-800 font-medium">"ROI moyen : " {service.roi_percentage.unwrap_or(300)} "% en 12 mois"</span>
                                            </div>
                                        }.into_view()
                                    }}

                                    <a href="/contact" class=format!("block w-full text-center py-3 px-6 rounded-xl font-semibold transition-all duration-300 {}",
                                        if is_exploration {
                                            "bg-gradient-to-r from-primary-500 to-primary-600 text-white hover:shadow-lg hover:scale-105"
                                        } else {
                                            "border-2 border-primary-500 text-primary-600 hover:bg-primary-500 hover:text-white"
                                        })>
                                        {if is_exploration { "Commencer l'audit" } else { "Discuter du projet" }}
                                    </a>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </ContentSection>

            // Social Proof with domain testimonials
            //<ContentSection
            //    title="Ils nous font confiance".to_string()
            //    background="bg-gray-50".to_string()
            //    centered=true
            //>
            //    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mt-12">
            //        {featured_cases.into_iter().filter_map(|case| {
            //            case.testimonial.map(|testimonial| {
            //                view! {
            //                    <TestimonialCard
            //                        quote=testimonial
            //                        author=case.client_name.clone()
            //                        company=format!("{} employés", case.client_size_employees)
            //                    />
            //                }
            //            })
            //        }).collect::<Vec<_>>()}
            //    </div>
            //</ContentSection>

            // Target Market Section
            // <ContentSection
            //     title=format!("Nous accompagnons les PME du {}", company_info.target_region)
            //     centered=true
            // >
            //     <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mt-12">
            //         <FeatureCard
            //             title="🏭 Secteurs industriels".to_string()
            //             description="Manufacturing, métallurgie, agroalimentaire, chimie".to_string()
            //         />
            //         <FeatureCard
            //             title="🚚 Distribution & Logistique".to_string()
            //             description="Commerce B2B, grossistes, transporteurs".to_string()
            //         />
            //         <FeatureCard
            //             title="⚙️ Services B2B".to_string()
            //             description="Conseil, ingénierie, services techniques".to_string()
            //         />
            //         <FeatureCard
            //             title="📊 Taille optimale".to_string()
            //             description="PME de 10 à 250 salariés".to_string()
            //             highlighted=true
            //         />
            //     </div>
            // </ContentSection>

            // Market Context
            <StatsSection
                title="Un marché en pleine transformation".to_string()
                subtitle="Source : Étude Bpifrance sur la transformation digitale des PME françaises".to_string()
            >
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <StatItem
                        value="+150 000".to_string()
                        label="PME concernées en France".to_string()
                        icon="🏢".to_string()
                    />
                    <StatItem
                        value="20%".to_string()
                        label="Croissance annuelle du marché data/IA".to_string()
                        icon="📈".to_string()
                    />
                    <StatItem
                        value="5 000".to_string()
                        label=format!("Entreprises B2B en {}", company_info.target_region)
                        icon="🎯".to_string()
                    />
                </div>
            </StatsSection>

            // Final CTA
            <CtaSection
                title="Prêt à transformer vos données en profits ?".to_string()
                description="Audit gratuit de 30 minutes pour identifier votre potentiel data/IA".to_string()
                cta_text="Réserver mon audit gratuit".to_string()
                cta_href="/contact".to_string()
                variant="gradient".to_string()
            />

            // Additional info section
            <div class="bg-white py-8">
                <div class="container-soft">
                    <p class="text-center text-secondary-600 text-sm">
                        "Sans engagement • Réponse sous 24h • Déplacement en région Centre"
                    </p>
                </div>
            </div>

            <Footer/>
        </div>
    }
}
