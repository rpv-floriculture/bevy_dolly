use super::{RigDriver};
use bevy::prelude::*;

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug)]
pub struct Arm {
    pub offset: Vec3,
}

impl Arm {
    pub fn new(offset: Vec3) -> Self {
        Self { offset }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, transform: &mut Transform, _delta_time_seconds: f32) {
        transform.translation += transform.rotation * self.offset;
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
