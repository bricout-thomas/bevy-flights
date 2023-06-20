pub use crate::traits::TimeModifier;

pub struct Accelerate(pub f32);
impl TimeModifier for Accelerate {
    fn output(&self, s: f32) -> f32 {
        s * self.0
    }
}


pub struct TimeOffset(pub f32);
impl TimeModifier for TimeOffset {
    fn output(&self, s: f32) -> f32 {
        s - self.0
    }
}