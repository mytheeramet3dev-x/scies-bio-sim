//! Cellular colony aggregation and population dynamics statistics.

use crate::core::world::{EntityId, EntityStore};
use crate::state::cell::{CellState, CellType};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Statistical snapshot of a cellular population colony.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PopulationStats {
    /// Total count of cells tracked in colony.
    pub total_cells: usize,
    /// Count of viable living cells.
    pub viable_cells: usize,
    /// Count of dead / apoptotic cells.
    pub apoptotic_cells: usize,
    /// Distribution of cells by lineage type.
    pub type_distribution: BTreeMap<String, usize>,
    /// Mean viability score (0.0 to 1.0).
    pub mean_viability: f64,
    /// Mean pluripotency ground state score (0.0 to 1.0).
    pub mean_pluripotency: f64,
    /// Mean biological epigenetic age (years).
    pub mean_epigenetic_age: f64,
    /// Mean DNA damage score (0.0 to 1.0).
    pub mean_dna_damage: f64,
    /// Overall viability ratio (viable / total).
    pub viability_ratio: f64,
}

/// A biological colony or in vitro culture vessel of multiple cells.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Colony {
    /// Unique entity ID of the colony.
    pub id: EntityId,
    /// Descriptive name of the culture (e.g. "Hayashi-PGCLC-Vessel-A").
    pub name: String,
    /// NCBI Taxonomy ID of the host species (e.g. 9606 for Homo sapiens, 338153 for Panthera spelaea).
    pub species_tax_id: u32,
    /// Entity IDs of all member cells in this colony.
    pub cell_ids: Vec<EntityId>,
    /// Simulation hour when colony was seeded.
    pub seeded_hour: u64,
}

impl Colony {
    /// Creates a new empty colony container.
    pub fn new(id: EntityId, name: &str, species_tax_id: u32, seeded_hour: u64) -> Self {
        Self {
            id,
            name: name.to_string(),
            species_tax_id,
            cell_ids: Vec::new(),
            seeded_hour,
        }
    }

    /// Adds a cell to the colony.
    pub fn add_cell(&mut self, cell_id: EntityId) {
        if !self.cell_ids.contains(&cell_id) {
            self.cell_ids.push(cell_id);
        }
    }

    /// Removes a cell from the colony.
    pub fn remove_cell(&mut self, cell_id: EntityId) {
        self.cell_ids.retain(|&id| id != cell_id);
    }

    /// Computes aggregated statistical metrics across all cells in this colony.
    pub fn compute_stats(&self, store: &EntityStore<CellState>) -> PopulationStats {
        let mut total = 0;
        let mut viable_count = 0;
        let mut apoptotic_count = 0;
        let mut sum_viability = 0.0;
        let mut sum_pluripotency = 0.0;
        let mut sum_epigenetic_age = 0.0;
        let mut sum_dna_damage = 0.0;
        let mut distribution = BTreeMap::new();

        for &id in &self.cell_ids {
            if let Some(cell) = store.get(id) {
                total += 1;
                let type_name = format!("{:?}", cell.cell_type);
                *distribution.entry(type_name).or_insert(0) += 1;

                if cell.is_alive() {
                    viable_count += 1;
                } else {
                    apoptotic_count += 1;
                }

                sum_viability += cell.viability;
                sum_pluripotency += cell.pluripotency;
                sum_epigenetic_age += cell.epigenetic_age;
                sum_dna_damage += cell.dna_damage;
            }
        }

        let n = total.max(1) as f64;
        PopulationStats {
            total_cells: total,
            viable_cells: viable_count,
            apoptotic_cells: apoptotic_count,
            type_distribution: distribution,
            mean_viability: sum_viability / n,
            mean_pluripotency: sum_pluripotency / n,
            mean_epigenetic_age: sum_epigenetic_age / n,
            mean_dna_damage: sum_dna_damage / n,
            viability_ratio: if total > 0 {
                viable_count as f64 / total as f64
            } else {
                0.0
            },
        }
    }

    /// Returns the fraction of viable cells matching a specified target `CellType`.
    pub fn fraction_of_type(&self, target_type: &CellType, store: &EntityStore<CellState>) -> f64 {
        let mut matching = 0;
        let mut total_viable = 0;

        for &id in &self.cell_ids {
            if let Some(cell) = store.get(id) {
                if cell.is_alive() {
                    total_viable += 1;
                    if &cell.cell_type == target_type {
                        matching += 1;
                    }
                }
            }
        }

        if total_viable == 0 {
            0.0
        } else {
            matching as f64 / total_viable as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colony_stats() {
        let mut store = EntityStore::new();
        let mut colony = Colony::new(EntityId(100), "TestColony", 9606, 0);

        for i in 1..=10 {
            let id = EntityId(i);
            let mut cell = CellState::somatic(id, 30.0);
            if i > 8 {
                cell.viability = 0.01; // apoptotic
            }
            store.insert(id, cell);
            colony.add_cell(id);
        }

        let stats = colony.compute_stats(&store);
        assert_eq!(stats.total_cells, 10);
        assert_eq!(stats.viable_cells, 8);
        assert_eq!(stats.apoptotic_cells, 2);
        assert_eq!(stats.viability_ratio, 0.8);
    }
}
