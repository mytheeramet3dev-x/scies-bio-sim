//! Artificial Womb (Biobag / Extra-uterine life support) gestation protocol.

use super::trait_def::{Action, Protocol, Step, TimeCondition};
use scies_bio_th::ivg::DevelopmentalStage;

/// Extra-uterine artificial womb incubation protocol.
/// Manages amniotic circuit flow, oxygenation, and developmental progression through gestation.
#[derive(Debug, Clone)]
pub struct ArtificialWombProtocol {
    full_term_days: u64,
    steps: Vec<Step>,
}

impl Default for ArtificialWombProtocol {
    fn default() -> Self {
        Self::cave_lion_default()
    }
}

impl ArtificialWombProtocol {
    /// Constructs protocol tailored for Cave Lion (Panthera spelaea) 110-day gestation.
    pub fn cave_lion_default() -> Self {
        Self::new(110)
    }

    /// Constructs protocol tailored for Human (Homo sapiens) 280-day gestation.
    pub fn human_default() -> Self {
        Self::new(280)
    }

    /// Constructs artificial womb protocol for arbitrary term length in days.
    pub fn new(full_term_days: u64) -> Self {
        let day_implant = 6.min(full_term_days / 15);
        let day_organo = 15.min(full_term_days / 6);
        let day_fetal = 45.min(full_term_days / 2);
        let day_late = 90.min(full_term_days * 4 / 5);

        let steps = vec![
            Step::new(
                "Artificial Womb Biobag Cannulation",
                TimeCondition::AtDay(day_implant),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Gastrula,
                    },
                    Action::LogNote("Embryo transferred into sterile synthetic amniotic chamber".to_string()),
                ],
            ),
            Step::new(
                "Early Organogenesis Viability Checkpoint",
                TimeCondition::AtDay(day_organo),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Organogenesis,
                    },
                    Action::ApplyFactor {
                        name: "Sequential Nutrient & Micronutrient Infusion".to_string(),
                        pluripotency_boost: 0.0,
                        viability_impact: 0.05,
                        epigenetic_rejuvenation: 0.0,
                    },
                ],
            ),
            Step::new(
                "Fetal Growth & Ossification Stage",
                TimeCondition::AtDay(day_fetal),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::FetalDevelopment,
                    },
                    Action::ApplyFactor {
                        name: "Surfactant & Corticosteroid Priming".to_string(),
                        pluripotency_boost: 0.0,
                        viability_impact: 0.03,
                        epigenetic_rejuvenation: 0.0,
                    },
                ],
            ),
            Step::new(
                "Late Fetal Maturation",
                TimeCondition::AtDay(day_late),
                vec![
                    Action::LogNote("Neurodevelopmental assessment: Fetal reflexes active".to_string()),
                ],
            ),
            Step::new(
                "Full-Term Decannulation & Neonatal Emergence",
                TimeCondition::AtDay(full_term_days),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::FullTermNeonate,
                    },
                    Action::LogNote(format!("Artificial womb decannulation complete at day {}", full_term_days)),
                ],
            ),
        ];

        Self {
            full_term_days,
            steps,
        }
    }

    /// Full term gestation days.
    pub fn full_term_days(&self) -> u64 {
        self.full_term_days
    }
}

impl Protocol for ArtificialWombProtocol {
    fn name(&self) -> &str {
        "Biobag Extra-Uterine Artificial Womb Protocol"
    }

    fn description(&self) -> &str {
        "Sterile synthetic amniotic fluid circulation, arteriovenous oxygenation, and physiological milestone monitoring."
    }

    fn steps(&self) -> &[Step] {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_womb_protocol_steps() {
        let proto = ArtificialWombProtocol::cave_lion_default();
        assert_eq!(proto.steps().len(), 5);
        assert_eq!(proto.full_term_days(), 110);
    }
}
