use bevy_ecs::prelude::Component;
use bevy_transform::prelude::Transform;

use crate::traits::*;

/// This wrapper is unfortunately necessary so as to avoid conflicting
/// implementations. This wrapper is completely cost free.
#[derive(Component)]
pub struct Translation2dDescriptorWrapper<T: Translation2dDescriptor>(T);
impl<T: Translation2dDescriptor> FlightComponent for Translation2dDescriptorWrapper<T> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        let result = self.0.translation(t);
        transform.translation.x = result.x;
        transform.translation.y = result.y;
    }
}
impl<T: Translation2dDescriptor> From<T> for Translation2dDescriptorWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// create using .wrap(). This wrapper is unfortunately necessary so as to avoid conflicting
/// implementations. This wrapper is completely cost free.
#[derive(Component)]
pub struct Translation3dDescriptorWrapper<T: Translation3dDescriptor>(T);
impl<T: Translation3dDescriptor> FlightComponent for Translation3dDescriptorWrapper<T> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        transform.translation = self.0.translation(t);
    }
}
impl<T: Translation3dDescriptor> From<T> for Translation3dDescriptorWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
