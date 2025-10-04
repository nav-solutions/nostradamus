use crate::{Epoch, Frame, State};

#[derive(Debug, Default, Copy, Clone, PartialEq, PartialOrd)]
pub struct ClockState {
    offset: f64,
}

impl State for ClockState {
    fn default(_: Epoch) -> Self {
        Self {
            offset: Default::default(),
        }
    }

    fn random(_: Epoch) -> Self {
        Self {
            offset: Default::default(),
        }
    }

    fn epoch(&self) -> Epoch {
        Default::default()
    }

    fn set_epoch(&mut self, _: Epoch) {}

    fn temporal_update(&mut self, _: Epoch, temporal: &Self) {
        self = *temporal;
    }
}
