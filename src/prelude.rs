//! Prelude containing commonly used types, traits, and functions.

pub use crate::core::clock::{SimDuration, SimulationClock, TimeUnit};
pub use crate::core::event::{Event, EventPayload, EventQueue};
pub use crate::core::rng::DeterministicRng;
pub use crate::core::world::{EntityId, EntityStore, World};

pub use crate::state::cell::{CellState, CellType};
pub use crate::state::colony::{Colony, PopulationStats};
pub use crate::state::epigenetic::EpigeneticState;
pub use crate::state::metrics::{QualityScore, StageProgress, WombState};

pub use crate::protocol::ivg::HayashiIvgProtocol;
pub use crate::protocol::reprogramming::ReprogrammingProtocol;
pub use crate::protocol::scnt::ScntProtocol;
pub use crate::protocol::trait_def::{Action, Protocol, Step, TimeCondition};
pub use crate::protocol::womb::ArtificialWombProtocol;

pub use crate::engine::reporter::{ColonySnapshot, SimulationReport, WombSnapshot};
pub use crate::engine::runner::SimulationRunner;
pub use crate::engine::scheduler::StepScheduler;

pub use crate::error::{BioSimError, Result};

// Re-exports from scies-bio-th
pub use scies_bio_th::ivg::{
    ActivationMethod, ArtificialWombChamber, DevelopmentalStage, EmbryoCultureEnvironment,
    EpigeneticProfile, GameteQuality, GameteType, HistoneMark, ImprintPattern, InductionProtocol,
    SyntheticEmbryo, SyntheticGamete,
};
pub use scies_bio_th::{BioError, Dna, Protein, Rna};
