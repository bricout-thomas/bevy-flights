use bevy_transform::prelude::Transform;
use bevy_math::{Vec2, Vec3, Quat};

use crate::wrappers::*;
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
        Translation2dDescriptorWrapper::from(self)
    }
    fn add<B: Translation2dDescriptor>(self, rhs: B) -> TranslationSum2d<Self, B> {
        TranslationSum2d::sum(self, rhs)
    }
}

/// Describes movement in 3Dimensional space
/// omitting rotation and scaling
pub trait Translation3dDescriptor: Sized + Sync + Send {
    /// The translation relative to time since startup
    fn translation(&self, t: f32) -> Vec3;
    fn wrap(self: Self) -> Translation3dDescriptorWrapper<Self> {
        Translation3dDescriptorWrapper::from(self)
    }
    fn add<B: Translation3dDescriptor>(self, rhs: B) -> TranslationSum3d<Self, B> {
        TranslationSum3d::sum(self, rhs)
    }
}

/// Describes a variable that changes over time, and according to its input
/// Works the same as TranslationDescriptors, expect they don't have this visual stuff
/// Though I guess it will be used to describes moving along an axis at some point
pub trait VariableDescriptor: Sized + Sync + Send {
    fn output(&self, t: f32) -> f32;
    /// serve as an easing function
    fn ease(&self, s: f32) -> f32 {
        if s < 0. { 0. }
        else if s > 1. { 1. }
        else { self.output(s) }
    }
}

/// describes rotation over time
pub trait RotationDescriptor: Sized + Sync + Send {
    /// The rotation relative to time since startup
    fn rotation(&self, t: f32) -> Quat;
}