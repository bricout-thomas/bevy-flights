use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};

use crate::{traits::{Translation2dDescriptor, Translation3dDescriptor}, composites::prelude::{TranslationSum2d, TranslationSum3d}};

// Describes moving in a continuous linear flight
// that crosses the origin
#[derive(Component, Default)]
pub struct LinearFlight2d {
    pub slope: Vec2,
}

impl Translation2dDescriptor for LinearFlight2d {
    fn translation(&self, t: f32) -> Vec2 { self.slope * t }
}

#[derive(Component, Default)]
pub struct LinearFlight3d {
    pub slope: Vec3,
}

impl Translation3dDescriptor for LinearFlight3d {
    fn translation(&self, t: f32) -> Vec3 { self.slope * t }
}

pub type AffineFlight2d = TranslationSum2d<Vec2, LinearFlight2d>;

impl AffineFlight2d {
    pub fn from_velocity(velocity: Vec2, initial_position: Vec2, initial_time: f32) -> Self {
        TranslationSum2d::sum( 
            initial_position - velocity * initial_time,
            LinearFlight2d { slope: velocity },
        )
    }
}

pub type AffineFlight3d = TranslationSum3d<Vec3, LinearFlight3d>;

impl AffineFlight3d {
    pub fn from_velocity(velocity: Vec3, initial_position: Vec3, initial_time: f32) -> Self {
        TranslationSum3d::sum( 
            initial_position - velocity * initial_time,
            LinearFlight3d { slope: velocity },
        )
    }
}