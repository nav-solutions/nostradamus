mod allocated;
mod referenced;

pub use allocated::*;
pub use referenced::*;

use sp3::prelude::{Error as SP3Error, SP3Entry, SP3Key, SV};

use crate::prelude::{Predictable, Satellite};

impl Satellite {
    /// Convert this [Satellite] state snapshot to SP3 compatible state,
    /// ready to be formatted.
    fn to_sp3(&self) -> (SP3Key, SP3Entry) {
        let pos_vel_km = self.spacecraft.orbit.to_cartesian_pos_vel();

        (
            SP3Key {
                sv: self.sv,
                epoch: self.spacecraft.orbit.epoch,
            },
            SP3Entry {
                maneuver: false,
                clock_event: false,
                predicted_orbit: self.predicted(),
                position_km: (pos_vel_km[0], pos_vel_km[1], pos_vel_km[2]),
                velocity_km_s: Some((pos_vel_km[3], pos_vel_km[4], pos_vel_km[5])),
                predicted_clock: self.clock.predicted(),
                clock_us: Some(self.clock.offset_seconds() * 1e6),
                clock_drift_ns: Some(self.clock.drift_seconds_sec() * 1e9),
            },
        )
    }
}
