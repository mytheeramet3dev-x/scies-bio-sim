//! Biological quality metrics, stage progress tracking, and artificial womb physiological state.

use crate::core::world::EntityId;
use scies_bio_th::ivg::{ArtificialWombChamber, DevelopmentalStage};
use serde::{Deserialize, Serialize};

/// Composite quality assessment score for synthetic gametes or embryos.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QualityScore {
    /// Overall composite quality score (0.0 to 100.0).
    pub overall: f64,
    /// Viability factor (0.0 to 100.0).
    pub viability_score: f64,
    /// Pluripotency or differentiation competence factor (0.0 to 100.0).
    pub pluripotency_score: f64,
    /// Epigenetic fidelity and imprint appropriateness (0.0 to 100.0).
    pub epigenetic_fidelity: f64,
    /// Genomic integrity / low DNA damage factor (0.0 to 100.0).
    pub genomic_integrity: f64,
}

impl QualityScore {
    /// Computes a composite quality score from physiological parameters.
    pub fn compute(
        viability: f64,
        pluripotency: f64,
        epigenetic_fidelity: f64,
        dna_damage: f64,
    ) -> Self {
        let v = (viability.clamp(0.0, 1.0)) * 100.0;
        let p = (pluripotency.clamp(0.0, 1.0)) * 100.0;
        let e = (epigenetic_fidelity.clamp(0.0, 1.0)) * 100.0;
        let g = ((1.0 - dna_damage).clamp(0.0, 1.0)) * 100.0;

        let overall = (v * 0.35 + p * 0.25 + e * 0.20 + g * 0.20).clamp(0.0, 100.0);
        Self {
            overall,
            viability_score: v,
            pluripotency_score: p,
            epigenetic_fidelity: e,
            genomic_integrity: g,
        }
    }
}

/// Progress tracker through embryogenesis and gestation milestones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageProgress {
    /// Current developmental morphological stage.
    pub stage: DevelopmentalStage,
    /// Cumulative age in days.
    pub age_days: f64,
    /// Estimated percentage of term or stage completed (0.0 to 100.0%).
    pub percent_completed: f64,
    /// Whether development has arrested prematurely.
    pub is_arrested: bool,
}

/// Detailed physiological state of an Artificial Womb (Biobag / Extra-uterine device).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WombState {
    /// Entity ID of the artificial womb chamber.
    pub id: EntityId,
    /// Implanted synthetic or biological embryo entity ID.
    pub embryo_id: Option<EntityId>,
    /// Elapsed gestation timeline in days.
    pub gestation_day: f64,
    /// Target full-term gestation period in days (e.g. 110 days for Cave Lion, 280 days for human).
    pub full_term_days: f64,
    /// Current estimated fetal weight in grams.
    pub fetal_weight_grams: f64,
    /// Current fetal physiological viability (0.0 to 100.0).
    pub fetal_viability: f64,
    /// Accumulated physiological stress / hypoxia index (0.0 to 1.0).
    pub stress_index: f64,
    /// Artificial womb chamber mechanical and fluidic parameters.
    pub chamber: ArtificialWombChamber,
    /// Whether decannulation or premature cessation occurred.
    pub is_terminated: bool,
    /// Whether full-term birth / emergence was achieved successfully.
    pub has_emerged: bool,
}

impl WombState {
    /// Creates an artificial womb configured for a specific gestation length.
    pub fn new(id: EntityId, full_term_days: f64) -> Self {
        Self {
            id,
            embryo_id: None,
            gestation_day: 0.0,
            full_term_days: full_term_days.max(1.0),
            fetal_weight_grams: 0.001,
            fetal_viability: 95.0,
            stress_index: 0.0,
            chamber: ArtificialWombChamber::default(),
            is_terminated: false,
            has_emerged: false,
        }
    }

    /// Default artificial womb configured for Cave Lion (Panthera spelaea) 110-day gestation.
    pub fn cave_lion_default(id: EntityId) -> Self {
        Self::new(id, 110.0)
    }

    /// Connects / implants an embryo into the womb.
    pub fn implant(&mut self, embryo_id: EntityId, initial_viability: f64) {
        self.embryo_id = Some(embryo_id);
        self.fetal_viability = initial_viability.clamp(0.0, 100.0);
        self.is_terminated = false;
        self.has_emerged = false;
    }

    /// Advances the gestation timeline by `hours`.
    pub fn advance_gestation(&mut self, hours: f64) {
        if self.is_terminated || self.has_emerged || self.embryo_id.is_none() {
            return;
        }

        let dt_days = hours / 24.0;
        self.gestation_day += dt_days;

        // Physiological evaluation of chamber conditions
        let pao2_optimal = (28.0..=35.0).contains(&self.chamber.oxygenator_pao2_mmhg);
        let patency_optimal = self.chamber.umbilical_cannulation_patency > 0.90;
        let temp_optimal = (37.0..=39.0).contains(&self.chamber.chamber_temp_celsius);

        if !pao2_optimal || !patency_optimal || !temp_optimal {
            self.stress_index = (self.stress_index + dt_days * 0.05).min(1.0);
            self.fetal_viability = (self.fetal_viability - dt_days * 2.0).max(0.0);
        } else {
            self.stress_index = (self.stress_index - dt_days * 0.02).max(0.0);
        }

        // Arrest if viability collapsed
        if self.fetal_viability < 15.0 {
            self.is_terminated = true;
            return;
        }

        // Fetal weight sigmoidal / exponential growth curve
        let progress_ratio = (self.gestation_day / self.full_term_days).clamp(0.0, 1.0);
        let max_birth_weight = 1850.0; // ~1.85 kg for Cave Lion cub
        self.fetal_weight_grams = max_birth_weight * progress_ratio.powf(3.2);

        // Check if full-term delivery achieved
        if self.gestation_day >= self.full_term_days {
            self.has_emerged = true;
        }
    }
}
