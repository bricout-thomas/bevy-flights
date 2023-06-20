use crate::prelude::{Translation2dDescriptor, Translation3dDescriptor};


/// Sums the result of the a and b
/// Allows for the creation of more complex movement
/// such as coming down on the screen while circling
pub struct TranslationSum2d<A: Translation2dDescriptor, B: Translation2dDescriptor>{ pub a: A, pub b: B,}
impl<A: Translation2dDescriptor, B: Translation2dDescriptor> Translation2dDescriptor for TranslationSum2d<A, B>
{
    fn translation(&self, t: f32) -> bevy_math::Vec2 {
        self.a.translation(t) + self.b.translation(t)
    }
}
impl<A: Translation2dDescriptor, B: Translation2dDescriptor> TranslationSum2d<A, B> {
    pub const fn sum(a: A, b: B) -> Self {
        TranslationSum2d { a, b }
    }
}

/// Sums the result of a and b
/// but in three dimensions!
pub struct TranslationSum3d<A: Translation3dDescriptor, B: Translation3dDescriptor>{ pub a: A, pub b: B,}
impl<A: Translation3dDescriptor, B: Translation3dDescriptor> Translation3dDescriptor for TranslationSum3d<A, B>
{
    fn translation(&self, t: f32) -> bevy_math::Vec3 {
        self.a.translation(t) + self.b.translation(t)
    }
}
impl<A: Translation3dDescriptor, B: Translation3dDescriptor> TranslationSum3d<A, B> {
    pub const fn sum(a: A, b: B) -> Self {
        Self { a, b }
    }
}