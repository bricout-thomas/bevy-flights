use bevy_transform::prelude::Transform;
use bevy_math::Quat;

pub mod translations;
pub use translations::*;

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


pub use bevy_flights_derive::VariableDescriptor;
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