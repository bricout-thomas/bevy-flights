use bevy_math::Vec2;

use super::fixed::FixedTranslation2d;

use crate::traits::Translation2dDescriptor;
use crate::composites::TranslationSum2d;

/// Corresponds to spinning in circles
/// around the origin
/// but only one the horizontal axis
pub struct HorizontalCircleFlight {
    radius: f32,
    frequency: f32,
    time_offset: f32,
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
    radius: f32,
    frequency: f32,
    time_offset: f32,
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
pub struct CircleFlight {
    radius: f32,
    frequency: f32,
    time_offset: f32,
}

impl Translation2dDescriptor for CircleFlight {
    fn translation(&self, t: f32) -> Vec2 {
        let angle = (t - self.time_offset)*self.frequency*std::f32::consts::TAU;
        Vec2::new(
            angle.cos() * self.radius,
            angle.sin() * self.radius
        )
    }
}

/// A circle flight not centered around the origin
type OffsetCircleFlight = TranslationSum2d<FixedTranslation2d, CircleFlight>;
impl OffsetCircleFlight {
    pub fn new(center: Vec2, radius: f32, frequency: f32, time_offset: f32) -> Self {
        Self::sum(
            FixedTranslation2d::new(center),
            CircleFlight {
                radius,
                frequency,
                time_offset,
            }
        )
    }
}
