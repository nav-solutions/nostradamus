use env_logger::{Builder, Target};

use nostradamus_core::prelude::{
    Almanac, Constellation, Duration, SP3Scenario, Simulation, TimeScale, SV,
};

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

    // example: simulation preset from text file
    // currently limited to SP3 only
    let g01 = SV::new(Constellation::GPS, 1); // example

    let mut scenario =
        SP3Scenario::from_file("data/SP3/C/ESA0OPSULT_20232320600_02D_15M_ORB.SP3.gz", g01)
            .unwrap();

    let mut simulation = Simulation::new(scenario, almanac);
}
