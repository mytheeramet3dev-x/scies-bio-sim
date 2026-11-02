//! Primary simulation engine orchestrator: SimulationRunner.

use super::reporter::{ColonySnapshot, SimulationReport, WombSnapshot};
use super::scheduler::StepScheduler;
use crate::core::clock::{SimDuration, SimulationClock, TimeUnit};
use crate::core::event::{EventPayload, EventQueue};
use crate::core::rng::DeterministicRng;
use crate::core::world::{EntityId, EntityStore, World};
use crate::error::{BioSimError, Result};
use crate::protocol::trait_def::{Action, Protocol};
use crate::state::cell::{CellState, CellType};
use crate::state::colony::Colony;
use crate::state::metrics::WombState;

/// Main simulation engine running deterministic, time-stepped biological processes.
#[derive(Debug, Clone)]
pub struct SimulationRunner {
    /// Internal clock tracking simulation time.
    pub clock: SimulationClock,
    /// Seedable deterministic random number generator.
    pub rng: DeterministicRng,
    /// World managing Entity ID generation.
    pub world: World,
    /// Store of all active individual cells.
    pub cells: EntityStore<CellState>,
    /// Store of colonies / vessels.
    pub colonies: EntityStore<Colony>,
    /// Store of artificial womb devices.
    pub wombs: EntityStore<WombState>,
    /// Chronological event recorder.
    pub events: EventQueue,
    /// Protocol step scheduler.
    pub scheduler: StepScheduler,
}

impl SimulationRunner {
    /// Creates a new simulation initialized with a fixed random seed.
    pub fn new(seed: u64) -> Self {
        Self {
            clock: SimulationClock::new(TimeUnit::Hour),
            rng: DeterministicRng::new(seed),
            world: World::new(),
            cells: EntityStore::new(),
            colonies: EntityStore::new(),
            wombs: EntityStore::new(),
            events: EventQueue::new(),
            scheduler: StepScheduler::new(),
        }
    }

    /// Returns the initial seed.
    pub fn seed(&self) -> u64 {
        self.rng.seed()
    }

    /// Registers and stores an existing cell into the world.
    pub fn insert_cell(&mut self, cell: CellState) -> EntityId {
        let id = cell.id;
        self.cells.insert(id, cell);
        id
    }

    /// Allocates and creates a somatic cell with specified biological age.
    pub fn create_somatic_cell(&mut self, biological_age: f64) -> EntityId {
        let id = self.world.next_entity_id();
        let cell = CellState::somatic(id, biological_age);
        self.cells.insert(id, cell);
        id
    }

    /// Allocates and creates a naive iPSC cell.
    pub fn create_ipsc_cell(&mut self) -> EntityId {
        let id = self.world.next_entity_id();
        let cell = CellState::naive_ipsc(id);
        self.cells.insert(id, cell);
        id
    }

    /// Creates a population colony of somatic cells.
    pub fn create_colony_from_somatic(
        &mut self,
        species_tax_id: u32,
        count: usize,
        donor_age: f64,
    ) -> EntityId {
        let colony_id = self.world.next_entity_id();
        let mut colony = Colony::new(
            colony_id,
            &format!("Somatic-Colony-Tax{}", species_tax_id),
            species_tax_id,
            self.clock.current_hours(),
        );

        for _ in 0..count {
            let cell_id = self.create_somatic_cell(donor_age);
            colony.add_cell(cell_id);
        }

        self.colonies.insert(colony_id, colony);
        colony_id
    }

    /// Creates a population colony of naive iPSC cells.
    pub fn create_colony_from_ipsc(&mut self, species_tax_id: u32, count: usize) -> EntityId {
        let colony_id = self.world.next_entity_id();
        let mut colony = Colony::new(
            colony_id,
            &format!("iPSC-Colony-Tax{}", species_tax_id),
            species_tax_id,
            self.clock.current_hours(),
        );

        for _ in 0..count {
            let cell_id = self.create_ipsc_cell();
            colony.add_cell(cell_id);
        }

        self.colonies.insert(colony_id, colony);
        colony_id
    }

    /// Allocates and initializes an Artificial Womb device.
    pub fn create_artificial_womb(&mut self, full_term_days: f64) -> EntityId {
        let womb_id = self.world.next_entity_id();
        let womb = WombState::new(womb_id, full_term_days);
        self.wombs.insert(womb_id, womb);
        womb_id
    }

    /// Performs SCNT: Fuses a donor nucleus into an enucleated oocyte cytoplast, yielding a zygote.
    pub fn create_scnt_embryo(
        &mut self,
        donor_id: EntityId,
        recipient_id: EntityId,
    ) -> Result<EntityId> {
        let donor = self
            .cells
            .get(donor_id)
            .ok_or(BioSimError::EntityNotFound(donor_id))?
            .clone();

        let recipient = self
            .cells
            .get(recipient_id)
            .ok_or(BioSimError::EntityNotFound(recipient_id))?
            .clone();

        if recipient.cell_type != CellType::Oocyte && recipient.cell_type != CellType::EnucleatedOocyte {
            return Err(BioSimError::IncompatibleCellType(
                "SCNT recipient must be an Oocyte or EnucleatedOocyte".to_string(),
            ));
        }

        // Epigenetic barrier penalty from donor age
        let age_penalty = (donor.epigenetic_age * 0.005).min(0.3);
        let zygote_viability = (donor.viability * 0.5 + recipient.viability * 0.5 - age_penalty)
            .clamp(0.2, 0.98);

        let embryo_id = self.world.next_entity_id();
        let embryo = CellState::zygote(embryo_id, zygote_viability);
        self.cells.insert(embryo_id, embryo);

        self.events.record(
            self.clock.current_hours(),
            EventPayload::NuclearTransferCompleted {
                donor_id,
                recipient_id,
                success: true,
            },
        );

        Ok(embryo_id)
    }

    /// Implants an embryo into an Artificial Womb device.
    pub fn implant_into_womb(&mut self, embryo_id: EntityId, womb_id: EntityId) -> Result<()> {
        let embryo = self
            .cells
            .get(embryo_id)
            .ok_or(BioSimError::EntityNotFound(embryo_id))?;

        let embryo_viability = embryo.viability * 100.0;

        let womb = self
            .wombs
            .get_mut(womb_id)
            .ok_or(BioSimError::EntityNotFound(womb_id))?;

        womb.implant(embryo_id, embryo_viability);

        self.events.record(
            self.clock.current_hours(),
            EventPayload::Message(format!(
                "Embryo {} successfully cannulated into Artificial Womb {}",
                embryo_id, womb_id
            )),
        );

        Ok(())
    }

    /// Executes protocol actions targeting a specific cellular colony.
    pub fn apply_action_to_colony(&mut self, action: &Action, colony_id: EntityId) -> Result<()> {
        let cell_ids = {
            let colony = self
                .colonies
                .get(colony_id)
                .ok_or(BioSimError::EntityNotFound(colony_id))?;
            colony.cell_ids.clone()
        };

        match action {
            Action::ChangeCellType { from, to, probability } => {
                for id in cell_ids {
                    if let Some(cell) = self.cells.get_mut(id) {
                        if !cell.is_alive() {
                            continue;
                        }
                        let matches_from = match from {
                            Some(expected) => &cell.cell_type == expected,
                            None => true,
                        };

                        if matches_from && self.rng.gen_bool(*probability) {
                            let old_type = format!("{:?}", cell.cell_type);
                            cell.cell_type = to.clone();
                            self.events.record(
                                self.clock.current_hours(),
                                EventPayload::CellDifferentiated {
                                    cell_id: id,
                                    from: old_type,
                                    to: format!("{:?}", to),
                                },
                            );
                        }
                    }
                }
            }

            Action::ApplyFactor {
                name,
                pluripotency_boost,
                viability_impact,
                epigenetic_rejuvenation,
            } => {
                let mut affected = 0;
                for id in cell_ids {
                    if let Some(cell) = self.cells.get_mut(id) {
                        if cell.is_alive() {
                            cell.adjust_pluripotency(*pluripotency_boost);
                            if *viability_impact >= 0.0 {
                                cell.heal(*viability_impact);
                            } else {
                                cell.apply_stress(-viability_impact);
                            }
                            cell.adjust_epigenetic_age(-epigenetic_rejuvenation * cell.epigenetic_age);
                            affected += 1;
                        }
                    }
                }
                self.events.record(
                    self.clock.current_hours(),
                    EventPayload::FactorApplied {
                        factor: name.clone(),
                        affected_count: affected,
                    },
                );
            }

            Action::DemethylateGermline { efficiency } => {
                for id in cell_ids {
                    if let Some(cell) = self.cells.get_mut(id) {
                        if cell.is_alive() {
                            cell.adjust_epigenetic_age(-efficiency * cell.epigenetic_age);
                            cell.add_tag("demethylated");
                        }
                    }
                }
            }

            Action::AdvanceStage { to } => {
                for id in cell_ids {
                    if let Some(cell) = self.cells.get_mut(id) {
                        if cell.is_alive() {
                            cell.stage = *to;
                            self.events.record(
                                self.clock.current_hours(),
                                EventPayload::StageAdvanced {
                                    entity_id: id,
                                    stage: format!("{:?}", to),
                                },
                            );
                        }
                    }
                }
            }

            Action::CullSenescentCells {
                min_viability,
                max_dna_damage,
            } => {
                for id in cell_ids {
                    if let Some(cell) = self.cells.get_mut(id) {
                        if cell.viability < *min_viability || cell.dna_damage > *max_dna_damage {
                            cell.viability = 0.0;
                            self.events.record(
                                self.clock.current_hours(),
                                EventPayload::CellDied {
                                    cell_id: id,
                                    reason: "CullSenescentCondition".to_string(),
                                },
                            );
                        }
                    }
                }
            }

            Action::LogNote(msg) => {
                self.events
                    .record(self.clock.current_hours(), EventPayload::Message(msg.clone()));
            }

            _ => {}
        }

        Ok(())
    }

    /// Advances simulation by one discrete time step `dt_hours`.
    pub fn step(&mut self, dt_hours: u64) {
        self.clock.advance_hours(dt_hours);

        // Advance any active artificial wombs
        let current_hour = self.clock.current_hours();
        let womb_ids: Vec<EntityId> = self.wombs.iter().map(|(id, _)| id).collect();

        for womb_id in womb_ids {
            if let Some(womb) = self.wombs.get_mut(womb_id) {
                womb.advance_gestation(dt_hours as f64);
                // Record daily milestone if at day boundary
                if current_hour % 24 == 0 {
                    let day = womb.gestation_day as u64;
                    let viab = womb.fetal_viability;
                    let weight = womb.fetal_weight_grams;
                    self.events.record(
                        current_hour,
                        EventPayload::WombMilestone {
                            womb_id,
                            gestation_day: day,
                            viability: viab,
                            fetal_weight_grams: weight,
                        },
                    );
                }
            }
        }
    }

    /// Runs the simulation for a specified duration.
    pub fn run_for(&mut self, duration: SimDuration) {
        let dt = 1; // 1-hour discrete step
        let total = duration.as_hours();
        for _ in 0..total {
            self.step(dt);
        }
    }

    /// Runs until reaching a target simulation day.
    pub fn run_until_day(&mut self, target_day: u64) {
        let target_hours = target_day * 24;
        while self.clock.current_hours() < target_hours {
            self.step(1);
        }
    }

    /// Runs an experimental protocol against a specific colony for a specified duration.
    pub fn run_protocol(
        &mut self,
        protocol: &dyn Protocol,
        colony_id: EntityId,
        duration: SimDuration,
    ) -> Result<()> {
        let total_hours = duration.as_hours();
        let dt = 1;

        for protocol_hour in 0..total_hours {
            // Evaluate which protocol steps trigger at this protocol timeline hour
            let triggers = self.scheduler.evaluate_triggers(protocol, protocol_hour, dt);

            for (_idx, step) in triggers {
                for action in &step.actions {
                    self.apply_action_to_colony(action, colony_id)?;
                }
            }

            self.step(dt);
        }

        Ok(())
    }

    /// Captures a snapshot of the specified colony.
    pub fn snapshot(&self, colony_id: EntityId) -> Option<ColonySnapshot> {
        let colony = self.colonies.get(colony_id)?;
        Some(ColonySnapshot::capture(
            self.clock.current_hours(),
            colony,
            &self.cells,
        ))
    }

    /// Captures a snapshot of the specified artificial womb.
    pub fn womb_snapshot(&self, womb_id: EntityId) -> Option<WombSnapshot> {
        let womb = self.wombs.get(womb_id)?;
        Some(WombSnapshot::capture(self.clock.current_hours(), womb))
    }

    /// Generates a comprehensive simulation report.
    pub fn generate_report(
        &self,
        colony_id: Option<EntityId>,
        womb_id: Option<EntityId>,
    ) -> SimulationReport {
        let colony_snapshot = colony_id.and_then(|id| self.snapshot(id));
        let womb_snapshot = womb_id.and_then(|id| self.womb_snapshot(id));

        SimulationReport {
            seed: self.seed(),
            total_hours: self.clock.current_hours(),
            colony_snapshot,
            womb_snapshot,
            key_events: self.events.all().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ivg::HayashiIvgProtocol;

    #[test]
    fn test_simulation_determinism() {
        let seed = 42;
        let mut sim1 = SimulationRunner::new(seed);
        let mut sim2 = SimulationRunner::new(seed);

        let col1 = sim1.create_colony_from_somatic(9606, 50, 30.0);
        let col2 = sim2.create_colony_from_somatic(9606, 50, 30.0);

        let proto = HayashiIvgProtocol::new();
        sim1.run_protocol(&proto, col1, SimDuration::days(10)).unwrap();
        sim2.run_protocol(&proto, col2, SimDuration::days(10)).unwrap();

        let snap1 = sim1.snapshot(col1).unwrap();
        let snap2 = sim2.snapshot(col2).unwrap();

        assert_eq!(snap1.stats.viable_cells, snap2.stats.viable_cells);
        assert_eq!(snap1.stats.mean_viability, snap2.stats.mean_viability);
        assert_eq!(snap1.stats.mean_pluripotency, snap2.stats.mean_pluripotency);
    }
}
