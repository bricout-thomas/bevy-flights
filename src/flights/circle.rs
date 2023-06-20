use bevy_math::Vec2;

use super::fixed::FixedTranslation2d;

use crate::prelude::{Accelerate, TimeOffset};
use crate::traits::Translation2dDescriptor;
use crate::composites::{TranslationSum2d, Scale2d, Feed};

/// Corresponds to spinning in circles
/// around the origin
/// but only one the horizontal axis
pub struct HorizontalCircleFlight;
impl Translation2dDescriptor for HorizontalCircleFlight {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        Vec2::X * (std::f32::consts::TAU * t).cos()
    }
}

// corresponds to spinning on circles,
// around the origin
// but only one the vertical axis.
pub struct VerticalUnitCircleFlight;
impl Translation2dDescriptor for VerticalUnitCircleFlight {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        Vec2::Y * (std::f32::consts::TAU * t).sin()
    }
}

/// Describe a complete circle flight around the origin
/// time = 0 corresponds to being at the right of the circle
pub type UnitCircleFlight = TranslationSum2d<HorizontalCircleFlight, VerticalUnitCircleFlight>;
impl UnitCircleFlight { 
    const VALUE: Self = TranslationSum2d::sum(HorizontalCircleFlight, VerticalUnitCircleFlight);
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
                        UnitCircleFlight::VALUE
                    )))
        )
    }
}
