use crate::{
    errors::Error,
    prelude::{Clock, Orbit, Satellite, EARTH_J2000},
    traits::{Scenario, State},
};

use sp3::prelude::{Error as SP3Error, SP3, SV};

use anise::math::Vector6;

use nyx_space::{cosmic::GuidanceMode, Spacecraft};

/// [SP3Scenario] allows loading and pre-allocating a [Scenario] from an SP3 text file easily.
pub struct SP3Scenario {
    /// Total number of states
    size: usize,

    /// pointer
    ptr: usize,

    /// Collected [Satellite] states
    satellites: Vec<Satellite>,
}

impl SP3Scenario {
    /// Loads an [SP3Scenario] from parsed [SP3] object.
    ///
    /// ## Input
    /// - filename: readable SP3 fullpath. Gzip compressed files
    /// are supported but they must be terminated with ".gz".
    ///
    /// - satellite: specific filter, the scenario will only provide
    /// data for this spacecraft.
    ///
    /// ## Output
    /// - [SP3Scenario] on parsing success (yet it may be empty).
    pub fn from_file(filename: &str, satellite: SV) -> Result<Self, SP3Error> {
        let sp3 = if filename.ends_with(".gz") {
            SP3::from_gzip_file(filename)
        } else {
            SP3::from_file(filename)
        };

        let mut sp3 = sp3?;

        // Makes sure this is OD compatible
        if !sp3.has_satellite_velocity() {
            sp3.resolve_dynamics_mut();
        }

        // This will drop predicted sallites.
        // Both temporal and orbital states must be provided.
        // Restrict to selected satellite
        let satellites = sp3
            .data
            .iter()
            .filter_map(|(k, v)| {
                // although file may be generally compatible,
                // we still need to check each observation remains OD compatible
                if k.sv == satellite
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
            })
            .collect::<Vec<_>>();

        Ok(Self {
            ptr: 0,
            size: satellites.len(),
            satellites,
        })
    }
}

impl Iterator for SP3Scenario {
    type Item = Satellite;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr < self.size - 1 {
            None
        } else {
            self.ptr += 1;
            Some(self.satellites[self.ptr - 1])
        }
    }
}

impl<S: State> Scenario<S> for SP3Scenario {
    fn size(&self) -> usize {
        self.size
    }

    fn remaining(&self) -> usize {
        self.size - self.ptr
    }

    fn insert(&mut self, state: S) {
        // TODO
    }

    fn with_state(mut self, state: S) -> Self {
        // TODO
        self
    }
}
