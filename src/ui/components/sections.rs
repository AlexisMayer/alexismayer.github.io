//! Modern UI Section Components
//!
//! This module provides reusable section components with modern Tailwind styling,
//! including hero sections, content layouts, and interactive elements.

use leptos::*;
use crate::ui::components::neural_network_animation::NeuralNetworkAnimation;

/// Hero section with gradient background and modern layout
#[component]
pub fn HeroSection(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] subtitle: Option<String>,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] cta_text: Option<String>,
    #[prop(optional)] cta_href: Option<String>,
    #[prop(optional)] secondary_cta_text: Option<String>,
    #[prop(optional)] secondary_cta_href: Option<String>,
    #[prop(default = false)] centered: bool,
    #[prop(optional)] background_variant: Option<String>,
    children: Children,
) -> impl IntoView {
    let bg_class = background_variant.unwrap_or_else(|| "bg-secondary-950".to_string());

    view! {
        <section class=format!("relative overflow-hidden {}", bg_class)>
            // Animated SVG Background will be managed by a dedicated component
            <div class="absolute inset-0 w-full h-full z-0">
                <NeuralNetworkAnimation />
            </div>

            <div class="relative container-soft section-py z-10">
                <div class=format!("max-w-5xl {}", if centered { "mx-auto text-center" } else { "" })>
                    // Title
                    {title.map(|t| view! {
                        <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold text-white mb-6 animate-fade-in-up">
                            {t}
                        </h1>
                    })}

                    // Subtitle
                    {subtitle.map(|s| view! {
                        <p class="text-xl md:text-3xl text-white/90 font-medium mb-6 animate-fade-in-up" style="animation-delay: 0.1s">
                            {s}
                        </p>
                    })}

                    // Description
                    {description.map(|d| view! {
                        <p class="text-lg text-white/80 mb-8 max-w-2xl animate-fade-in-up" class=if centered { "mx-auto" } else { "" } style="animation-delay: 0.2s">
                            {d}
                        </p>
                    })}

                    // CTA buttons
                    <div class=format!("flex flex-col sm:flex-row gap-4 animate-fade-in-up {}", if centered { "justify-center" } else { "" }) style="animation-delay: 0.3s">
                        {cta_text.zip(cta_href).map(|(text, href)| view! {
                            <a href=href class="btn-primary group relative overflow-hidden">
                                <span class="relative z-10">{text}</span>
                                <div class="absolute inset-0 bg-gradient-to-r from-white/20 to-white/30 transform scale-x-0 group-hover:scale-x-100 transition-transform origin-left duration-300"></div>
                            </a>
                        })}

                        {secondary_cta_text.zip(secondary_cta_href).map(|(text, href)| view! {
                            <a href=href class="inline-flex items-center px-6 py-3 border-2 border-white/30 text-white font-semibold rounded-xl hover:bg-white/10 transition-all duration-300">
                                {text}
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                                </svg>
                            </a>
                        })}
                    </div>

                    // Custom content
                    <div class="mt-8 animate-fade-in-up" style="animation-delay: 0.4s">
                        {children()}
                    </div>
                </div>
            </div>

            // Scroll indicator
            <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce-gentle">
                <div class="w-6 h-10 border-2 border-white/30 rounded-full flex justify-center">
                    <div class="w-1 h-3 bg-white/60 rounded-full mt-2 animate-pulse"></div>
                </div>
            </div>
        </section>
    }
}

/// Content section with flexible layout options
#[component]
pub fn ContentSection(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] subtitle: Option<String>,
    #[prop(optional)] background: Option<String>,
    #[prop(default = false)] centered: bool,
    #[prop(default = "lg".to_string())] size: String,
    children: Children,
) -> impl IntoView {
    let bg_class = background.unwrap_or_else(|| "bg-white".to_string());
    let container_class = match size.as_str() {
        "sm" => "container-narrow",
        "lg" => "container-soft",
        _ => "container-soft",
    };

    view! {
        <section class=format!("relative {}", bg_class)>
            <div class=format!("{} section-py-sm", container_class)>
                <div class=format!("max-w-5xl {}", if centered { "mx-auto text-center" } else { "" })>
                    {title.map(|t| view! {
                        <h2 class="text-3xl md:text-4xl font-bold text-secondary-900 mb-4 animate-fade-in-up">
                            {t}
                        </h2>
                    })}

                    {subtitle.map(|s| view! {
                        <p class="text-xl text-secondary-600 mb-8 animate-fade-in-up" style="animation-delay: 0.1s">
                            {s}
                        </p>
                    })}

                    <div class="animate-fade-in-up" style="animation-delay: 0.2s">
                        {children()}
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Stats section with animated counters
#[component]
pub fn StatsSection(
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] subtitle: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="relative bg-gradient-to-br from-primary-50 to-primary-100 overflow-hidden">
            <div class="absolute inset-0 bg-hero-pattern opacity-5"></div>

            <div class="relative container-soft section-py-sm">
                {title.map(|t| view! {
                    <div class="text-center mb-12">
                        <h2 class="text-3xl md:text-4xl font-bold text-secondary-900 mb-4 animate-fade-in-up">
                            {t}
                        </h2>
                        {subtitle.map(|s| view! {
                            <p class="text-xl text-secondary-600 animate-fade-in-up" style="animation-delay: 0.1s">
                                {s}
                            </p>
                        })}
                    </div>
                })}

                <div class="animate-fade-in-up" style="animation-delay: 0.2s">
                    {children()}
                </div>
            </div>
        </section>
    }
}

/// Single stat item component
#[component]
pub fn StatItem(
    value: String,
    label: String,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] icon: Option<String>,
) -> impl IntoView {
    view! {
        <div class="text-center group">
            {icon.map(|i| view! {
                <div class="w-12 h-12 bg-gradient-to-br from-primary-500 to-primary-600 rounded-2xl flex items-center justify-center mx-auto mb-4 group-hover:scale-110 transition-transform duration-300">
                    <span class="text-2xl">{i}</span>
                </div>
            })}

            <div class="text-4xl md:text-5xl font-bold text-gradient-primary mb-2 group-hover:scale-105 transition-transform duration-300">
                {value}
            </div>
            <div class="text-lg font-semibold text-secondary-800 mb-1">
                {label}
            </div>
            {description.map(|d| view! {
                <div class="text-secondary-600 text-sm">
                    {d}
                </div>
            })}
        </div>
    }
}

/// Feature card component
#[component]
pub fn FeatureCard(
    title: String,
    description: String,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] href: Option<String>,
    #[prop(default = false)] highlighted: bool,
) -> impl IntoView {
    let card_class = if highlighted {
        "card-hover bg-gradient-to-br from-primary-50 to-primary-100 border-primary-200"
    } else {
        "card-hover"
    };

    let content = view! {
        <div class=format!("p-6 h-full flex flex-col {}", card_class)>
            {icon.map(|i| view! {
                <div class="w-12 h-12 bg-gradient-to-br from-primary-500 to-primary-600 rounded-xl flex items-center justify-center mb-4 shadow-lg">
                    <span class="text-2xl">{i}</span>
                </div>
            })}

            <h3 class="text-xl font-bold text-secondary-900 mb-3">
                {title}
            </h3>
            <p class="text-secondary-600 flex-grow">
                {description}
            </p>

            {href.is_some().then(|| view! {
                <div class="mt-4 pt-4 border-t border-secondary-200">
                    <span class="text-primary-600 font-medium text-sm flex items-center group-hover:text-primary-700">
                        "En savoir plus"
                        <svg class="w-4 h-4 ml-1 transform group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                        </svg>
                    </span>
                </div>
            })}
        </div>
    };

    if let Some(link) = href {
        view! {
            <a href=link class="block group">
                {content}
            </a>
        }
        .into_view()
    } else {
        content.into_view()
    }
}

/// Testimonial card component
#[component]
pub fn TestimonialCard(
    quote: String,
    author: String,
    #[prop(optional)] company: Option<String>,
    #[prop(optional)] avatar: Option<String>,
) -> impl IntoView {
    view! {
        <div class="card-hover p-6 relative">
            // Quote icon
            <div class="absolute top-4 left-4 text-primary-200">
                <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M14.017 21v-7.391c0-5.704 3.731-9.57 8.983-10.609l.995 2.151c-2.432.917-3.995 3.638-3.995 5.849h4v10h-9.983zm-14.017 0v-7.391c0-5.704 3.748-9.57 9-10.609l.996 2.151c-2.433.917-3.996 3.638-3.996 5.849h4v10h-10z"/>
                </svg>
            </div>

            <blockquote class="text-secondary-700 italic text-lg leading-relaxed mb-6 pt-8">
                {format!("\"{quote}\"")}
            </blockquote>

            <div class="flex items-center">
                {if let Some(a) = avatar {
                    view! {
                        <img src=a alt=format!("Photo de {}", author) class="w-12 h-12 rounded-full mr-4 object-cover"/>
                    }.into_view()
                } else {
                    view! {
                        <div class="w-12 h-12 bg-gradient-to-br from-primary-500 to-primary-600 rounded-full flex items-center justify-center mr-4">
                            <span class="text-white font-semibold">{author.chars().next().unwrap_or('?')}</span>
                        </div>
                    }.into_view()
                }}

                <div>
                    <div class="font-semibold text-secondary-900">{author}</div>
                    {company.map(|c| view! {
                        <div class="text-secondary-600 text-sm">{c}</div>
                    })}
                </div>
            </div>
        </div>
    }
}

/// CTA (Call-to-Action) section
#[component]
pub fn CtaSection(
    title: String,
    description: String,
    cta_text: String,
    cta_href: String,
    #[prop(optional)] secondary_cta_text: Option<String>,
    #[prop(optional)] secondary_cta_href: Option<String>,
    #[prop(default = "primary".to_string())] variant: String,
) -> impl IntoView {
    let bg_class = match variant.as_str() {
        "gradient" => "bg-gradient-to-br from-primary-600 to-primary-700",
        "dark" => "bg-secondary-900",
        _ => "bg-primary-600",
    };

    view! {
        <section class=format!("relative {} overflow-hidden", bg_class)>
            <div class="absolute inset-0 bg-hero-pattern opacity-10"></div>

            <div class="relative container-soft py-16 lg:py-20">
                <div class="max-w-3xl mx-auto text-center">
                    <h2 class="text-3xl md:text-4xl font-bold text-white mb-4 animate-fade-in-up">
                        {title}
                    </h2>
                    <p class="text-xl text-white/90 mb-8 animate-fade-in-up" style="animation-delay: 0.1s">
                        {description}
                    </p>

                    <div class="flex flex-col sm:flex-row gap-4 justify-center animate-fade-in-up" style="animation-delay: 0.2s">
                        <a href=cta_href class="btn-primary bg-white text-primary-600 hover:bg-primary-50 shadow-xl">
                            {cta_text}
                        </a>

                        {secondary_cta_text.zip(secondary_cta_href).map(|(text, href)| view! {
                            <a href=href class="inline-flex items-center px-6 py-3 border-2 border-white/30 text-white font-semibold rounded-xl hover:bg-white/10 transition-all duration-300">
                                {text}
                                <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                                </svg>
                            </a>
                        })}
                    </div>
                </div>
            </div>
        </section>
    }
}
