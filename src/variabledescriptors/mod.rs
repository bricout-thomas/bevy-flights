pub use crate::traits::VariableDescriptor;

impl VariableDescriptor for f32 {
    fn output(&self, _t: f32) -> f32 {
        *self
    }
}

pub struct Accelerate(pub f32);
impl VariableDescriptor for Accelerate {
    fn output(&self, s: f32) -> f32 {
        s * self.0
    }
}


pub struct TimeOffset(pub f32);
impl VariableDescriptor for TimeOffset {
    fn output(&self, s: f32) -> f32 {
        s - self.0
    }
}