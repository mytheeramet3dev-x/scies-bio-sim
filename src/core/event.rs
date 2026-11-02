//! Simulation event queue and event records.

use super::world::EntityId;
use serde::{Deserialize, Serialize};

/// Specific payload describing an event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventPayload {
    /// A cell perished due to senescence or environmental stress.
    CellDied {
        cell_id: EntityId,
        reason: String,
    },
    /// A cell transitioned from one state or lineage to another.
    CellDifferentiated {
        cell_id: EntityId,
        from: String,
        to: String,
    },
    /// A biological or chemical factor (e.g. BMP4, OSKM, KDM4) was applied.
    FactorApplied {
        factor: String,
        affected_count: usize,
    },
    /// Nuclear transfer occurred between donor nucleus and recipient cytoplast.
    NuclearTransferCompleted {
        donor_id: EntityId,
        recipient_id: EntityId,
        success: bool,
    },
    /// Embryonic or cellular stage advanced.
    StageAdvanced {
        entity_id: EntityId,
        stage: String,
    },
    /// Artificial womb daily milestone.
    WombMilestone {
        womb_id: EntityId,
        gestation_day: u64,
        viability: f64,
        fetal_weight_grams: f64,
    },
    /// Generic or informational event message.
    Message(String),
}

/// An immutable event occurrence in the simulation timeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    /// Simulation hour when event occurred.
    pub hour: u64,
    /// Detailed event payload.
    pub payload: EventPayload,
}

impl Event {
    /// Creates a new simulation event.
    pub fn new(hour: u64, payload: EventPayload) -> Self {
        Self { hour, payload }
    }
}

/// Event bus storing simulation historical records.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    /// Creates an empty event queue.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Appends a new event.
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Records an event directly with current hour.
    pub fn record(&mut self, hour: u64, payload: EventPayload) {
        self.push(Event::new(hour, payload));
    }

    /// Returns all events in chronological order.
    pub fn all(&self) -> &[Event] {
        &self.events
    }

    /// Returns the number of events.
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Whether event queue is empty.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Clears all events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
