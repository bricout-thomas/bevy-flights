use bevy_ecs::prelude::Component;
use bevy_transform::prelude::Transform;
use bevy_math::{Vec3, Vec2, Quat};

use crate::composites::*;

/// Component trait to finally be inserted on a bullet
/// Make other traits such as Translation2dDescriptor into
/// this one using .wrap().
pub trait FlightComponent {
    fn apply(&self, t: f32, transform: &mut Transform);
    fn transform(&self, t: f32) -> Transform {
        let mut transform = Transform::default();
        self.apply(t, &mut transform);
        transform
    }
}

/// Describes movement on a two dimensional plane
/// omitting rotation and scaling
pub trait Translation2dDescriptor: Sized + Sync + Send {
    /// The translation relative to time since startup
    fn translation(&self, t: f32) -> Vec2;
    fn wrap(self) -> Translation2dDescriptorWrapper<Self> {
        Translation2dDescriptorWrapper(self)
    }
    fn add<B: Translation2dDescriptor>(self, rhs: B) -> TranslationSum2d<Self, B> {
        TranslationSum2d::new(self, rhs)
    }
}

/// Describes movement in 3Dimensional space
/// omitting rotation and scaling
pub trait Translation3dDescriptor: Sized + Sync + Send {
    /// The translation relative to time since startup
    fn translation(&self, t: f32) -> Vec3;
    fn wrap(self: Self) -> Translation3dDescriptorWrapper<Self> {
        Translation3dDescriptorWrapper(self)
    }
    fn add<B: Translation3dDescriptor>(self, rhs: B) -> TranslationSum3d<Self, B> {
        TranslationSum3d::new(self, rhs)
    }
}

/// describes rotation over time
pub trait RotationDescriptor: Sized + Sync + Send {
    /// The rotation relative to time since startup
    fn rotation(&self, t: f32) -> Quat;
}

/// create using .wrap(). This wrapper is unfortunately necessary so as to avoid conflicting
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

/// create using .wrap(). This wrapper is unfortunately necessary so as to avoid conflicting
/// implementations. This wrapper is completely cost free.
#[derive(Component)]
pub struct Translation3dDescriptorWrapper<T: Translation3dDescriptor>(T);
impl<T: Translation3dDescriptor> FlightComponent for Translation3dDescriptorWrapper<T> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        transform.translation = self.0.translation(t);
    }
}

mod fixed;
mod linear;
mod circle;

pub use circle::*;
pub use linear::*;
