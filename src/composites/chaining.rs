use crate::prelude::{VariableDescriptor, Translation2dDescriptor, Translation3dDescriptor};

/// Feeds the result of the time modifier function into the descriptor 2d
pub struct Feed2d<E: VariableDescriptor, T: Translation2dDescriptor> { pub modifier: E, pub descriptor: T }
impl<E: VariableDescriptor, T: Translation2dDescriptor> Translation2dDescriptor for Feed2d<E, T> {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        self.descriptor.translation(self.modifier.output(t))
    }
}
impl<E: VariableDescriptor, T: Translation2dDescriptor> Feed2d<E, T> {
    pub fn new(modifier: E, descriptor: T) -> Self {
        Self {
            modifier,
            descriptor
        }
    }
}

/// Feeds the result of the time modifier function into the descriptor 3d
pub struct Feed3d<E: VariableDescriptor, T: Translation3dDescriptor> { pub modifier: E, pub descriptor: T }
impl<E: VariableDescriptor, T: Translation3dDescriptor> Translation3dDescriptor for Feed3d<E, T> {
    fn translation(&self, t: f32) -> bevy_math::Vec3 {
        self.descriptor.translation(self.modifier.output(t))
    }
}
impl<E: VariableDescriptor, T: Translation3dDescriptor> Feed3d<E, T> {
    pub fn new(modifier: E, descriptor: T) -> Self {
        Self {
            modifier,
            descriptor
        }
    }
}

/// Feeds the result of the time modifier function into the variable descriptor
pub struct Feed<E: VariableDescriptor, T: VariableDescriptor> { pub modifier: E, pub descriptor: T }
impl<E: VariableDescriptor, T: VariableDescriptor> VariableDescriptor for Feed<E, T> {
    fn output(&self, t: f32) -> f32 {
        self.descriptor.output(self.modifier.output(t))
    }
}
impl<E: VariableDescriptor, T: VariableDescriptor> Feed<E, T> {
    pub fn new(modifier: E, descriptor: T) -> Self {
        Self {
            modifier,
            descriptor
        }
    }
}

/// A wrapper that makes the ease function the default
#[derive(Default)]
pub struct Ease<E: VariableDescriptor> { pub modifier: E }
impl<E: VariableDescriptor> VariableDescriptor for Ease<E> {
    fn output(&self, t: f32) -> f32 {
        self.modifier.ease(t)
    }
}