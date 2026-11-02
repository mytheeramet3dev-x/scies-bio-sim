//! Yamanaka transcription factor (OSKM) cellular reprogramming protocol.

use super::trait_def::{Action, Protocol, Step, TimeCondition};
use crate::state::cell::CellType;
use scies_bio_th::ivg::DevelopmentalStage;

/// Yamanaka 4-factor (Oct4, Sox2, Klf4, c-Myc) somatic reprogramming protocol.
/// Reverses somatic epigenetic age and restores naive ground-state pluripotency.
#[derive(Debug, Clone)]
pub struct ReprogrammingProtocol {
    steps: Vec<Step>,
}

impl Default for ReprogrammingProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl ReprogrammingProtocol {
    /// Constructs standard OSKM reprogramming protocol.
    pub fn new() -> Self {
        let steps = vec![
            Step::new(
                "OSKM Factor Transduction",
                TimeCondition::AtDay(0),
                vec![
                    Action::ApplyFactor {
                        name: "OSKM (Oct4, Sox2, Klf4, c-Myc)".to_string(),
                        pluripotency_boost: 0.15,
                        viability_impact: -0.05,
                        epigenetic_rejuvenation: 0.20,
                    },
                    Action::LogNote("OSKM transcription factors introduced via synthetic mRNA/episomal vector".to_string()),
                ],
            ),
            Step::new(
                "Mesenchymal-to-Epithelial Transition (MET)",
                TimeCondition::AtDay(5),
                vec![
                    Action::ApplyFactor {
                        name: "TGF-beta inhibitor (A83-01) + Vitamin C".to_string(),
                        pluripotency_boost: 0.25,
                        viability_impact: 0.05,
                        epigenetic_rejuvenation: 0.30,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::Somatic),
                        to: CellType::PrimedPluripotent,
                        probability: 0.60,
                    },
                ],
            ),
            Step::new(
                "Endogenous Pluripotency Network Activation",
                TimeCondition::AtDay(12),
                vec![
                    Action::ApplyFactor {
                        name: "2i/LIF medium (CHIR99021 + PD0325901)".to_string(),
                        pluripotency_boost: 0.35,
                        viability_impact: 0.02,
                        epigenetic_rejuvenation: 0.40,
                    },
                    Action::DemethylateGermline { efficiency: 0.70 },
                ],
            ),
            Step::new(
                "Naive iPSC Ground State Stabilization",
                TimeCondition::AtDay(18),
                vec![
                    Action::ChangeCellType {
                        from: Some(CellType::PrimedPluripotent),
                        to: CellType::NaiveiPSC,
                        probability: 0.85,
                    },
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Blastocyst,
                    },
                    Action::CullSenescentCells {
                        min_viability: 0.50,
                        max_dna_damage: 0.40,
                    },
                ],
            ),
        ];

        Self { steps }
    }
}

impl Protocol for ReprogrammingProtocol {
    fn name(&self) -> &str {
        "Yamanaka OSKM Cellular Reprogramming Protocol"
    }

    fn description(&self) -> &str {
        "Transduction of Oct4, Sox2, Klf4, c-Myc into somatic cells driving reversion to naive pluripotent stem cells (iPSCs)."
    }

    fn steps(&self) -> &[Step] {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reprogramming_steps() {
        let proto = ReprogrammingProtocol::new();
        assert_eq!(proto.steps().len(), 4);
    }
}
