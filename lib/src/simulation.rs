use hifitime::errors::HifitimeError;

use crate::{Duration, Epoch, SatelliteState, TimeScale, UserState};

pub struct Simulation {
    /// Constellation
    constellation: Constellation,

    /// Simulation start [Epoch]
    start_t: Epoch,

    /// Current [Epoch]
    current_t: Epoch,

    /// Simulation time step (time axis quantization)
    step: Duration,

    /// User
    user: UserState,

    /// Satellites
    satellites: Vec<SatelliteState>,
}

impl std::fmt::Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        format!("sim-run - duration={}", self.duration())
    }
}

impl Simulation {
    /// Initiates a new [Simulation].
    pub fn new(
        start_t: Epoch,
        user: UserState,
        satellites: Vec<SatelliteState>,
        step: Duration,
    ) -> Self {
        Self {
            step,
            user,
            start_t,
            satellites,
            current_t: start_t,
        }
    }

    /// Initiates a new [Simulation] with system time "now".
    pub fn now(
        user: UserState,
        satellites: Vec<SatelliteState>,
        step: Duration,
    ) -> Result<Self, HifitimeError> {
        let now = Epoch::now()?;
        Ok(Self::new(now, user, satellites, step))
    }

    // /// Initiates a new [Simulation] from a RINEX file
    // pub fn from_rinex(rinex: &Rinex) -> Self {
    //     Self {

    //     }
    // }

    /// Returns duration of this [Simulation] run
    pub fn duration(&self) -> Duration {
        self.current_t - self.start_t
    }

    /// Returns the timescale being simulated
    pub fn timescale(&self) -> TimeScale {}
}
