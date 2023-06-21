use crate::prelude::{Translation2dDescriptor, VariableDescriptor, Translation3dDescriptor};

/// Scales the result of a Translation2dDescriptor
pub struct Scale2d<S: VariableDescriptor, T: Translation2dDescriptor> { pub scale: S, pub descriptor: T } 
impl <S: VariableDescriptor, T: Translation2dDescriptor> Translation2dDescriptor for Scale2d<S, T> {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        self.descriptor.translation(t) * self.scale.output(t)
    }
}
impl<S: VariableDescriptor, T: Translation2dDescriptor> Scale2d<S, T> {
    pub fn new(scale: S, descriptor: T) -> Self {
        Self{ scale, descriptor }
    }
}

/// Scales the result of a Translation3dDescriptor
pub struct Scale3d<S: VariableDescriptor, T: Translation3dDescriptor> { pub scale: S, pub descriptor: T } 
impl <S: VariableDescriptor, T: Translation3dDescriptor> Translation3dDescriptor for Scale3d<S, T> {
    fn translation(&self, t: f32) -> bevy_math::Vec3 {
        self.descriptor.translation(t) * self.scale.output(t)
    }
}
impl<S: VariableDescriptor, T: Translation3dDescriptor> Scale3d<S, T> {
    pub fn new(scale: S, descriptor: T) -> Self {
        Self{ scale, descriptor }
    }
}