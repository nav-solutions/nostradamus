use crate::prelude::{Duration, Epoch, PhysicsResult};

#[cfg(doc)]
use crate::prelude::TimeScale;

/// [State] model
pub trait State: Copy + Clone + PartialEq {
    /// Generates a default yet physically correct [State].
    fn default(epoch: Epoch) -> Self;

    /// Generates a random yet physically correct [State].
    fn random(epoch: Epoch) -> Self;

    /// Returns current [Epoch] of this [State]
    fn epoch(&self) -> Epoch;

    /// Provide a new [State] observation.
    fn observed(self, state: Self) -> Self;

    /// Observe a new [State].
    fn observe_mut(&mut self, state: Self) {
        *self = self.observed(state);
    }
}

/// [Predictable] [State]s
pub trait Predictable: State {
    /// Returns true if this [Predictable] is currently compatible with a new prediction.
    /// Which might not be the case, in particular when the pending update is not complete.
    fn predictable(&self) -> bool;

    /// Returns true if this latest [Predictable] state was actually predicted or not.
    /// Which is not necesarilly the case, it may come from actual measurement.
    fn predicted(&self) -> bool;

    /// Predict this [Predictable] using last measurement and current state, of given step [Duration].
    /// Use negative [Duration]s for backwards prediction.
    fn predict(self, step: Duration) -> PhysicsResult<Self>;

    /// Advance this mutable [Predictable] state using current state, either forward
    /// or backwards when using negative [Duration].
    fn predict_mut(&mut self, step: Duration) -> PhysicsResult<()> {
        *self = self.predict(step)?;
        Ok(())
    }
}

/// A [Scenario] is a chronological serie of non-predictable [State]s.
pub trait Scenario<S: State>: Iterator {
    /// Returns the total number of [State]s to be provided by this [Scenario]
    fn size(&self) -> usize;

    /// Insert a new [State] into this [Scenario] (builder helper)
    fn insert(&mut self, state: S);

    /// Adds a new [State] to this [Scenario] returning a new [Scenario]
    fn with_state(self, state: S) -> Self;
}
