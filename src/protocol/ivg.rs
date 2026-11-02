//! In Vitro Gametogenesis (IVG) protocols (Hayashi et al. 2016, Saitou et al. 2018).

use super::trait_def::{Action, Protocol, Step, TimeCondition};
use crate::state::cell::CellType;
use scies_bio_th::ivg::DevelopmentalStage;

/// In Vitro Gametogenesis protocol based on Hayashi et al. (Nature 2016).
/// Drives somatic / pluripotent stem cells into functional synthetic oocytes through PGCLC induction.
#[derive(Debug, Clone)]
pub struct HayashiIvgProtocol {
    steps: Vec<Step>,
}

impl Default for HayashiIvgProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl HayashiIvgProtocol {
    /// Constructs the standard Hayashi 2016 IVG protocol.
    pub fn new() -> Self {
        let steps = vec![
            Step::new(
                "Epiblast-Like Cell (EpiLC) Priming",
                TimeCondition::AtDay(0),
                vec![
                    Action::ApplyFactor {
                        name: "Activin A + bFGF".to_string(),
                        pluripotency_boost: 0.1,
                        viability_impact: 0.0,
                        epigenetic_rejuvenation: 0.1,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::NaiveiPSC),
                        to: CellType::PrimedPluripotent,
                        probability: 0.85,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::Somatic),
                        to: CellType::PrimedPluripotent,
                        probability: 0.50,
                    },
                ],
            ),
            Step::new(
                "PGCLC Specification Induction",
                TimeCondition::AtDay(2),
                vec![
                    Action::ApplyFactor {
                        name: "BMP4 + LIF + SCF + EGF".to_string(),
                        pluripotency_boost: 0.05,
                        viability_impact: -0.05,
                        epigenetic_rejuvenation: 0.2,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::PrimedPluripotent),
                        to: CellType::PGCLC,
                        probability: 0.75,
                    },
                ],
            ),
            Step::new(
                "Germline Epigenetic Imprint Erasure",
                TimeCondition::AtDay(6),
                vec![
                    Action::DemethylateGermline { efficiency: 0.90 },
                    Action::ApplyFactor {
                        name: "Ascorbic Acid (Vitamin C / TET activator)".to_string(),
                        pluripotency_boost: 0.0,
                        viability_impact: 0.02,
                        epigenetic_rejuvenation: 0.3,
                    },
                ],
            ),
            Step::new(
                "Gonadal Reconstitution & Meiotic Entry",
                TimeCondition::AtDay(14),
                vec![
                    Action::ApplyFactor {
                        name: "Retinoic Acid (Stra8 induction)".to_string(),
                        pluripotency_boost: -0.3,
                        viability_impact: -0.05,
                        epigenetic_rejuvenation: 0.0,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::PGCLC),
                        to: CellType::Gonocyte,
                        probability: 0.80,
                    },
                ],
            ),
            Step::new(
                "In Vitro Maturation (IVM) to MII Oocytes",
                TimeCondition::AtDay(35),
                vec![
                    Action::ApplyFactor {
                        name: "FSH + LH + Amphiregulin (Oocyte maturation cocktail)".to_string(),
                        pluripotency_boost: -0.2,
                        viability_impact: 0.05,
                        epigenetic_rejuvenation: 0.0,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::Gonocyte),
                        to: CellType::Oocyte,
                        probability: 0.70,
                    },
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Zygote,
                    },
                    Action::CullSenescentCells {
                        min_viability: 0.25,
                        max_dna_damage: 0.60,
                    },
                ],
            ),
        ];

        Self { steps }
    }
}

impl Protocol for HayashiIvgProtocol {
    fn name(&self) -> &str {
        "Hayashi IVG (2016) Protocol"
    }

    fn description(&self) -> &str {
        "Murine/mammalian reconstitutive In Vitro Gametogenesis: Naive iPSC -> EpiLC -> PGCLC -> Epigenetic Reset -> Synthetic Oocyte."
    }

    fn steps(&self) -> &[Step] {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hayashi_protocol_definition() {
        let proto = HayashiIvgProtocol::new();
        assert_eq!(proto.steps().len(), 5);
        assert_eq!(proto.steps()[0].name, "Epiblast-Like Cell (EpiLC) Priming");
    }
}
