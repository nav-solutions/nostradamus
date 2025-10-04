use hifitime::errors::HifitimeError;

use crate::prelude::{Almanac, Duration, Epoch, TimeScale};

pub struct Simulation {
    /// Reference [TimeScale].
    /// All satellite states are referred to this [TimeScale].
    /// Any satellite we cannot reference will not contribute to the simulation process.
    timescale: TimeScale,

    /// Simulation start [Epoch]
    start_epoch: Epoch,

    /// Current [Epoch]
    epoch: Epoch,

    /// [Simulation] time-axis quantization as [Duration]
    step: Duration,

    /// [Simulation] [Almanac]
    almanac: Almanac,
}

impl std::fmt::Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}) elapsed={}", self.epoch, self.duration())
    }
}

impl Simulation {
    /// Initiates a new [Simulation].
    ///
    /// ## Input
    /// - epoch: simulation start time as [Epoch]
    /// - step: simulation step [Duration]
    /// - timescale: reference [TimeScale]
    /// - almanac: [Almanac] definition for this session
    pub fn new(epoch: Epoch, step: Duration, timescale: TimeScale, almanac: Almanac) -> Self {
        let epoch = epoch.to_time_scale(timescale);
        Self {
            step,
            epoch,
            timescale,
            almanac,
            start_epoch: epoch,
        }
    }

    /// Initiates a new [Simulation] with system time "now".
    /// - step: simulation step [Duration]
    /// - timescale: reference [TimeScale]
    /// - almanac: [Almanac] definition for this session
    pub fn now(
        step: Duration,
        timescale: TimeScale,
        almanac: Almanac,
    ) -> Result<Self, HifitimeError> {
        let now = Epoch::now()?.to_time_scale(timescale);

        Ok(Self::new(now, step, timescale, almanac))
    }

    /// Returns total [Duration] of this [Simulation] so far.
    pub fn duration(&self) -> Duration {
        self.epoch - self.start_epoch
    }

    /// Returns reference [TimeScale] for this [Simulation]
    pub fn timescale(&self) -> TimeScale {
        self.timescale
    }
}
