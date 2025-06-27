//! Case Studies domain module
//!
//! This module contains the business logic and types related to SoftIA's case studies.
//! It demonstrates real business value and ROI from data/IA implementations.

/// Types of solutions provided by SoftIA
#[derive(Debug, Clone, PartialEq)]
pub enum SolutionType {
    ProcessAutomation,
}

impl SolutionType {
    pub fn get_label(&self) -> &'static str {
        match self {
            Self::ProcessAutomation => "Automatisation de processus",
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match self {
            Self::ProcessAutomation => "🤖",
        }
    }
}

/// Business metrics achieved
#[derive(Debug, Clone)]
pub struct BusinessMetric {
    pub name: String,
    pub improvement_percentage: f64,
    pub description: String,
}

impl BusinessMetric {
    pub fn new(metric_name: &str, improvement_percentage: f64, description: &str) -> Self {
        Self {
            name: metric_name.to_string(),
            improvement_percentage,
            description: description.to_string(),
        }
    }

    pub fn format_improvement(&self) -> String {
        if self.improvement_percentage > 0.0 {
            format!("+{:.0}%", self.improvement_percentage)
        } else {
            format!("{:.0}%", self.improvement_percentage)
        }
    }
}

/// ROI metrics for financial impact
#[derive(Debug, Clone)]
pub struct RoiMetrics {
    pub investment_euros: u32,
    pub annual_savings_euros: u32,
    pub payback_months: u8,
    pub roi_percentage_12months: u16,
}

impl RoiMetrics {
    pub fn format_investment(&self) -> String {
        format!("{} k€", self.investment_euros / 1000)
    }

    pub fn format_annual_savings(&self) -> String {
        format!("{} k€/an", self.annual_savings_euros / 1000)
    }
}

/// Portfolio statistics
#[derive(Debug, Clone)]
pub struct PortfolioStats {
    pub total_clients: u16,
    pub average_roi_percentage: u16,
    pub average_payback_months: u16,
    pub average_implementation_months: u8,
}

impl PortfolioStats {
    pub fn format_average_roi(&self) -> String {
        format!("{}%", self.average_roi_percentage)
    }

    pub fn format_payback(&self) -> String {
        format!("< {} mois", self.average_payback_months)
    }
}

/// Main case study entity
#[derive(Debug, Clone)]
pub struct CaseStudy {
    pub title: String,
    pub client_name: String,
    pub client_size_employees: u16,
    pub solution_type: SolutionType,
    pub challenge_description: String,
    pub solution_description: String,
    pub business_metrics: Vec<BusinessMetric>,
    pub roi_metrics: RoiMetrics,
    pub testimonial: Option<String>,
    pub is_featured: bool,
    pub completion_date: String,
    pub technologies_used: Vec<String>,
}

impl CaseStudy {
    pub fn formatted_duration(&self) -> String {
        "3-4 mois".to_string()
    }
}

impl CaseStudy {
    /// Create the Métallurgie Loire case study
    pub fn metallurgie_loire() -> Self {
        let metrics = vec![
            BusinessMetric::new(
                "Arrêts non planifiés",
                -40.0,
                "Réduction des arrêts de production imprévus",
            ),
            BusinessMetric::new(
                "Coûts de maintenance",
                -25.0,
                "Optimisation des interventions préventives",
            ),
            BusinessMetric::new(
                "Productivité équipements",
                15.0,
                "Amélioration du taux de rendement synthétique",
            ),
        ];

        let roi = RoiMetrics {
            investment_euros: 15000,
            annual_savings_euros: 80000,
            payback_months: 3,
            roi_percentage_12months: 433,
        };

        Self {
            title: "Prédiction de pannes pour optimiser la maintenance".to_string(),
            client_name: "Métallurgie Loire".to_string(),
            client_size_employees: 85,
            solution_type: SolutionType::ProcessAutomation,
            challenge_description: "Arrêts production imprévus coûteux et maintenance réactive inefficace".to_string(),
            solution_description: "Système de maintenance prédictive basé sur l'IA analysant vibrations et températures".to_string(),
            business_metrics: metrics,
            roi_metrics: roi,
            testimonial: Some("Grâce à SoftIA, nous avons divisé par deux nos arrêts non planifiés. Le ROI a été au rendez-vous dès le 3ème mois.".to_string()),
            is_featured: true,
            completion_date: "Octobre 2024".to_string(),
            technologies_used: vec![
                "Dashboard temps réel (React/D3.js)".to_string(),
                "Algorithmes ML (Python/scikit-learn)".to_string(),
                "Base de données InfluxDB".to_string(),
                "API d'intégration FastAPI".to_string(),
            ],
        }
    }

    /// Create the Logistique Centre case study
    pub fn logistique_centre() -> Self {
        let metrics = vec![
            BusinessMetric::new(
                "Coûts de transport",
                -20.0,
                "Optimisation des tournées de livraison",
            ),
            BusinessMetric::new(
                "Satisfaction client",
                30.0,
                "Réduction des retards de livraison",
            ),
            BusinessMetric::new(
                "Temps de planification",
                -60.0,
                "Automatisation de la planification des tournées",
            ),
        ];

        let roi = RoiMetrics {
            investment_euros: 4000,
            annual_savings_euros: 45000,
            payback_months: 2,
            roi_percentage_12months: 1025,
        };

        Self {
            title: "Optimisation des tournées de livraison".to_string(),
            client_name: "Logistique Centre".to_string(),
            client_size_employees: 45,
            solution_type: SolutionType::ProcessAutomation,
            challenge_description: "Planification manuelle des tournées chronophage et coûteuse".to_string(),
            solution_description: "Algorithme d'optimisation intégré au système de gestion pour planification automatique".to_string(),
            business_metrics: metrics,
            roi_metrics: roi,
            testimonial: Some("L'outil développé par SoftIA nous fait économiser 2h de planification par jour et 20% sur nos coûts de transport.".to_string()),
            is_featured: true,
            completion_date: "Septembre 2024".to_string(),
            technologies_used: vec![
                "Algorithme d'optimisation OR-Tools".to_string(),
                "API de géolocalisation".to_string(),
                "Interface web (Vue.js)".to_string(),
                "Intégration ERP".to_string(),
            ],
        }
    }

    /// Get all case studies
    pub fn get_all_case_studies() -> Vec<Self> {
        vec![Self::metallurgie_loire(), Self::logistique_centre()]
    }

    /// Get featured case studies for home page
    pub fn get_featured_case_studies() -> Vec<Self> {
        vec![Self::metallurgie_loire(), Self::logistique_centre()]
    }

    /// Calculate average ROI across all case studies
    pub fn calculate_average_roi() -> u16 {
        let cases = Self::get_all_case_studies();
        let total_roi: u16 = cases
            .iter()
            .map(|c| c.roi_metrics.roi_percentage_12months)
            .sum();
        total_roi / cases.len() as u16
    }
}

/// Business intelligence for case studies
pub struct CaseStudyAnalytics;

impl CaseStudyAnalytics {
    /// Get key statistics across all case studies
    pub fn get_portfolio_stats() -> PortfolioStats {
        let cases = CaseStudy::get_all_case_studies();

        let avg_roi = CaseStudy::calculate_average_roi();
        let avg_payback = cases
            .iter()
            .map(|c| c.roi_metrics.payback_months as u16)
            .sum::<u16>()
            / cases.len() as u16;

        let total_clients = cases.len() as u16;

        PortfolioStats {
            total_clients,
            average_roi_percentage: avg_roi,
            average_payback_months: avg_payback,
            average_implementation_months: 3,
        }
    }
}
