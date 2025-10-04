use crate::prelude::{Duration, Epoch};

/// Modeled and predictable [State]s
pub trait State: Copy + Clone {
    /// Generates a default yet physically correct [State].
    fn default(epoch: Epoch) -> Self;

    /// Generates a random yet physically correct [State].
    fn random(epoch: Epoch) -> Self;

    /// Returns true when this [State] does not result from an actual observation.
    fn predicted(&self) -> bool;

    /// Predict a new [S]tate either forward or backwards, of given step [Duration].
    fn predict(self, step: Duration) -> Self;

    /// Observe a new [S]tate at specific [Epoch]
    fn observe(&mut self, state: Self);

    /// Moves to new [S]tate either forward or backwards, of given step [Duration].
    fn predict_mut(&mut self, step: Duration) {
        *self = self.predict(step);
    }
}
