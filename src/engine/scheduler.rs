//! Protocol step scheduler and trigger evaluation.

use crate::protocol::trait_def::{Protocol, Step, TimeCondition};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Tracks protocol step executions across simulation time.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepScheduler {
    executed_keys: HashSet<String>,
}

impl StepScheduler {
    /// Creates a new scheduler.
    pub fn new() -> Self {
        Self {
            executed_keys: HashSet::new(),
        }
    }

    /// Determines which steps in the given protocol should trigger in the current tick.
    pub fn evaluate_triggers<'a>(
        &mut self,
        protocol: &'a dyn Protocol,
        current_hour: u64,
        dt_hours: u64,
    ) -> Vec<(usize, &'a Step)> {
        let mut triggered = Vec::new();

        for (idx, step) in protocol.steps().iter().enumerate() {
            let is_match = step.at.matches(current_hour, dt_hours);

            if is_match {
                let execution_key = match step.at {
                    TimeCondition::AtHour(h) => format!("{}:step_{}:h{}", protocol.name(), idx, h),
                    TimeCondition::AtDay(d) => format!("{}:step_{}:d{}", protocol.name(), idx, d),
                    TimeCondition::EveryHours(_) => {
                        format!("{}:step_{}:every_h{}", protocol.name(), idx, current_hour)
                    }
                    TimeCondition::EveryDays(_) => {
                        format!("{}:step_{}:every_d{}", protocol.name(), idx, current_hour / 24)
                    }
                    TimeCondition::WhenCondition(_) => {
                        format!("{}:step_{}:cond_{}", protocol.name(), idx, current_hour)
                    }
                };

                if !self.executed_keys.contains(&execution_key) {
                    self.executed_keys.insert(execution_key);
                    triggered.push((idx, step));
                }
            }
        }

        triggered
    }

    /// Resets all execution records.
    pub fn reset(&mut self) {
        self.executed_keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::ivg::HayashiIvgProtocol;

    #[test]
    fn test_step_triggering() {
        let mut scheduler = StepScheduler::new();
        let proto = HayashiIvgProtocol::new();

        // At hour 0, day 0 step should trigger
        let triggered = scheduler.evaluate_triggers(&proto, 0, 1);
        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0].1.name, "Epiblast-Like Cell (EpiLC) Priming");

        // Same hour, should not trigger again
        let triggered_again = scheduler.evaluate_triggers(&proto, 0, 1);
        assert_eq!(triggered_again.len(), 0);

        // At Day 2 (48 hours), second step triggers
        let triggered_d2 = scheduler.evaluate_triggers(&proto, 48, 1);
        assert_eq!(triggered_d2.len(), 1);
        assert_eq!(triggered_d2[0].1.name, "PGCLC Specification Induction");
    }
}
