use crate::prelude::{Duration, Epoch, Error, PhysicsResult, TimeSeries};

/// [TimeAxis] definition, all our synchronous objects are tied to a [TimeAxis].
#[derive(Clone, Copy, PartialEq)]
pub struct TimeAxis {
    /// First [Epoch] in chronological order
    pub start: Epoch,

    /// Last [Epoch] in chronological order
    pub end: Epoch,

    /// Axis quantization as [Duration]
    pub step: Duration,

    /// Current [Epoch] pointer
    pub current: Epoch,
}

impl TimeAxis {
    /// Defines a new finite [TimeAxis]. Stepping will terminate once
    /// the final [Epoch] has been reached.
    ///
    /// ## Input
    /// - start: initial [Epoch] (included)
    /// - end: final [Epoch] (included)
    /// - step: sign defines the axis direction
    pub fn new(start: Epoch, end: Epoch, step: Duration) -> Self {
        if step.is_negative() {
            Self {
                step,
                end: start,
                start: end,
                current: end,
            }
        } else {
            Self {
                end,
                step,
                start,
                current: start,
            }
        }
    }

    /// Increase this time axis of a single step in the correct direction
    pub fn extended(mut self) -> Self {
        self.end += self.step;
        self
    }

    /// Extend this mutable [TimeAxis]
    pub fn extend_mut(&mut self) {
        self.end += self.step;
    }

    /// Returns true if this is a [TimeAxis] is iterated forward
    pub fn is_forward(&self) -> bool {
        !self.step.is_negative()
    }

    /// Creates a reversed [TimeAxis] starting from latest state.
    pub fn reversed(&self) -> Self {
        Self {
            end: self.start,
            start: self.end,
            step: self.step,
            current: self.current,
        }
    }

    /// Returns elapsed [Duration] since initial [Epoch] and current state
    pub fn elapsed(&self) -> Duration {
        self.current - self.start
    }

    /// Returns remaining [Duration] before final [Epoch] is reached
    pub fn remaining(&self) -> Duration {
        if self.current == self.end {
            Duration::ZERO
        } else {
            self.end - self.current
        }
    }

    /// Reverse this mutable [TimeAxis], offset position is preserved.
    pub fn reserve_mut(&mut self) {
        self.end = self.start;
        self.start = self.end;
        self.step = -self.step;
    }

    /// Returns total time span convered by this [TimeAxis] as a [Duration]
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    /// Defines a new [TimeAxis] with updated quantization
    pub fn quantized(&self, step: Duration) -> Self {
        let mut cloned = self.clone();
        cloned.step = step;

        if step.is_negative() {
            // revert
            cloned.end = cloned.start;
            cloned.start = self.start;
        }

        cloned
    }

    /// Creates a common [TimeAxis] from two different [TimeAxis] definitions.
    /// Stepping sign must agree.
    pub fn combined(&self, rhs: &Self) -> Result<Self, Error> {
        let forward = self.is_forward();

        if forward && !rhs.is_forward() {
            return Err(Error::IncompatibleTimeAxis);
        }

        if !forward && rhs.is_forward() {
            return Err(Error::IncompatibleTimeAxis);
        }

        Ok(Self {
            start: if forward {
                self.start.min(rhs.start)
            } else {
                self.start.max(rhs.start)
            },
            end: if forward {
                self.end.max(rhs.end)
            } else {
                self.end.min(rhs.end)
            },
            step: self.step.min(rhs.step),
            current: self.current, // wrong should be smallest common denominator
        })
    }
}

impl Iterator for TimeAxis {
    type Item = Epoch;

    /// Returns the next quantized instant as [Epoch].
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            // axis terminated
            None
        } else {
            self.current += self.step;

            // avoid overflowing the final epoch
            if self.step.is_negative() {
                self.current = self.current.max(self.end);
            } else {
                self.current = self.current.min(self.end);
            }

            Some(self.current)
        }
    }
}

impl DoubleEndedIterator for TimeAxis {
    /// Never ending quantized [Iterator] that will repeat itself
    fn next_back(&mut self) -> Option<Epoch> {
        if self.current == self.end {
            Some(self.start)
        } else {
            self.current += self.step;
            Some(self.current)
        }
    }
}
