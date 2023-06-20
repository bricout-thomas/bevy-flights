use bevy_math::Vec2;

use super::fixed::FixedTranslation2d;

use crate::prelude::{Accelerate, TimeOffset};
use crate::traits::Translation2dDescriptor;
use crate::composites::{TranslationSum2d, Scale2d, Feed};

/// Corresponds to spinning in circles
/// around the origin
/// but only one the horizontal axis
pub struct HorizontalCircleFlight {
    pub radius: f32,
    pub frequency: f32,
    pub time_offset: f32,
}

impl Translation2dDescriptor for HorizontalCircleFlight {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        Vec2::X * ((t - self.time_offset)*self.frequency*std::f32::consts::TAU).cos() * self.radius
    }
}

// corresponds to spinning on circles,
// around the origin
// but only one the vertical axis.
pub struct VerticalCircleFlight {
    pub radius: f32,
    pub frequency: f32,
    pub time_offset: f32,
}

impl Translation2dDescriptor for VerticalCircleFlight {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        Vec2::Y * ((t - self.time_offset)*self.frequency*std::f32::consts::TAU).sin() * self.radius
    }
}

/// Describe a complete circle flight
/// around the origin
/// with frequency turns per second
/// time = 0 corresponds to being at the right of the circle
pub struct UnitCircleFlight;

impl Translation2dDescriptor for UnitCircleFlight {
    fn translation(&self, t: f32) -> Vec2 {
        let angle = t*std::f32::consts::TAU;
        Vec2::new(
            angle.cos(),
            angle.sin()
        )
    }
}

/// A circle flight not centered around the origin
pub type CircleFlight = 
    TranslationSum2d<
        FixedTranslation2d, 
        Feed< 
            TimeOffset, 
            Feed<
                Accelerate,
                Scale2d<UnitCircleFlight>
            >
        >
    >
;
impl CircleFlight {
    pub fn new(center: Vec2, radius: f32, frequency: f32, time_offset: f32) -> Self {
        Self::sum(
            FixedTranslation2d::new(center),
            Feed::new(
                TimeOffset(time_offset),
                Feed::new(
                    Accelerate(frequency),
                    Scale2d::new(
                        radius,
                        UnitCircleFlight
                    )))
        )
    }
}
