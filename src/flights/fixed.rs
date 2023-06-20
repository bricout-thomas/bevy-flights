use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};

use super::{Translation2dDescriptor, Translation3dDescriptor};

struct FixedTranslation2d {
    translation: Vec2,
}

impl Translation2dDescriptor for FixedTranslation2d {
    fn translation(&self, _t: f32) -> Vec2 {
        self.translation
    }
}

impl FixedTranslation2d {
    fn new(translation: Vec2) -> Self {
        Self {
            translation,
        }
    }
}

struct FixedTranslation3d {
    translation: Vec3,
}

impl Translation3dDescriptor for FixedTranslation3d {
    fn translation(&self, _t: f32) -> Vec3 {
        self.translation
    }
}

impl FixedTranslation3d {
    fn new(translation: Vec3) -> Self {
        Self {
            translation,
        }
    }
}
