use bevy_math::{Vec2, Vec3};
use crate::traits::{Translation2dDescriptor, Translation3dDescriptor};

impl Translation2dDescriptor for Vec2 {
    fn translation(&self, _t: f32) -> Vec2 {
        *self
    }
}

impl Translation3dDescriptor for Vec3 {
    fn translation(&self, _t: f32) -> Vec3 {
        *self
    }
}