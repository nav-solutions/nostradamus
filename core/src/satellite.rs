use crate::{
    constants::EARTH_ANGULAR_VEL_DEG_S,
    prelude::{Clock, Duration, Epoch, Frame, Orbit, State, EARTH_J2000},
};

use anise::astro::PhysicsResult;

use nyx_space::{md::prelude::GuidanceMode, Spacecraft};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Satellite {
    /// True when current state is predicted
    predicted: bool,

    /// [Spacecraft] model from nyx.
    /// The orbital state contains the [Epoch] of observation/preduction.
    spacecraft: Spacecraft,

    /// Satellite on-board [Clock]
    clock: Clock,
}

impl Satellite {
    /// Returns the [Frame] this [Satellite] is expressed-in
    fn frame(&self) -> Frame {
        self.spacecraft.orbit.frame
    }

    /// Copies and returns updated [Satellite] state with new orbital position,
    /// from latitude longitude coordinates in degrees, and altitude in kilometers.
    pub fn with_latlongalt_km(
        mut self,
        latitude_deg: f64,
        longitude_deg: f64,
        height_km: f64,
    ) -> PhysicsResult<Self> {
        self.spacecraft.orbit = Orbit::try_latlongalt(
            latitude_deg,
            longitude_deg,
            height_km,
            EARTH_ANGULAR_VEL_DEG_S,
            self.epoch(),
            self.frame(),
        )?;

        Ok(self)
    }

    /// Copies and returns updated [Satellite] state with new orbital position,
    /// from ECEF position in kilometers.
    pub fn with_position_km(mut self, x_km: f64, y_km: f64, z_km: f64) -> Self {
        self.spacecraft.orbit = Orbit::from_position(x_km, y_km, z_km, self.epoch(), self.frame());
        self
    }

    /// Copies and returns updated [Satellite] model with new drag area and Cd
    pub fn with_drag(mut self, drag_area_m2: f64, cd: f64) -> Self {
        self.spacecraft = self.spacecraft.with_drag(drag_area_m2, cd);
        self
    }

    /// Copies and returns updated [Satellite] model with new solar radial pressure
    /// area and Cr
    pub fn with_srp(mut self, srp_area_m2: f64, cr: f64) -> Self {
        self.spacecraft = self.spacecraft.with_srp(srp_area_m2, cr);
        self
    }

    /// Copies and returns updated [Satellite] model with new dry-mass in kilograms.
    pub fn with_dry_mass(mut self, mass_kg: f64) -> Self {
        self.spacecraft = self.spacecraft.with_dry_mass(mass_kg);
        self
    }
}

impl State for Satellite {
    fn default(epoch: Epoch) -> Self {
        // model:
        // Dry mass: 1_500 kg
        // SRP   Cr: 1.3
        // SRP area: 25 (m^2)
        // Drag  Cd: 2.2
        // DragArea: 25  (m^2)
        let orbit = Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000); // TODO
        Self {
            predicted: false,
            clock: Clock::default(epoch),
            spacecraft: Spacecraft::builder()
                .orbit(orbit)
                .build()
                .with_dry_mass(1_500.0)
                .with_srp(25.0, 1.3)
                .with_drag(25.0, 2.0)
                .with_guidance_mode(GuidanceMode::Coast)
                .with_prop_mass(0.0),
        }
    }

    fn random(epoch: Epoch) -> Self {
        // model:
        // Dry mass: 1_500 kg
        // SRP   Cr: 1.3
        // SRP area: 25 (m^2)
        // Drag  Cd: 2.2
        // DragArea: 25  (m^2)
        let orbit = Orbit::from_position(0.0, 0.0, 0.0, epoch, EARTH_J2000); // TODO
        Self {
            predicted: false,
            clock: Clock::random(epoch),
            spacecraft: Spacecraft::builder()
                .orbit(orbit)
                .build()
                .with_dry_mass(1_500.0)
                .with_srp(25.0, 1.3)
                .with_drag(25.0, 2.0)
                .with_guidance_mode(GuidanceMode::Coast)
                .with_prop_mass(0.0),
        }
    }

    fn epoch(&self) -> Epoch {
        self.spacecraft.orbit.epoch
    }

    fn predicted(&self) -> bool {
        self.predicted
    }

    fn predict(mut self, step: Duration) -> Self {
        self.predicted = true;
        self
    }

    fn observe(&mut self, state: Self) {}
}
