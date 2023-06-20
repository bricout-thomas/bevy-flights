use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};

use crate::{traits::{Translation2dDescriptor, Translation3dDescriptor}, composites::{TranslationSum2d, TranslationSum3d}, prelude::{FixedTranslation2d, FixedTranslation3d}};

// Describes moving in a continuous linear flight
// that crosses the origin
#[derive(Component)]
pub struct LinearFlight2d {
    pub slope: Vec2,
}

impl Translation2dDescriptor for LinearFlight2d {
    fn translation(&self, t: f32) -> Vec2 { self.slope * t }
}

#[derive(Component)]
pub struct LinearFlight3d {
    pub slope: Vec3,
}

impl Translation3dDescriptor for LinearFlight3d {
    fn translation(&self, t: f32) -> Vec3 { self.slope * t }
}

pub type AffineFlight2d = TranslationSum2d<FixedTranslation2d, LinearFlight2d>;

impl AffineFlight2d {
    pub fn from_velocity(velocity: Vec2, initial_position: Vec2, initial_time: f32) -> Self {
        TranslationSum2d::sum( 
            FixedTranslation2d::new(initial_position - velocity * initial_time),
            LinearFlight2d { slope: velocity },
        )
    }
}

pub type AffineFlight3d = TranslationSum3d<FixedTranslation3d, LinearFlight3d>;

impl AffineFlight3d {
    pub fn from_velocity(velocity: Vec3, initial_position: Vec3, initial_time: f32) -> Self {
        TranslationSum3d::sum( 
            FixedTranslation3d::new(initial_position - velocity * initial_time),
            LinearFlight3d { slope: velocity },
        )
    }
}