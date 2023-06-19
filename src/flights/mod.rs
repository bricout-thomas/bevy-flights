use bevy_transform::prelude::Transform;

pub trait TrajectoryDescriptor {
    fn update(&self, t: f32, transform: &mut Transform);
}

mod linear;

pub use linear::*;
