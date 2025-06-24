use std::f64::{INFINITY, NEG_INFINITY};

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: NEG_INFINITY,
    };

    pub const FULL: Interval = Interval {
        min: NEG_INFINITY,
        max: INFINITY,
    };

    pub const UNIT: Interval = Interval { min: 0.0, max: 1.0 };

    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        (self.max - self.min).max(0.0)
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        value > self.min && value < self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        value.clamp(self.min, self.max)
    }
}
