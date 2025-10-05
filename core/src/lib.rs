#![doc(
    html_logo_url = "https://raw.githubusercontent.com/nav-solutions/.github/master/logos/logo2.jpg"
)]
#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod clock;
mod observer;
mod satellite;
mod scenario;
mod simulation;
mod state;

pub mod constants;

pub mod prelude {
    pub use anise::{
        astro::PhysicsResult,
        constants::frames::EARTH_J2000,
        prelude::{Almanac, Frame, Orbit},
    };

    pub use hifitime::prelude::{Duration, Epoch, TimeScale};

    pub use crate::{
        clock::Clock,
        observer::Observer,
        satellite::Satellite,
        scenario::Scenario,
        simulation::Simulation,
        state::{Predictable, State},
    };
}
