//! Services domain module
//!
//! This module contains the business logic and types related to SoftIA's service offerings.
//! It defines the core service entities and their business rules.

/// Service offering types
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceType {
    Exploration,
    Builder,
    Support,
    Hosting,
}

/// Main service offering structure
#[derive(Debug, Clone)]
pub struct ServiceOffering {
    pub name: String,
    pub service_type: ServiceType,
    pub price_ht: u32,
    pub duration_weeks: (u8, u8), // (min, max)
    pub description: String,
    pub benefits: Vec<String>,
    pub deliverables: Vec<String>,
    pub target_audience: Vec<String>,
    pub guarantee: Option<String>,
    pub roi_percentage: Option<u16>,
}

/// Business rules and logic for services
impl ServiceOffering {
    /// Create the Exploration service offering
    pub fn exploration() -> Self {
        Self {
            name: "Offre Exploration".to_string(),
            service_type: ServiceType::Exploration,
            price_ht: 4000,
            duration_weeks: (3, 6),
            description: "Audit complet de vos données et preuve de valeur via démonstrateur"
                .to_string(),
            benefits: vec![
                "Analyse de l'existant".to_string(),
                "Identification des opportunités".to_string(),
                "Démonstrateur fonctionnel".to_string(),
                "ROI estimé et plan d'action".to_string(),
            ],
            deliverables: vec![
                "Rapport d'audit data (20 pages)".to_string(),
                "Démonstrateur sur vos données".to_string(),
                "Plan de transformation (3 mois)".to_string(),
                "Estimation ROI personnalisée".to_string(),
            ],
            target_audience: vec![
                "PME débutantes en data".to_string(),
                "Dirigeants cherchant des quick wins".to_string(),
            ],
            guarantee: Some("Démonstrateur fonctionnel garanti ou remboursé".to_string()),
            roi_percentage: Some(300),
        }
    }

    /// Create the Builder service offering
    pub fn builder() -> Self {
        Self {
            name: "Offre Bâtisseur".to_string(),
            service_type: ServiceType::Builder,
            price_ht: 15000,
            duration_weeks: (8, 12),
            description: "Implémentation complète d'une solution data/IA sur mesure".to_string(),
            benefits: vec![
                "Solution complète et opérationnelle".to_string(),
                "Formation de vos équipes".to_string(),
                "Support pendant 3 mois".to_string(),
                "Évolutivité garantie".to_string(),
            ],
            deliverables: vec![
                "Plateforme data opérationnelle".to_string(),
                "Dashboards métier personnalisés".to_string(),
                "Modèles IA déployés".to_string(),
                "Documentation technique".to_string(),
                "Formation utilisateurs (2 jours)".to_string(),
            ],
            target_audience: vec![
                "PME avec projet data défini".to_string(),
                "Équipes ayant les bases techniques".to_string(),
            ],
            guarantee: Some("Solution fonctionnelle et adoptée par vos équipes".to_string()),
            roi_percentage: Some(400),
        }
    }

    /// Create the Support service offering
    pub fn support() -> Self {
        Self {
            name: "Support & Évolution".to_string(),
            service_type: ServiceType::Support,
            price_ht: 1500,
            duration_weeks: (4, 4),
            description: "Accompagnement pour l'évolution de vos solutions en continu".to_string(),
            benefits: vec![
                "Évolutions et améliorations".to_string(),
                "Support technique réactif".to_string(),
                "Nouveaux cas d'usage".to_string(),
                "Montée en compétences".to_string(),
            ],
            deliverables: vec![
                "Support technique (8h/mois)".to_string(),
                "Évolutions fonctionnelles".to_string(),
                "Rapport mensuel d'usage".to_string(),
                "Recommandations d'optimisation".to_string(),
            ],
            target_audience: vec![
                "Clients existants".to_string(),
                "Équipes autonomes souhaitant évoluer".to_string(),
            ],
            guarantee: None,
            roi_percentage: Some(200),
        }
    }

    /// Create the Hosting service offering
    pub fn hosting() -> Self {
        Self {
            name: "Hébergement & Maintenance".to_string(),
            service_type: ServiceType::Hosting,
            price_ht: 200,
            duration_weeks: (4, 4),
            description: "Hébergement sécurisé et maintenance de vos solutions data".to_string(),
            benefits: vec![
                "Infrastructure cloud sécurisée".to_string(),
                "Monitoring 24/7".to_string(),
                "Sauvegardes automatiques".to_string(),
                "Mises à jour de sécurité".to_string(),
            ],
            deliverables: vec![
                "Hébergement cloud (OVH/AWS)".to_string(),
                "Monitoring et alertes".to_string(),
                "Sauvegardes quotidiennes".to_string(),
                "Rapport mensuel de performance".to_string(),
            ],
            target_audience: vec![
                "PME sans infrastructure IT".to_string(),
                "Clients souhaitant externaliser".to_string(),
            ],
            guarantee: Some("Disponibilité 99.9% garantie".to_string()),
            roi_percentage: None,
        }
    }

    /// Get all main services
    pub fn all_main_services() -> Vec<Self> {
        vec![Self::exploration(), Self::builder()]
    }

    /// Get all additional services
    pub fn all_additional_services() -> Vec<Self> {
        vec![Self::support(), Self::hosting()]
    }

    /// Get formatted price string
    pub fn formatted_price(&self) -> String {
        match self.service_type {
            ServiceType::Support | ServiceType::Hosting => {
                format!("À partir de {} €/mois", self.price_ht)
            }
            _ => {
                format!("À partir de {} € HT", self.price_ht)
            }
        }
    }

    /// Get formatted duration string
    pub fn formatted_duration(&self) -> String {
        match self.duration_weeks {
            (0, 0) => "En continu".to_string(),
            (min, max) if min == max => format!("{} semaines", min),
            (min, max) => format!("{}-{} semaines", min, max),
        }
    }
}

/// Service categories for grouping
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceCategory {
    CoreOfferings,
    AdditionalServices,
}

/// Get services by category
pub fn get_services_by_category(category: ServiceCategory) -> Vec<ServiceOffering> {
    match category {
        ServiceCategory::CoreOfferings => ServiceOffering::all_main_services(),
        ServiceCategory::AdditionalServices => ServiceOffering::all_additional_services(),
    }
}
