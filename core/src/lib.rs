mod clock;
mod satellite;
mod simulation;
mod state;
// mod user;

// pub use satellite::*;
// pub use simulation::*;
// pub use user::*;

pub mod prelude {
    pub use anise::{
        constants::frames::EARTH_J2000,
        prelude::{Almanac, Frame, Orbit},
    };

    pub use hifitime::prelude::{Duration, Epoch, TimeScale};

    pub use crate::{clock::Clock, satellite::Satellite, simulation::Simulation, state::State};
}
