//! Error types for scies-bio-sim.

use crate::core::world::EntityId;
use thiserror::Error;

/// Result type alias for simulation operations.
pub type Result<T> = std::result::Result<T, BioSimError>;

/// Errors encountered during simulation execution.
#[derive(Debug, Error)]
pub enum BioSimError {
    /// Entity not found in the entity store.
    #[error("Entity not found: {0}")]
    EntityNotFound(EntityId),

    /// Incompatible cell types for protocol action.
    #[error("Incompatible cell types for operation: {0}")]
    IncompatibleCellType(String),

    /// Embryo development arrested prematurely.
    #[error("Development arrested at stage {0:?}: {1}")]
    DevelopmentArrested(scies_bio_th::ivg::DevelopmentalStage, String),

    /// Womb or life support system failure.
    #[error("Artificial womb terminated: {0}")]
    WombFailure(String),

    /// Bridge error from scies-bio-th.
    #[error("Bio-TH error: {0}")]
    BioTh(#[from] scies_bio_th::BioError),

    /// Custom error message.
    #[error("Simulation error: {0}")]
    Other(String),
}
