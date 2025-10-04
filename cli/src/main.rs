use env_logger::{Builder, Target};

use nostradamus_core::prelude::{Almanac, Duration, Simulation, TimeScale};

pub fn main() {
    let mut builder = Builder::from_default_env();

    builder
        .target(Target::Stdout)
        .format_timestamp_secs()
        .format_module_path(false)
        .init();

    let almanac = Almanac::until_2035().unwrap_or_else(|e| {
        panic!("failed to initiate ANISE session: {}", e);
    });

    let mut simulation =
        Simulation::now(Duration::from_microseconds(1.0), TimeScale::GPST, almanac);
}
