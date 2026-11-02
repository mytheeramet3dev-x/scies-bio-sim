//! Protocol, Step, Action, and Trigger condition definitions.

use crate::core::world::EntityId;
use crate::state::cell::CellType;
use scies_bio_th::ivg::DevelopmentalStage;
use serde::{Deserialize, Serialize};

/// Condition dictating when a protocol step is triggered.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeCondition {
    /// Triggers when the simulation clock reaches the specified absolute hour.
    AtHour(u64),
    /// Triggers when the simulation clock reaches the specified absolute day.
    AtDay(u64),
    /// Triggers every N hours continuously.
    EveryHours(u64),
    /// Triggers every N days continuously.
    EveryDays(u64),
    /// Arbitrary expression or tag-based condition.
    WhenCondition(String),
}

impl TimeCondition {
    /// Evaluates whether the step should trigger at the given clock state and step interval.
    pub fn matches(&self, current_hour: u64, dt_hours: u64) -> bool {
        match *self {
            TimeCondition::AtHour(target_hour) => {
                let prev_hour = current_hour.saturating_sub(dt_hours);
                if target_hour == 0 {
                    current_hour == 0
                } else {
                    target_hour > prev_hour && target_hour <= current_hour
                }
            }
            TimeCondition::AtDay(target_day) => {
                let target_hour = target_day * 24;
                let prev_hour = current_hour.saturating_sub(dt_hours);
                if target_hour == 0 {
                    current_hour == 0
                } else {
                    target_hour > prev_hour && target_hour <= current_hour
                }
            }
            TimeCondition::EveryHours(interval) => {
                if interval == 0 {
                    false
                } else {
                    current_hour > 0 && current_hour % interval == 0
                }
            }
            TimeCondition::EveryDays(days) => {
                let interval = days * 24;
                if interval == 0 {
                    false
                } else {
                    current_hour > 0 && current_hour % interval == 0
                }
            }
            TimeCondition::WhenCondition(_) => false,
        }
    }
}

/// Biological or environmental action executed within a protocol step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Changes the lineage of matching cells probabilistically.
    ChangeCellType {
        from: Option<CellType>,
        to: CellType,
        probability: f64,
    },
    /// Delivers molecular, growth, or transcription factors (e.g. BMP4, LIF, OSKM, KDM4).
    ApplyFactor {
        name: String,
        pluripotency_boost: f64,
        viability_impact: f64,
        epigenetic_rejuvenation: f64,
    },
    /// Erases parental imprints and performs TET-mediated demethylation.
    DemethylateGermline {
        efficiency: f64,
    },
    /// Advances morphological stage of embryonic constructs.
    AdvanceStage {
        to: DevelopmentalStage,
    },
    /// Enucleates an oocyte, removing maternal nuclear genetic material.
    EnucleateOocyte {
        oocyte_id: Option<EntityId>,
    },
    /// Transfers somatic donor nucleus into an enucleated oocyte cytoplast.
    NuclearTransfer {
        donor: Option<EntityId>,
        recipient: Option<EntityId>,
    },
    /// Induces artificial activation with SrCl2/Ionomycin or electrofusion.
    ActivateScntConstruct {
        efficiency: f64,
    },
    /// Eliminates non-viable or genetically degraded cells below thresholds.
    CullSenescentCells {
        min_viability: f64,
        max_dna_damage: f64,
    },
    /// Logs an experimental note or event.
    LogNote(String),
}

/// A discrete step comprising a trigger condition and executable actions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    /// Human-readable step name (e.g. "PGCLC Specification induction").
    pub name: String,
    /// When this step triggers.
    pub at: TimeCondition,
    /// Set of biological actions applied during this step.
    pub actions: Vec<Action>,
}

impl Step {
    /// Creates a new protocol step.
    pub fn new(name: &str, at: TimeCondition, actions: Vec<Action>) -> Self {
        Self {
            name: name.to_string(),
            at,
            actions,
        }
    }
}

/// Core trait representing an experimental protocol.
pub trait Protocol: std::fmt::Debug {
    /// Protocol name.
    fn name(&self) -> &str;

    /// Detailed description and citation of the protocol.
    fn description(&self) -> &str;

    /// Sequence of steps comprising the protocol.
    fn steps(&self) -> &[Step];

    /// Target species taxonomy ID, if restricted to a specific species.
    fn species_tax_id(&self) -> Option<u32> {
        None
    }
}
