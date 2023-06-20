use crate::prelude::{Translation2dDescriptor, Translation3dDescriptor, TimeModifier};

/// Sums the result of the a and b
/// Allows for the creation of more complex movement
/// such as coming down on the screen while circling
pub struct TranslationSum2d<A: Translation2dDescriptor, B: Translation2dDescriptor>{ a: A, b: B,}
impl<A: Translation2dDescriptor, B: Translation2dDescriptor> Translation2dDescriptor for TranslationSum2d<A, B>
{
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        self.a.translation(t) + self.b.translation(t)
    }
}
impl<A: Translation2dDescriptor, B: Translation2dDescriptor> TranslationSum2d<A, B> {
    pub fn sum(a: A, b: B) -> Self {
        TranslationSum2d { a, b }
    }
}

/// Sums the result of a and b
/// but in three dimensions!
pub struct TranslationSum3d<A: Translation3dDescriptor, B: Translation3dDescriptor>{ a: A, b: B,}
impl<A: Translation3dDescriptor, B: Translation3dDescriptor> Translation3dDescriptor for TranslationSum3d<A, B>
{
    fn translation(&self, t: f32) -> bevy_math::Vec3 {
        self.a.translation(t) + self.b.translation(t)
    }
}
impl<A: Translation3dDescriptor, B: Translation3dDescriptor> TranslationSum3d<A, B> {
    pub fn sum(a: A, b: B) -> Self {
        Self { a, b }
    }
}

/// Feeds the result of the time modifier function into the descriptor
pub struct Feed<E: TimeModifier, T: Translation2dDescriptor> { pub modifier: E, pub descriptor: T }
impl<E: TimeModifier, T: Translation2dDescriptor> Translation2dDescriptor for Feed<E, T> {
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        self.descriptor.translation(self.modifier.output(t))
    }
}
impl<E: TimeModifier, T: Translation2dDescriptor> Feed<E, T> {
    pub fn new(modifier: E, descriptor: T) -> Self {
        Self {
            modifier,
            descriptor
        }
    }
}

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