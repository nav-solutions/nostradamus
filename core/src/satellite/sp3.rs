use crate::{
    errors::Error,
    prelude::{Clock, Orbit, Satellite, EARTH_J2000},
    traits::Scenario,
};

use sp3::prelude::{Error as SP3Error, SP3, SV};

use anise::math::Vector6;

use nyx_space::{cosmic::GuidanceMode, Spacecraft};

/// [SP3Scenario] allows loading [Scenario]s from SP3 text file easily.
pub struct SP3Scenario<'a> {
    /// [Satellite] states provider
    iter: Box<dyn Iterator<Item = Satellite> + 'a>,
}

impl<'a> SP3Scenario<'a> {
    /// Deploys an [SP3Scenario] from parsed [SP3] object.
    ///
    /// ## Input
    /// - sp3: parsed [SP3] which must be OD compatible.
    ///
    /// - satellite: specific filter, the scenario will only provide
    /// data for this spacecraft.
    ///
    /// ## Output
    /// - [SP3Scenario] on parsing success (yet it may be empty).
    pub fn from_sp3(sp3: &'a SP3, satellite: &'a SV) -> Result<Self, Error> {
        if !sp3.has_satellite_velocity() {
            // OD requires dynamics
            return Err(Error::ODIncompatible);
        }

        // This will drop predicted sallites.
        // Both temporal and orbital states must be provided.
        // Restrict to selected satellite
        Ok(Self {
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

impl Iterator for SP3Scenario<'_> {
    type Item = Satellite;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
