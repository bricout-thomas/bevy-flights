pub use crate::traits::TimeModifier;

pub struct Accelerate(f32);
impl TimeModifier for Accelerate {
    fn output(&self, s: f32) -> f32 {
        s * self.0
    }
}