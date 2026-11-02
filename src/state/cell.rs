//! Individual cell state and cellular lineage types.

use crate::core::world::EntityId;
use scies_bio_th::ivg::DevelopmentalStage;
use serde::{Deserialize, Serialize};

/// Type and differentiation lineage of a biological cell.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CellType {
    /// Fully differentiated somatic cell (e.g. dermal fibroblast).
    Somatic,
    /// Reprogrammed naive induced Pluripotent Stem Cell (iPSC).
    NaiveiPSC,
    /// Primed pluripotent epiblast-like cell ready for germline induction.
    PrimedPluripotent,
    /// Primordial Germ Cell-Like Cell (PGCLC, Sox17+/Blimp1+).
    PGCLC,
    /// Sex-specified embryonic gonocyte.
    Gonocyte,
    /// Unfertilized haploid oocyte (egg cell).
    Oocyte,
    /// Unfertilized haploid spermatozoon (sperm cell).
    Spermatozoon,
    /// Oocyte after nuclear enucleation (cytoplast only).
    EnucleatedOocyte,
    /// Fertilized diploid zygote or reconstructed SCNT construct.
    Zygote,
    /// Early cleavage stage blastomere (2-cell, 4-cell, 8-cell, morula).
    Blastomere,
    /// Pluripotent inner cell mass cell within blastocyst.
    InnerCellMass,
    /// Extra-embryonic trophectoderm cell.
    Trophectoderm,
    /// Terminally differentiated custom cell lineage.
    Differentiated(String),
}

impl CellType {
    /// Returns true if the cell is pluripotent (ESC or iPSC).
    pub fn is_pluripotent(&self) -> bool {
        matches!(self, CellType::NaiveiPSC | CellType::PrimedPluripotent | CellType::InnerCellMass)
    }

    /// Returns true if the cell is a germline lineage cell (PGCLC, Gonocyte, Oocyte, Sperm).
    pub fn is_germline(&self) -> bool {
        matches!(
            self,
            CellType::PGCLC | CellType::Gonocyte | CellType::Oocyte | CellType::Spermatozoon
        )
    }

    /// Returns true if the cell is an embryonic construct.
    pub fn is_embryonic(&self) -> bool {
        matches!(
            self,
            CellType::Zygote | CellType::Blastomere | CellType::InnerCellMass | CellType::Trophectoderm
        )
    }
}

/// Detailed physiological and epigenetic state of a single cell.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellState {
    /// Unique entity ID.
    pub id: EntityId,
    /// Current cell type lineage.
    pub cell_type: CellType,
    /// Viability probability factor (0.0 = non-viable/apoptotic, 1.0 = fully robust).
    pub viability: f64,
    /// Biological epigenetic age in years (or equivalent biological timeline).
    pub epigenetic_age: f64,
    /// Pluripotency score (0.0 = differentiated somatic, 1.0 = naive embryonic ground state).
    pub pluripotency: f64,
    /// Accumulated DNA double-strand break / mutation damage score (0.0 to 1.0).
    pub dna_damage: f64,
    /// Functional mitochondrial load capacity (0.0 to 1.0).
    pub mitochondrial_load: f64,
    /// Embryonic or developmental stage mapping.
    pub stage: DevelopmentalStage,
    /// User or protocol markers and metadata tags.
    pub tags: Vec<String>,
}

impl CellState {
    /// Creates a default somatic cell (e.g. donor skin fibroblast).
    pub fn somatic(id: EntityId, biological_age: f64) -> Self {
        Self {
            id,
            cell_type: CellType::Somatic,
            viability: 0.95,
            epigenetic_age: biological_age.max(0.0),
            pluripotency: 0.0,
            dna_damage: (biological_age * 0.005).clamp(0.01, 0.40),
            mitochondrial_load: (1.0 - biological_age * 0.005).clamp(0.5, 1.0),
            stage: DevelopmentalStage::FullTermNeonate,
            tags: vec!["somatic".to_string()],
        }
    }

    /// Creates a naive pluripotent stem cell (iPSC / ESC).
    pub fn naive_ipsc(id: EntityId) -> Self {
        Self {
            id,
            cell_type: CellType::NaiveiPSC,
            viability: 0.98,
            epigenetic_age: 0.0,
            pluripotency: 0.95,
            dna_damage: 0.02,
            mitochondrial_load: 0.95,
            stage: DevelopmentalStage::Blastocyst,
            tags: vec!["ipsc".to_string(), "pluripotent".to_string()],
        }
    }

    /// Creates an in vitro primed PGCLC (Primordial Germ Cell-Like Cell).
    pub fn pgclc(id: EntityId) -> Self {
        Self {
            id,
            cell_type: CellType::PGCLC,
            viability: 0.90,
            epigenetic_age: 0.0,
            pluripotency: 0.60,
            dna_damage: 0.03,
            mitochondrial_load: 0.92,
            stage: DevelopmentalStage::Gastrula,
            tags: vec!["pgclc".to_string(), "germline".to_string()],
        }
    }

    /// Creates an unfertilized synthetic oocyte.
    pub fn oocyte(id: EntityId) -> Self {
        Self {
            id,
            cell_type: CellType::Oocyte,
            viability: 0.92,
            epigenetic_age: 0.0,
            pluripotency: 0.10,
            dna_damage: 0.02,
            mitochondrial_load: 0.99,
            stage: DevelopmentalStage::Zygote,
            tags: vec!["oocyte".to_string(), "haploid".to_string()],
        }
    }

    /// Creates a fertilized or reconstructed zygote.
    pub fn zygote(id: EntityId, viability: f64) -> Self {
        Self {
            id,
            cell_type: CellType::Zygote,
            viability: viability.clamp(0.0, 1.0),
            epigenetic_age: 0.0,
            pluripotency: 0.85,
            dna_damage: 0.02,
            mitochondrial_load: 0.95,
            stage: DevelopmentalStage::Zygote,
            tags: vec!["zygote".to_string(), "embryo".to_string()],
        }
    }

    /// Returns true if the cell is viable and alive.
    pub fn is_alive(&self) -> bool {
        self.viability > 0.05 && self.dna_damage < 0.95
    }

    /// Applies physiological stress or damage, decreasing viability.
    pub fn apply_stress(&mut self, damage_factor: f64) {
        self.viability = (self.viability - damage_factor).clamp(0.0, 1.0);
        self.dna_damage = (self.dna_damage + damage_factor * 0.5).clamp(0.0, 1.0);
    }

    /// Heals or optimizes culture conditions, recovering viability.
    pub fn heal(&mut self, recovery_factor: f64) {
        self.viability = (self.viability + recovery_factor).clamp(0.0, 1.0);
        self.dna_damage = (self.dna_damage - recovery_factor * 0.3).clamp(0.0, 1.0);
    }

    /// Modifies pluripotency score.
    pub fn adjust_pluripotency(&mut self, delta: f64) {
        self.pluripotency = (self.pluripotency + delta).clamp(0.0, 1.0);
    }

    /// Modifies epigenetic biological age.
    pub fn adjust_epigenetic_age(&mut self, delta: f64) {
        self.epigenetic_age = (self.epigenetic_age + delta).max(0.0);
    }

    /// Appends a tag if not present.
    pub fn add_tag(&mut self, tag: &str) {
        if !self.has_tag(tag) {
            self.tags.push(tag.to_string());
        }
    }

    /// Checks if a tag exists.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_somatic_creation() {
        let cell = CellState::somatic(EntityId(1), 25.0);
        assert_eq!(cell.cell_type, CellType::Somatic);
        assert_eq!(cell.epigenetic_age, 25.0);
        assert!(cell.is_alive());
    }

    #[test]
    fn test_stress_and_healing() {
        let mut cell = CellState::naive_ipsc(EntityId(2));
        assert!(cell.is_alive());
        cell.apply_stress(0.4);
        assert!(cell.viability < 0.98);
        assert!(cell.dna_damage > 0.02);
        cell.heal(0.2);
        assert!(cell.viability > 0.6);
    }
}
