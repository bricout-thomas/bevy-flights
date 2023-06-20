use bevy_math::Vec2;

use crate::prelude::{Accelerate, TimeOffset, VariableDescriptor};
use crate::traits::Translation2dDescriptor;
use crate::composites::prelude::{TranslationSum2d, Scale2d, Feed2d};

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

pub type CenteredCircleFlight<O, A, S> = 
    Feed2d< 
        TimeOffset<O>, 
        Feed2d<
            Accelerate<A>,
            Scale2d<S, UnitCircleFlight>
        >
    >
;
impl<O: VariableDescriptor, A: VariableDescriptor, S: VariableDescriptor> CenteredCircleFlight<O, A, S> {
    pub fn create(radius:S , frequency: A, time_offset: O) -> Self {
        Feed2d::new(
            TimeOffset(time_offset),
            Feed2d::new(
                Accelerate(frequency),
                Scale2d::new(
                    radius,
                    UnitCircleFlight::VALUE
                )
            )
        )
    }
    pub fn radius_mut(&mut self) -> &mut S {
        &mut self.descriptor.descriptor.scale
    }
    pub fn radius(&mut self) -> &S {
        &self.descriptor.descriptor.scale
    }
    pub fn frequency(&self) -> &A {
        &self.descriptor.modifier.0
    }
    pub fn time_offset(&self) -> &O {
        &self.modifier.0
    }
    pub fn time_offset_mut(&mut self) -> &mut O {
        &mut self.modifier.0
    }
}

/// A circle flight not centered around the origin
pub type CircleFlight<O, A, S> = 
    TranslationSum2d<
        Vec2, 
        CenteredCircleFlight<O, A, S>
    >
;
impl<O, A, S> CircleFlight<O, A, S> 
where S: VariableDescriptor, A: VariableDescriptor, O: VariableDescriptor {
    pub fn create(center: Vec2, radius: S, frequency: A, time_offset: O) -> Self {
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
    pub fn radius_mut(&mut self) -> &mut S {
        self.b.radius_mut()
    }
    pub fn radius(&mut self) -> &S {
        self.b.radius()
    }
    pub fn frequency(&self) -> &A {
        self.b.frequency()
    }
    pub fn time_offset(&self) -> &O {
        self.b.time_offset()
    }
    pub fn time_offset_mut(&mut self) -> &mut O {
        self.b.time_offset_mut()
    }
}
