use crate::{Duration, Epoch, State};

// use core::f64::consts::PI;

// const PI_2: f64 = PI * 2.0;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ClockState {
    /// Epoch of current state
    epoch: Epoch,

    /// current offset to reference
    offset: f64,

    /// average clock frequency
    frequency: f64,

    /// True when current state is predicted
    predicted: bool,
}

impl State for ClockState {
    fn default(epoch: Epoch) -> Self {
        Self {
            epoch,
            offset: 0.0,
            frequency: 1E6,
            predicted: false,
        }
    }

    fn random(epoch: Epoch) -> Self {
        Self {
            epoch,
            offset: 0.0,
            frequency: 1E6,
            predicted: false,
        }
    }

    fn predict(mut self, step: Duration) -> Self {
        self.epoch += step;
        self
    }

    fn observe(&mut self, state: Self) {
        *self = state;
        self.predicted = false;
    }

    fn predicted(&self) -> bool {
        self.predicted
    }
}
