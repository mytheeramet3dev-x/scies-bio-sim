//! Epigenetic landscape state, DNA methylation, and histone modification tracking.

use scies_bio_th::ivg::{EpigeneticProfile, ImprintPattern};
use serde::{Deserialize, Serialize};

/// Detailed epigenetic chromatin state of a biological cell or nucleus.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpigeneticState {
    /// Global 5mC DNA methylation density (0.0 = completely hypomethylated, 1.0 = fully methylated).
    pub global_methylation: f64,
    /// Somatic H3K9me3 heterochromatin reprogramming barrier (0.0 to 1.0).
    pub h3k9me3_barrier: f64,
    /// Polycomb repressive mark H3K27me3 (0.0 to 1.0).
    pub h3k27me3_level: f64,
    /// Parent-of-origin genomic imprinting configuration.
    pub imprint_pattern: ImprintPattern,
    /// Biological Horvath-like epigenetic clock age in years.
    pub epigenetic_age: f64,
}

impl EpigeneticState {
    /// Creates a somatic epigenetic profile based on donor age.
    pub fn somatic(donor_age_years: f64) -> Self {
        Self {
            global_methylation: 0.75,
            h3k9me3_barrier: 0.85,
            h3k27me3_level: 0.70,
            imprint_pattern: ImprintPattern::Paternal,
            epigenetic_age: donor_age_years.max(0.0),
        }
    }

    /// Creates a naive pluripotent ground state epigenetic profile.
    pub fn naive_pluripotent() -> Self {
        Self {
            global_methylation: 0.30,
            h3k9me3_barrier: 0.15,
            h3k27me3_level: 0.35,
            imprint_pattern: ImprintPattern::Paternal,
            epigenetic_age: 0.0,
        }
    }

    /// Creates an epigenetic profile for a mature haploid oocyte.
    pub fn mature_oocyte() -> Self {
        Self {
            global_methylation: 0.45,
            h3k9me3_barrier: 0.10,
            h3k27me3_level: 0.35,
            imprint_pattern: ImprintPattern::Maternal,
            epigenetic_age: 0.0,
        }
    }

    /// Creates an epigenetic profile for a mature haploid spermatozoon.
    pub fn mature_sperm() -> Self {
        Self {
            global_methylation: 0.90,
            h3k9me3_barrier: 0.15,
            h3k27me3_level: 0.25,
            imprint_pattern: ImprintPattern::Paternal,
            epigenetic_age: 0.0,
        }
    }

    /// Simulates TET-mediated DNA demethylation and imprint erasure.
    pub fn demethylate(&mut self, efficiency: f64) {
        let eff = efficiency.clamp(0.0, 1.0);
        self.global_methylation = (self.global_methylation * (1.0 - eff * 0.95)).max(0.02);
        if self.global_methylation < 0.10 {
            self.imprint_pattern = ImprintPattern::Erased;
            self.epigenetic_age = 0.0;
        } else {
            self.epigenetic_age = (self.epigenetic_age * (1.0 - eff)).max(0.0);
        }
    }

    /// Simulates histone demethylase activity (such as KDM4 injection in SCNT).
    pub fn remove_h3k9me3_barrier(&mut self, efficiency: f64) {
        let eff = efficiency.clamp(0.0, 1.0);
        self.h3k9me3_barrier = (self.h3k9me3_barrier * (1.0 - eff * 0.9)).clamp(0.02, 1.0);
    }

    /// Rejuvenates epigenetic biological age towards ground state 0.
    pub fn rejuvenate(&mut self, rate: f64) {
        self.epigenetic_age = (self.epigenetic_age * (1.0 - rate.clamp(0.0, 1.0))).max(0.0);
    }

    /// Converts this state into `scies-bio-th` `EpigeneticProfile`.
    pub fn to_bio_th(&self) -> EpigeneticProfile {
        EpigeneticProfile {
            global_methylation: self.global_methylation,
            h3k9me3_barrier: self.h3k9me3_barrier,
            h3k27me3_level: self.h3k27me3_level,
            imprint_pattern: self.imprint_pattern,
            epigenetic_age: self.epigenetic_age,
        }
    }

    /// Creates from `scies-bio-th` `EpigeneticProfile`.
    pub fn from_bio_th(profile: &EpigeneticProfile) -> Self {
        Self {
            global_methylation: profile.global_methylation,
            h3k9me3_barrier: profile.h3k9me3_barrier,
            h3k27me3_level: profile.h3k27me3_level,
            imprint_pattern: profile.imprint_pattern,
            epigenetic_age: profile.epigenetic_age,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epigenetic_demethylation_and_barrier_removal() {
        let mut state = EpigeneticState::somatic(45.0);
        assert_eq!(state.imprint_pattern, ImprintPattern::Paternal);
        assert!(state.h3k9me3_barrier > 0.8);

        state.remove_h3k9me3_barrier(0.9);
        assert!(state.h3k9me3_barrier < 0.2);

        state.demethylate(0.95);
        assert!(state.global_methylation < 0.10);
        assert_eq!(state.imprint_pattern, ImprintPattern::Erased);
    }
}
