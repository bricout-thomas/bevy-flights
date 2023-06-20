use bevy_math::Vec2;

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

pub type CenteredCircleFlight = 
    Feed< 
        TimeOffset, 
        Feed<
            Accelerate,
            Scale2d<UnitCircleFlight>
        >
    >
;
impl CenteredCircleFlight {
    pub fn create(radius: f32, frequency: f32, time_offset: f32) -> Self {
        Feed::new(
            TimeOffset(time_offset),
            Feed::new(
                Accelerate(frequency),
                Scale2d::new(
                    radius,
                    UnitCircleFlight::VALUE
                )
            )
        )
    }
    pub fn radius_mut(&mut self) -> &mut f32 {
        &mut self.descriptor.descriptor.scale
    }
    pub fn radius(&mut self) -> f32 {
        self.descriptor.descriptor.scale
    }
    pub fn frequency(&self) -> f32 {
        self.descriptor.modifier.0
    }
    pub fn time_offset(&self) -> f32 {
        self.modifier.0
    }
    pub fn time_offset_mut(&mut self) -> &mut f32 {
        &mut self.modifier.0
    }
}

/// A circle flight not centered around the origin
pub type CircleFlight = 
    TranslationSum2d<
        Vec2, 
        CenteredCircleFlight
    >
;
impl CircleFlight {
    pub fn create(center: Vec2, radius: f32, frequency: f32, time_offset: f32) -> Self {
        Self::sum(
            center,
            CenteredCircleFlight::create(radius, frequency, time_offset)
        )
    }
    pub fn center(&self) -> Vec2 {
        self.a
    }
    pub fn center_mut(&mut self) -> &mut Vec2 {
        &mut self.a
    }
    pub fn radius_mut(&mut self) -> &mut f32 {
        self.b.radius_mut()
    }
    pub fn radius(&mut self) -> f32 {
        self.b.radius()
    }
    pub fn frequency(&self) -> f32 {
        self.b.frequency()
    }
    pub fn time_offset(&self) -> f32 {
        self.b.time_offset()
    }
    pub fn time_offset_mut(&mut self) -> &mut f32 {
        self.b.time_offset_mut()
    }
}
