//! Case Studies domain module
//!
//! This module contains the business logic and types related to CraftData's case studies.
//! It demonstrates real business value and ROI from data/IA implementations.

/// Types of solutions provided by CraftData
#[derive(Debug, Clone, PartialEq)]
pub enum SolutionType {
    ProcessAutomation,
    AiAssistant,
}

impl SolutionType {
    pub fn get_label(&self) -> &'static str {
        match self {
            Self::ProcessAutomation => "Automatisation de processus",
            Self::AiAssistant => "Assistant IA",
        }
    }

    pub fn get_icon(&self) -> &'static str {
        match self {
            Self::ProcessAutomation => "🤖",
            Self::AiAssistant => "🧠",
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

    /// Create SmartDocs case study
    pub fn smartdocs() -> CaseStudy {
        let metrics = vec![
            BusinessMetric::new(
                "Temps de recherche d’information",
                -70.0,
                "Réduction du temps passé à chercher dans les dossiers, procédures et documents internes",
            ),
            BusinessMetric::new(
                "Réactivité des équipes",
                35.0,
                "Amélioration de la vitesse de réponse aux demandes internes et externes",
            ),
            BusinessMetric::new(
                "Capitalisation du savoir",
                60.0,
                "Meilleure valorisation des connaissances contenues dans les fichiers historiques",
            ),
        ];

        let roi = RoiMetrics {
            investment_euros: 4000,
            annual_savings_euros: 45000,
            payback_months: 1,
            roi_percentage_12months: 1025,
        };

        Self {
            title: "SmartDocs – L’assistant IA universel pour vos fichiers internes".to_string(),
            client_name: "Entreprise multisite – secteur services".to_string(),
            client_size_employees: 80,
            solution_type: SolutionType::AiAssistant,
            challenge_description: "Les collaborateurs avaient du mal à retrouver et exploiter l’information dispersée dans des milliers de fichiers (PDF, Word, Excel, HTML…).".to_string(),
            solution_description: "Déploiement d’un assistant IA interrogeable en langage naturel via une interface web interne. Il analyse et synthétise le contenu des fichiers stockés dans l’arborescence de l’entreprise.".to_string(),
            business_metrics: metrics,
            roi_metrics: roi,
            testimonial: Some("SmartDocs a apporté une fluidité incroyable dans l’accès aux informations internes. Même les nouveaux arrivants deviennent opérationnels en quelques jours.".to_string()),
            is_featured: true,
            completion_date: "Juillet 2025".to_string(),
            technologies_used: vec![
                "Modèles de langage (LLM)".to_string(),
                "RAG (Retrieval Augmented Generation)".to_string(),
                "Base vectorielle (FAISS / Qdrant)".to_string(),
                "FastAPI + Gradio (interface interne)".to_string(),
                "LangChain ou LlamaIndex".to_string(),
            ],
        }
    }

    /// Create the IndusIA case study
    pub fn indusia() -> Self {
        let metrics = vec![
            BusinessMetric::new(
                "Temps de recherche doc",
                -70.0,
                "Réduction du temps passé à chercher des informations",
            ),
            BusinessMetric::new(
                "Conformité normes",
                25.0,
                "Amélioration de l'adhésion aux normes réglementaires",
            ),
            BusinessMetric::new(
                "Capitalisation savoirs",
                50.0,
                "Augmentation de la transmission des connaissances internes",
            ),
        ];

        let roi = RoiMetrics {
            investment_euros: 5000,
            annual_savings_euros: 60000,
            payback_months: 1,
            roi_percentage_12months: 1100,
        };

        Self {
            title: "IndusIA – L'assistant IA pour les équipes industrielles".to_string(),
            client_name: "PME Industrielle".to_string(),
            client_size_employees: 120,
            solution_type: SolutionType::AiAssistant,
            challenge_description: "Accès lent et fragmenté à la documentation technique et aux normes".to_string(),
            solution_description: "Assistant IA en langage naturel pour accès instantané à l'information".to_string(),
            business_metrics: metrics,
            roi_metrics: roi,
            testimonial: Some("IndusIA a révolutionné notre accès à l'information. Nos équipes gagnent un temps précieux chaque jour.".to_string()),
            is_featured: true,
            completion_date: "Juin 2025".to_string(),
            technologies_used: vec![
                "Modèles de langage (LLM)".to_string(),
                "Traitement du langage naturel (NLP)".to_string(),
                "Base de données vectorielle".to_string(),
                "API d'intégration".to_string(),
            ],
        }
    }

    /// Create the AgriConseil IA case study
    pub fn agriconseil_ia() -> Self {
        let metrics = vec![
            BusinessMetric::new(
                "Temps de veille réglementaire",
                -80.0,
                "Réduction du temps passé à la veille réglementaire",
            ),
            BusinessMetric::new(
                "Réactivité décisions",
                30.0,
                "Amélioration de la réactivité face aux changements",
            ),
            BusinessMetric::new(
                "Conformité PAC",
                20.0,
                "Augmentation de la conformité aux aides PAC",
            ),
        ];

        let roi = RoiMetrics {
            investment_euros: 3000,
            annual_savings_euros: 40000,
            payback_months: 1,
            roi_percentage_12months: 1200,
        };

        Self {
            title: "AgriConseil IA - Assistant Agricole Intelligent".to_string(),
            client_name: "Exploitation Agricole".to_string(),
            client_size_employees: 5,
            solution_type: SolutionType::AiAssistant,
            challenge_description: "Difficulté à suivre les évolutions réglementaires et innovations agricoles".to_string(),
            solution_description: "Assistant IA personnalisé pour la veille réglementaire et les actualités agricoles".to_string(),
            business_metrics: metrics,
            roi_metrics: roi,
            testimonial: Some("AgriConseil IA est devenu indispensable. Je suis informé sans effort et je peux me concentrer sur mes cultures.".to_string()),
            is_featured: true,
            completion_date: "Mai 2025".to_string(),
            technologies_used: vec![
                "Modèles de langage (LLM)".to_string(),
                "API de messagerie (SMS/WhatsApp)".to_string(),
                "Web scraping".to_string(),
                "Base de données NoSQL".to_string(),
            ],
        }
    }

    /// Get all case studies
    pub fn get_all_case_studies() -> Vec<Self> {
        vec![Self::smartdocs(), Self::indusia(), Self::agriconseil_ia()]
    }

    /// Get featured case studies for home page
    pub fn get_featured_case_studies() -> Vec<Self> {
        vec![Self::smartdocs(), Self::indusia(), Self::agriconseil_ia()]
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
