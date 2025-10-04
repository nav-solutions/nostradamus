use crate::{Epoch, Orbit};

pub trait State: Copy + Clone {
    /// Generates a default yet physically correct [State].
    fn default(epoch: Epoch) -> Self;

    /// Generates a random yet physically correct [State].
    fn random(epoch: Epoch) -> Self;

    /// Returns epoch of current [State]
    fn epoch(&self) -> Epoch;

    /// Updates [Epoch]
    fn set_epoch(&mut self, epoch: Epoch);

    /// Updates [Orbit]al state
    fn set_orbit(&mut self, orbit: Orbit);

    /// Copies and returns with updated [Epoch]
    fn with_epoch(mut self, epoch: Epoch) -> Self {
        self.set_epoch(epoch);
        self
    }

    /// Copies and returns with updated [Orbit]al state
    fn with_orbit(mut self, orbit: Orbit) -> Self {
        self.set_orbit(orbit);
        self
    }

    /// Temporal update mutable [Self].
    fn temporal_update(&mut self, epoch: Epoch, state: &Self);
}
