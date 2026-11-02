//! Biological state representations: Cells, Colonies, Epigenetics, and Quality Metrics.

pub mod cell;
pub mod colony;
pub mod epigenetic;
pub mod metrics;

pub use cell::{CellState, CellType};
pub use colony::{Colony, PopulationStats};
pub use epigenetic::EpigeneticState;
pub use metrics::{QualityScore, StageProgress, WombState};
