pub use crate::traits::VariableDescriptor;

impl VariableDescriptor for f32 {
    fn output(&self, _t: f32) -> f32 {
        *self
    }
}

pub struct Accelerate<V: VariableDescriptor>(pub V);
impl<V: VariableDescriptor> VariableDescriptor for Accelerate<V> {
    fn output(&self, s: f32) -> f32 {
        s * self.0.output(s)
    }
}


pub struct TimeOffset<V: VariableDescriptor>(pub V);
impl<V: VariableDescriptor> VariableDescriptor for TimeOffset<V> {
    fn output(&self, s: f32) -> f32 {
        s - self.0.output(s)
    }
}