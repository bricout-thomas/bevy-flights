use bevy_ecs::prelude::Component;
use bevy_transform::prelude::Transform;
use bevy_math::{Vec2, Vec3};

use super::TrajectoryDescriptor;

#[derive(Component)]
pub struct LinearFlight {
    at_origin: Vec2,
    slope: Vec2,
}

#[derive(Component)]
pub struct LinearFlight3d {
    at_origin: Vec3,
    slope: Vec3,
}

impl TrajectoryDescriptor for LinearFlight3d {
    fn update(&self, t: f32, transform: &mut Transform) {
        *transform = transform.with_translation(self.at_origin + self.slope * t)
    }
}

impl TrajectoryDescriptor for LinearFlight {
    fn update(&self, t: f32, transform: &mut Transform) {
        *transform = transform.with_translation((self.at_origin + self.slope * t).extend(transform.translation.z))
    }
}

impl LinearFlight3d {
    pub fn from_velocity(velocity: Vec3, initial_position: Vec3, initial_time: f32) -> Self {
        LinearFlight3d {
            slope: velocity,
            at_origin: initial_position - velocity * initial_time,
        }
    }
}

impl LinearFlight {
    pub fn from_velocity(velocity: Vec2, initial_position: Vec2, initial_time: f32) -> Self {
        LinearFlight { 
            slope: velocity,
            at_origin: initial_position - velocity * initial_time, 
        }
    }
}
