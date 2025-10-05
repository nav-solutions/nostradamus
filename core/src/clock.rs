use crate::{
    prelude::{Duration, Epoch, PhysicsResult},
    traits::*,
};

use anise::math::{Matrix3, Vector3};

// use core::f64::consts::PI;

// const PI_2: f64 = PI * 2.0;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Estimate {
    pub p: Matrix3,
    pub x: Vector3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Clock {
    /// Epoch of current state
    epoch: Epoch,

    /// Internal state vector
    x: Vector3,

    /// Latest estimate
    estimate: Estimate,

    /// STM
    stm: Matrix3,

    /// Q covar matrix
    q: Matrix3,

    /// P matrix
    p: Matrix3,

    /// True when current state is predicted
    predicted: bool,
}

impl Clock {
    /// Creates a new [Clock] state from offset observation (in seconds)
    pub fn from_offset_measurement(epoch: Epoch, offset_s: f64) -> Self {
        let x = Vector3::new(offset_s, 0.0, 0.0); // TODO
        Self {
            epoch,
            predicted: false,
            estimate: Estimate {
                p: Matrix3::identity(),
                x,
            },
            x,
            stm: Matrix3::identity(),
            q: Matrix3::identity(),
            p: Matrix3::identity(),
        }
    }
}

impl State for Clock {
    fn default(epoch: Epoch) -> Self {
        Self {
            epoch,
            predicted: false,
            x: Default::default(),
            q: Matrix3::identity(),
            p: Matrix3::identity(),
            stm: Matrix3::identity(),
            estimate: Default::default(),
        }
    }

    fn random(epoch: Epoch) -> Self {
        Self {
            epoch,
            predicted: false,
            x: Default::default(),
            q: Matrix3::identity(),
            p: Matrix3::identity(),
            stm: Matrix3::identity(),
            estimate: Default::default(),
        }
    }

    fn observed(mut self, state: Self) -> Self {
        self = state;
        self.predicted = false;
        self
    }

    fn epoch(&self) -> Epoch {
        self.epoch
    }
}

impl Predictable for Clock {
    fn predictable(&self) -> bool {
        true
    }

    fn predicted(&self) -> bool {
        self.predicted
    }

    fn predict(mut self, step: Duration) -> PhysicsResult<Self> {
        let dt_s = step.to_seconds();
        self.epoch += step;

        self.stm[(0, 1)] = dt_s;
        self.stm[(0, 2)] = 0.5 * dt_s.powi(2);
        self.stm[(1, 2)] = dt_s;

        self.q[(0, 0)] = dt_s.powi(4) / 4.0;
        self.q[(0, 1)] = dt_s.powi(3) / 2.0;
        self.q[(0, 1)] = dt_s.powi(2) / 2.0;

        self.q[(1, 0)] = dt_s.powi(3) / 2.0;
        self.q[(1, 1)] = dt_s.powi(2);
        self.q[(1, 2)] = dt_s;

        self.q[(2, 0)] = dt_s.powi(2) / 2.0;
        self.q[(2, 1)] = dt_s;

        self.predicted = true;
        Ok(self)
    }
}

impl Clock {}
