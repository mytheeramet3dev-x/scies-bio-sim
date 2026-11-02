//! Somatic Cell Nuclear Transfer (SCNT) cloning and embryonic activation protocol.

use super::trait_def::{Action, Protocol, Step, TimeCondition};
use crate::state::cell::CellType;
use scies_bio_th::ivg::DevelopmentalStage;

/// Somatic Cell Nuclear Transfer (SCNT) protocol with KDM4-mediated epigenetic barrier removal.
#[derive(Debug, Clone)]
pub struct ScntProtocol {
    steps: Vec<Step>,
}

impl Default for ScntProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl ScntProtocol {
    /// Constructs a standard mammalian SCNT protocol.
    pub fn new() -> Self {
        let steps = vec![
            Step::new(
                "Recipient Oocyte Enucleation",
                TimeCondition::AtHour(0),
                vec![
                    Action::EnucleateOocyte { oocyte_id: None },
                    Action::LogNote("Maternal spindle-chromosome complex aspirated".to_string()),
                ],
            ),
            Step::new(
                "Nuclear Transfer & Electrofusion",
                TimeCondition::AtHour(2),
                vec![
                    Action::NuclearTransfer {
                        donor: None,
                        recipient: None,
                    },
                    Action::LogNote("Donor somatic nucleus injected into perivitelline space".to_string()),
                ],
            ),
            Step::new(
                "Artificial Calcium Oscillation Activation",
                TimeCondition::AtHour(4),
                vec![
                    Action::ActivateScntConstruct { efficiency: 0.88 },
                    Action::ChangeCellType {
                        from: Some(CellType::Somatic),
                        to: CellType::Zygote,
                        probability: 0.85,
                    },
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Zygote,
                    },
                ],
            ),
            Step::new(
                "KDM4 Demethylase mRNA Injection",
                TimeCondition::AtHour(6),
                vec![
                    Action::ApplyFactor {
                        name: "KDM4A/D mRNA (H3K9me3 eraser)".to_string(),
                        pluripotency_boost: 0.35,
                        viability_impact: 0.05,
                        epigenetic_rejuvenation: 0.60,
                    },
                ],
            ),
            Step::new(
                "Embryonic Genome Activation (EGA) & 2-Cell Cleavage",
                TimeCondition::AtDay(1),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::TwoCell,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::Zygote),
                        to: CellType::Blastomere,
                        probability: 0.80,
                    },
                ],
            ),
            Step::new(
                "Morula Compaction",
                TimeCondition::AtDay(3),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Morula,
                    },
                ],
            ),
            Step::new(
                "Blastocyst Cavitation & Lineage Specification",
                TimeCondition::AtDay(5),
                vec![
                    Action::AdvanceStage {
                        to: DevelopmentalStage::Blastocyst,
                    },
                    Action::ChangeCellType {
                        from: Some(CellType::Blastomere),
                        to: CellType::InnerCellMass,
                        probability: 0.65,
                    },
                ],
            ),
        ];

        Self { steps }
    }
}

impl Protocol for ScntProtocol {
    fn name(&self) -> &str {
        "SCNT Nuclear Transfer & KDM4 Reprogramming Protocol"
    }

    fn description(&self) -> &str {
        "Enucleation -> Microinjection -> Chemical Activation -> KDM4 H3K9me3 demethylation -> Blastocyst progression."
    }

    fn steps(&self) -> &[Step] {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scnt_protocol_definition() {
        let proto = ScntProtocol::new();
        assert_eq!(proto.steps().len(), 7);
    }
}
