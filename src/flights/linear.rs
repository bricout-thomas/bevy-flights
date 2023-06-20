use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};

use crate::traits::{Translation2dDescriptor, Translation3dDescriptor};

// Describes moving in a continuous linear flight
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

impl Translation3dDescriptor for LinearFlight3d {
    fn translation(&self, t: f32) -> Vec3 {
        self.at_origin + self.slope * t
    }
}

impl Translation2dDescriptor for LinearFlight {
    fn translation(&self, t: f32) -> Vec2 {
        self.at_origin + self.slope * t
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
