use crate::traits::VariableDescriptor;

/// describes a sine function
/// ease is overwritten
pub struct Sine;
impl VariableDescriptor for Sine {
    fn output(&self, t: f32) -> f32 {
        t.sin()
    }
    fn ease(&self, s: f32) -> f32 {
        if s > 1. { return 1. }
        else if s < 0. { return 0. }
        else { s.sin() }
    }
}

/// describes a sine function
pub struct Cos;
impl VariableDescriptor for Cos {
    fn output(&self, t: f32) -> f32 {
        t.cos()
    }
}