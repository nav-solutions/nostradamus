use crate::prelude::{Duration, Epoch, State};

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

    fn predict(mut self, step: Duration) -> Self {
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
        self
    }

    fn observe(&mut self, _: Self) {
        self.predicted = false;
    }

    fn predicted(&self) -> bool {
        self.predicted
    }

    fn epoch(&self) -> Epoch {
        self.epoch
    }
}

impl Clock {}
