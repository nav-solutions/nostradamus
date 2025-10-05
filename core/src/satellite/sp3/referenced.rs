use crate::{
    errors::Error,
    prelude::{Clock, Duration, Orbit, Satellite, EARTH_J2000},
    traits::{Scenario, State},
};

use sp3::prelude::{Error as SP3Error, SP3, SV};

use anise::math::Vector6;

use nyx_space::{cosmic::GuidanceMode, Spacecraft};

/// [SP3ScenarioRef] allows executing [Scenario]s from SP3 text file easily,
/// without additional memory allocation.
pub struct SP3ScenarioRef<'a> {
    /// Sampling period
    step: Duration,

    /// [Satellite] [Iterator]
    iter: Box<dyn Iterator<Item = Satellite> + 'a>,
}

impl<'a> SP3ScenarioRef<'a> {
    /// Loads an [SP3Scenario] from parsed [SP3] object.
    ///
    /// ## Input
    /// - sp3: pre parsed [SP3] object which needs to be
    /// OD compatible (you should check that in advance).
    ///
    /// - satellite: specific filter, the scenario will only provide
    /// data for this spacecraft.
    ///
    /// ## Output
    /// - [SP3ScenarioRef] on success (yet it may be empty).
    pub fn from_sp3(sp3: &'a SP3, satellite: &'a SV) -> Result<Self, Error> {
        // must be OD compatible
        if !sp3.has_satellite_velocity() {
            return Err(Error::ODIncompatible);
        }

        // This will drop predicted sallites.
        // Both temporal and orbital states must be provided.
        // Restrict to selected satellite
        Ok(Self {
            step: sp3.header.sampling_period,
            iter: Box::new(sp3.data.iter().filter_map(|(k, v)| {
                // although file may be generally compatible,
                // we still need to check each observation remains OD compatible
                if k.sv == *satellite
                    && !v.predicted_clock
                    && !v.predicted_orbit
                    && !v.maneuver
                    && v.velocity_km_s.is_some()
                    && v.clock_us.is_some()
                {
                    let (x_km, y_km, z_km) = v.position_km;
                    // let (vx_km, vy_km, vz_km) = v.velocity_km_s.unwrap();
                    let (vx_km, vy_km, vz_km) = (0.0, 0.0, 0.0);

                    let pos_vel_km = Vector6::new(x_km, y_km, z_km, vx_km, vy_km, vz_km);
                    let orbit = Orbit::from_cartesian_pos_vel(pos_vel_km, k.epoch, EARTH_J2000);

                    // spacecraft modeling
                    let spacecraft = Spacecraft::builder()
                        .orbit(orbit)
                        .build()
                        .with_dry_mass(1_500.0)
                        .with_srp(25.0, 1.3)
                        .with_drag(25.0, 2.2)
                        .with_guidance_mode(GuidanceMode::Coast)
                        .with_prop_mass(0.0);

                    // on-board clock modeling
                    let clock = Clock::from_offset_measurement(k.epoch, v.clock_us.unwrap() * 1E-6);

                    Some(Satellite {
                        clock,
                        spacecraft,
                        predicted: false,
                    })
                } else {
                    None
                }
            })),
        })
    }
}

impl Iterator for SP3ScenarioRef<'_> {
    type Item = Satellite;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

// impl Scenario for SP3ScenarioRef<'_> {
//     fn step(&self) -> Duration {
//         self.step
//     }
//
//     fn size(&self) -> usize {
//         0 // TODO otherwise, provide a specific trait for non-allocated scenarios
//     }
//
//     fn remaining(&self) -> usize {
//         0 // TODO otherwise, provide a specific trait for non-allocated scenarios
//     }
// }
