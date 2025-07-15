//! Company domain module
//!
//! This module contains the business logic and types related to company information,
//! contact management, and business characteristics for CraftData's target market.

/// Business sector options for forms
pub fn get_business_sector_options() -> Vec<(String, String)> {
    vec![
        (
            "manufacturing".to_string(),
            "Industrie/Manufacturing".to_string(),
        ),
        (
            "distribution".to_string(),
            "Distribution/Commerce".to_string(),
        ),
        ("services".to_string(), "Services B2B".to_string()),
        ("logistics".to_string(), "Logistique/Transport".to_string()),
        ("construction".to_string(), "BTP/Construction".to_string()),
        ("other".to_string(), "Autre".to_string()),
    ]
}

/// Data challenge options for forms
pub fn get_data_challenge_options() -> Vec<(String, String)> {
    vec![
        (
            "data-scattered".to_string(),
            "Données dispersées (Excel, ERP, CRM...)".to_string(),
        ),
        (
            "no-visibility".to_string(),
            "Manque de visibilité sur les KPIs".to_string(),
        ),
        (
            "manual-reports".to_string(),
            "Rapports manuels chronophages".to_string(),
        ),
        (
            "inventory".to_string(),
            "Gestion des stocks/approvisionnements".to_string(),
        ),
        (
            "quality".to_string(),
            "Contrôle qualité/défauts".to_string(),
        ),
        (
            "customer-analysis".to_string(),
            "Analyse comportement clients".to_string(),
        ),
        (
            "predictive".to_string(),
            "Besoin de prédictions (demande, pannes...)".to_string(),
        ),
        (
            "automation".to_string(),
            "Automatisation de processus".to_string(),
        ),
        ("other".to_string(), "Autre défi".to_string()),
    ]
}

/// Project timeline options for forms
pub fn get_project_timeline_options() -> Vec<(String, String)> {
    vec![
        ("asap".to_string(), "Dès que possible".to_string()),
        ("1-3months".to_string(), "Dans 1-3 mois".to_string()),
        ("3-6months".to_string(), "Dans 3-6 mois".to_string()),
        ("6months+".to_string(), "Plus de 6 mois".to_string()),
        ("exploring".to_string(), "Phase d'exploration".to_string()),
    ]
}

/// Alexis Mayer - Founder information
#[derive(Debug, Clone)]
pub struct Founder {
    pub name: &'static str,
    pub title: &'static str,
    pub education: &'static str,
    pub experience_years: u8,
    pub philosophy: &'static str,
}

impl Founder {
    pub fn alexis_mayer() -> Self {
        Self {
            name: "Alexis Mayer",
            title: "Data spécialiste & Fondateur",
            education: "Toulouse School of Economics",
            experience_years: 7,
            philosophy: "La data et l'IA ne doivent pas être réservées aux GAFAM. \
                        Chaque PME dispose de données précieuses qu'elle peut exploiter \
                        pour gagner en compétitivité.",
        }
    }
}

/// CraftData company information
#[derive(Debug, Clone)]
pub struct CraftDataCompany {
    pub mission: &'static str,
    pub founded_year: u16,
    pub legal_form: &'static str,
    pub target_region: &'static str,
    pub founder: Founder,
}

impl CraftDataCompany {
    pub fn get_info() -> Self {
        Self {
            mission: "Rendre la data et l'IA accessibles et utiles aux PME, \
                     en particulier en région Centre-Val de Loire",
            founded_year: 2024,
            legal_form: "SASU",
            target_region: "Centre-Val de Loire",
            founder: Founder::alexis_mayer(),
        }
    }
}

