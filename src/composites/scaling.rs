use crate::prelude::Translation2dDescriptor;

/// Scales the result of a Translation2dDescriptor
pub struct Scale2d<T: Translation2dDescriptor> { pub scale: f32, pub descriptor: T } 
impl <T: Translation2dDescriptor> Translation2dDescriptor for Scale2d<T> {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        self.descriptor.translation(t) * self.scale
    }
}
impl<T: Translation2dDescriptor> Scale2d<T> {
    pub fn new(scale: f32, descriptor: T) -> Self {
        Self{ scale, descriptor }
    }
}