pub use crate::traits::VariableDescriptor;

impl<T: Clone + Into<f32> + Sync + Send + Sized> VariableDescriptor for T {
    fn output(&self, _t: f32) -> f32 {
        self.clone().into()
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

pub mod sine;

pub mod prelude {
    pub use super::*;
    pub use super::sine::*;
}