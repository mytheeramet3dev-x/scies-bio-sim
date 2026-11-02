//! # scies-bio-sim
//!
//! A deterministic, discrete time-step biological process simulator for Rust.
//!
//! Part of the `scies-series` bioinformatics ecosystem (`scies-bio-th`, `scies-bio-db`, `scies-bio-sim`).
//! Simulates multi-stage pipelines including In Vitro Gametogenesis (IVG), Somatic Cell Nuclear Transfer (SCNT),
//! Cellular Reprogramming, and Extra-uterine Artificial Womb (Biobag) gestation.
//!
//! ## Design Principles
//! - **Deterministic by default**: Given the same seed, two runs will always produce bit-for-bit identical outcomes.
//! - **Protocol-centric**: High-level modeling of experimental procedures through timed biological actions.
//! - **State is explicit**: Fully serializable world, colony, cell, and womb states.
//! - **Zero heavy dependency**: Retains the lightweight, fast philosophy of the `scies` ecosystem.

pub mod core;
pub mod engine;
pub mod error;
pub mod prelude;
pub mod protocol;
pub mod state;

// Direct convenient re-exports
pub use core::{DeterministicRng, EntityId, SimDuration, SimulationClock, TimeUnit, World};
pub use engine::{ColonySnapshot, SimulationReport, SimulationRunner, StepScheduler, WombSnapshot};
pub use error::{BioSimError, Result};
pub use protocol::{
    Action, ArtificialWombProtocol, HayashiIvgProtocol, Protocol, ReprogrammingProtocol,
    ScntProtocol, Step, TimeCondition,
};
pub use state::{
    CellState, CellType, Colony, EpigeneticState, PopulationStats, QualityScore, StageProgress,
    WombState,
};
