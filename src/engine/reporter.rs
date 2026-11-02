//! Simulation reporter, snapshots, and experimental summaries.

use crate::core::event::Event;
use crate::core::world::{EntityId, EntityStore};
use crate::state::cell::CellState;
use crate::state::colony::{Colony, PopulationStats};
use crate::state::metrics::WombState;
use serde::{Deserialize, Serialize};

/// Detailed snapshot of a cellular colony at a given simulation time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColonySnapshot {
    /// Simulation hour of snapshot.
    pub hour: u64,
    /// Simulation day of snapshot.
    pub day: u64,
    /// Colony ID.
    pub colony_id: EntityId,
    /// Colony name.
    pub colony_name: String,
    /// Detailed population statistics.
    pub stats: PopulationStats,
    /// Overall viability ratio (0.0 to 1.0).
    pub success_rate: f64,
    /// Average composite quality score (0.0 to 100.0).
    pub mean_quality: f64,
}

impl ColonySnapshot {
    /// Captures a snapshot of a colony.
    pub fn capture(
        hour: u64,
        colony: &Colony,
        store: &EntityStore<CellState>,
    ) -> Self {
        let stats = colony.compute_stats(store);
        let success_rate = stats.viability_ratio;
        let mean_quality = (stats.mean_viability * 40.0
            + stats.mean_pluripotency * 25.0
            + (1.0 - stats.mean_dna_damage) * 20.0
            + (100.0 - stats.mean_epigenetic_age).clamp(0.0, 100.0) * 0.15)
            .clamp(0.0, 100.0);

        Self {
            hour,
            day: hour / 24,
            colony_id: colony.id,
            colony_name: colony.name.clone(),
            stats,
            success_rate,
            mean_quality,
        }
    }
}

/// Snapshot of an artificial womb chamber during gestation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WombSnapshot {
    /// Simulation hour.
    pub hour: u64,
    /// Gestation day.
    pub gestation_day: f64,
    /// Fetal viability (0.0 to 100.0).
    pub fetal_viability: f64,
    /// Estimated fetal weight in grams.
    pub fetal_weight_grams: f64,
    /// Whether decannulation or early arrest occurred.
    pub is_terminated: bool,
    /// Whether full-term delivery was achieved.
    pub has_emerged: bool,
}

impl WombSnapshot {
    /// Captures a snapshot of an artificial womb.
    pub fn capture(hour: u64, womb: &WombState) -> Self {
        Self {
            hour,
            gestation_day: womb.gestation_day,
            fetal_viability: womb.fetal_viability,
            fetal_weight_grams: womb.fetal_weight_grams,
            is_terminated: womb.is_terminated,
            has_emerged: womb.has_emerged,
        }
    }
}

/// Full comprehensive experimental simulation report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationReport {
    /// Seed used for the simulation.
    pub seed: u64,
    /// Total hours simulated.
    pub total_hours: u64,
    /// Final colony snapshot, if a colony was tracked.
    pub colony_snapshot: Option<ColonySnapshot>,
    /// Final artificial womb snapshot, if a womb was tracked.
    pub womb_snapshot: Option<WombSnapshot>,
    /// Chronological log of key simulation events.
    pub key_events: Vec<Event>,
}

impl SimulationReport {
    /// Serializes report into a pretty-printed JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
