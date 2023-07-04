use bevy_ecs::prelude::Component;
use bevy_transform::prelude::Transform;

use crate::traits::*;

/// This wrapper is unfortunately necessary so as to avoid conflicting
/// implementations. This wrapper is completely cost free.
#[derive(Component)]
pub struct Translation2dDescriptorWrapper<T: Translation2dDescriptor>(pub T);
impl<T: Translation2dDescriptor> FlightComponent for Translation2dDescriptorWrapper<T> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        let result = self.0.translation(t);
        transform.translation.x = result.x;
        transform.translation.y = result.y;
    }
}

/// create using .wrap(). This wrapper is unfortunately necessary so as to avoid conflicting
/// implementations. This wrapper is completely cost free.
#[derive(Component)]
pub struct Translation3dDescriptorWrapper<T: Translation3dDescriptor>(pub T);
impl<T: Translation3dDescriptor> FlightComponent for Translation3dDescriptorWrapper<T> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        transform.translation = self.0.translation(t);
    }
}

/// Allows the modification of the Flight Component at runtime
/// Without having to actually change the Flight Component using bevy::prelude::Commands
#[derive(Component)]
pub struct RunTimeWrapper(pub dyn FlightComponent + Sync + Send);
impl FlightComponent for RunTimeWrapper {
    fn apply(&self, t: f32, transform: &mut Transform) {
        self.0.apply(t, transform);
    }
}
