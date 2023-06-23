use bevy_math::{Vec2, Vec3};

use crate::traits::*;

pub struct Compose2d<X: VariableDescriptor, Y: VariableDescriptor> { x: X, y: Y }
impl<X: VariableDescriptor, Y: VariableDescriptor> Translation2dDescriptor for Compose2d<X, Y> {
    fn translation(&self, t: f32) -> Vec2 {
        Vec2::new(self.x.output(t), self.y.output(t))
    }
}
impl<X: VariableDescriptor, Y: VariableDescriptor> Compose2d<X, Y> {
    pub const fn new(x: X, y: Y) -> Self {
        Self {
            x, y
        }
    }
}

pub struct Compose3d<X: VariableDescriptor, Y: VariableDescriptor, Z: VariableDescriptor> { x: X, y: Y, z: Z }
impl<X: VariableDescriptor, Y: VariableDescriptor, Z: VariableDescriptor> Translation3dDescriptor for Compose3d<X, Y, Z> {
    fn translation(&self, t: f32) -> Vec3 {
        Vec3::new(self.x.output(t), self.y.output(t), self.z.output(t))
    }
}
impl<X: VariableDescriptor, Y: VariableDescriptor, Z: VariableDescriptor> Compose3d<X, Y, Z> {
    pub const fn new(x: X, y: Y, z: Z) -> Self {
        Self { x, y, z }
    }
}