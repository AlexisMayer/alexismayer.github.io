//! Modern Header component with Tailwind CSS
//!
//! This component provides a sleek, responsive navigation bar with glass morphism effects,
//! smooth animations, and mobile-first design.

use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {
    let (is_mobile_open, set_mobile_open) = create_signal(false);

    view! {
        <header class="fixed top-0 left-0 right-0 z-50 glass backdrop-blur-lg border-b border-white/10">
            <nav class="container-soft">
                <div class="flex items-center justify-between h-16 lg:h-20">

                    // Brand
                    <div class="flex items-center space-x-1">
                        <A href="/" class="group flex items-center space-x-3 transition-transform duration-300 hover:scale-105">
                            <div class="flex items-center space-x-3">
                                <div class="w-10 h-10 bg-gradient-to-br from-primary-500 to-primary-600 rounded-xl flex items-center justify-center shadow-lg">
                                    <span class="text-white font-bold text-xl">"C"</span>
                                </div>
                                <div class="flex flex-col">
                                    <span class="text-xl lg:text-2xl font-bold text-gradient-primary">
                                    "CraftData"
                                </span>
                                <span class="text-xs lg:text-sm text-secondary-600 -mt-1 hidden sm:block">
                                    "Logiciel pour entreprise"
                                </span>
                                </div>
                            </div>
                        </A>
                    </div>

                    // Desktop Navigation
                    <div class="hidden lg:flex items-center space-x-1">
                        <NavLink href="/" exact=true>"Accueil"</NavLink>
                        <NavLink href="/about">"À propos"</NavLink>
                        <NavLink href="/services">"Services"</NavLink>
                        <NavLink href="/case-studies">"Réalisations"</NavLink>

                        // CTA Button
                        <div class="ml-4">
                            <A href="/contact" class="btn-primary group relative overflow-hidden">
                                <span class="relative z-10">"Audit gratuit"</span>
                                <div class="absolute inset-0 bg-gradient-to-r from-primary-600 to-primary-700 transform scale-x-0 group-hover:scale-x-100 transition-transform origin-left duration-300"></div>
                            </A>
                        </div>
                    </div>

                    // Mobile menu button
                    <button
                        class="lg:hidden p-2 rounded-xl hover:bg-white/10 transition-colors duration-200 focus-ring"
                        on:click=move |_| set_mobile_open.update(|open| *open = !*open)
                        aria-label="Toggle navigation"
                    >
                        <div class="w-6 h-6 flex flex-col justify-center items-center space-y-1">
                            <span class=move || format!("w-5 h-0.5 bg-secondary-700 transition-all duration-300 {}",
                                if is_mobile_open.get() { "rotate-45 translate-y-1.5" } else { "" })></span>
                            <span class=move || format!("w-5 h-0.5 bg-secondary-700 transition-all duration-300 {}",
                                if is_mobile_open.get() { "opacity-0" } else { "" })></span>
                            <span class=move || format!("w-5 h-0.5 bg-secondary-700 transition-all duration-300 {}",
                                if is_mobile_open.get() { "-rotate-45 -translate-y-1.5" } else { "" })></span>
                        </div>
                    </button>
                </div>

                // Mobile Navigation Menu
                <div class=move || format!("lg:hidden absolute top-full left-0 right-0 glass-dark backdrop-blur-xl border-b border-white/10 transition-all duration-300 {}",
                    if is_mobile_open.get() {
                        "opacity-100 visible translate-y-0"
                    } else {
                        "opacity-0 invisible -translate-y-4"
                    })>

                    <div class="container-soft py-6">
                        <nav class="flex flex-col space-y-1">
                            <MobileNavLink href="/" exact=true on_click=move |_| set_mobile_open.set(false)>
                                "Accueil"
                            </MobileNavLink>
                            <MobileNavLink href="/about" on_click=move |_| set_mobile_open.set(false)>
                                "À propos"
                            </MobileNavLink>
                            <MobileNavLink href="/services" on_click=move |_| set_mobile_open.set(false)>
                                "Services"
                            </MobileNavLink>
                            <MobileNavLink href="/case-studies" on_click=move |_| set_mobile_open.set(false)>
                                "Réalisations"
                            </MobileNavLink>

                            // Mobile CTA
                            <div class="pt-4 mt-4 border-t border-white/10">
                                <A
                                    href="/contact"
                                    class="btn-primary w-full text-center"
                                    on:click=move |_| set_mobile_open.set(false)
                                >
                                    "Audit gratuit"
                                </A>
                            </div>
                        </nav>
                    </div>
                </div>
            </nav>
        </header>

        // Spacer to account for fixed header
        <div class="h-16 lg:h-20"></div>
    }
}

#[component]
fn NavLink(
    href: &'static str,
    #[prop(default = false)] exact: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <A
            href=href
            exact=exact
            class="relative px-4 py-2 rounded-lg text-secondary-700 font-medium transition-all duration-200 hover:text-primary-600 hover:bg-white/5 group"
            active_class="text-primary-600 bg-white/10"
        >
            <span class="relative z-10">{children()}</span>
            // Animated underline
            <div class="absolute bottom-0 left-1/2 w-0 h-0.5 bg-gradient-to-r from-primary-500 to-primary-600 transition-all duration-300 group-hover:w-8 group-hover:left-1/2 group-hover:-translate-x-1/2"></div>
        </A>
    }
}

#[component]
fn MobileNavLink<F>(
    href: &'static str,
    #[prop(default = false)] exact: bool,
    #[prop(optional)] on_click: Option<F>,
    children: Children,
) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <A
            href=href
            exact=exact
            class="flex items-center px-4 py-3 rounded-xl text-white/90 font-medium transition-all duration-200 hover:text-white hover:bg-white/10 group"
            active_class="text-white bg-white/20"
            on:click=move |e| {
                if let Some(handler) = &on_click {
                    handler(e);
                }
            }
        >
            <span>{children()}</span>
            // Arrow icon
            <svg class="w-4 h-4 ml-auto opacity-60 group-hover:opacity-100 transition-opacity" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
            </svg>
        </A>
    }
}
