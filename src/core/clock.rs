//! Simulation clock, time units, and durations.

use serde::{Deserialize, Serialize};

/// Basic units of time supported by the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeUnit {
    /// 1 simulation hour.
    Hour,
    /// 1 simulation day (24 hours).
    Day,
    /// 1 standard cellular division cycle (~24 hours for typical mammalian cells).
    Cycle,
}

impl TimeUnit {
    /// Converts a value in this unit to hours.
    pub fn to_hours(self, value: u64) -> u64 {
        match self {
            TimeUnit::Hour => value,
            TimeUnit::Day => value * 24,
            TimeUnit::Cycle => value * 24,
        }
    }

    /// Converts hours to this unit (floored).
    pub fn from_hours(self, hours: u64) -> u64 {
        match self {
            TimeUnit::Hour => hours,
            TimeUnit::Day => hours / 24,
            TimeUnit::Cycle => hours / 24,
        }
    }
}

/// A representation of time elapsed or span in the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SimDuration {
    /// Duration expressed in hours.
    pub hours: u64,
}

impl SimDuration {
    /// Creates a duration from a count of hours.
    pub const fn from_hours(hours: u64) -> Self {
        Self { hours }
    }

    /// Creates a duration from a count of days (1 day = 24 hours).
    pub const fn from_days(days: u64) -> Self {
        Self { hours: days * 24 }
    }

    /// Shorthand alias for `from_days`.
    pub const fn days(days: u64) -> Self {
        Self::from_days(days)
    }

    /// Shorthand alias for `from_hours`.
    pub const fn hours(hours: u64) -> Self {
        Self::from_hours(hours)
    }

    /// Creates a duration from mammalian cell cycles (~24 hours per cycle).
    pub const fn cycles(cycles: u64) -> Self {
        Self { hours: cycles * 24 }
    }

    /// Returns the duration in whole days.
    pub const fn as_days(&self) -> u64 {
        self.hours / 24
    }

    /// Returns the duration in hours.
    pub const fn as_hours(&self) -> u64 {
        self.hours
    }
}

/// Simulation clock tracking discrete time advancement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationClock {
    /// Current elapsed simulation time in hours.
    current_hours: u64,
    /// Total discrete ticks performed.
    tick_count: u64,
    /// Primary base time unit displayed.
    base_unit: TimeUnit,
}

impl Default for SimulationClock {
    fn default() -> Self {
        Self::new(TimeUnit::Hour)
    }
}

impl SimulationClock {
    /// Creates a new clock starting at time 0.
    pub fn new(base_unit: TimeUnit) -> Self {
        Self {
            current_hours: 0,
            tick_count: 0,
            base_unit,
        }
    }

    /// Returns the current simulation time in hours.
    pub fn current_hours(&self) -> u64 {
        self.current_hours
    }

    /// Returns the current simulation day (0-indexed or 1-indexed helper).
    pub fn current_day(&self) -> u64 {
        self.current_hours / 24
    }

    /// Returns total tick steps executed.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Advances the clock by a specified duration in hours.
    pub fn advance_hours(&mut self, dt_hours: u64) {
        self.current_hours += dt_hours;
        self.tick_count += 1;
    }

    /// Advances the clock by a `SimDuration`.
    pub fn advance(&mut self, duration: SimDuration) {
        self.advance_hours(duration.as_hours());
    }

    /// Resets clock back to 0.
    pub fn reset(&mut self) {
        self.current_hours = 0;
        self.tick_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_advancement() {
        let mut clock = SimulationClock::default();
        assert_eq!(clock.current_hours(), 0);
        assert_eq!(clock.current_day(), 0);

        clock.advance_hours(12);
        assert_eq!(clock.current_hours(), 12);
        assert_eq!(clock.current_day(), 0);

        clock.advance(SimDuration::days(2));
        assert_eq!(clock.current_hours(), 60);
        assert_eq!(clock.current_day(), 2);
        assert_eq!(clock.tick_count(), 2);
    }

    #[test]
    fn test_time_units() {
        assert_eq!(TimeUnit::Day.to_hours(5), 120);
        assert_eq!(TimeUnit::Day.from_hours(120), 5);
        assert_eq!(SimDuration::cycles(3).as_hours(), 72);
    }
}
