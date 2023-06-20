use bevy_math::{Vec2, Vec3};

use crate::traits::{Translation2dDescriptor, Translation3dDescriptor};

pub struct FixedTranslation2d {
    translation: Vec2,
}

impl Translation2dDescriptor for FixedTranslation2d {
    fn translation(&self, _t: f32) -> Vec2 {
        self.translation
    }
}

impl FixedTranslation2d {
    pub fn new(translation: Vec2) -> Self {
        Self {
            translation,
        }
    }
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self::new(Vec2::new(x, y))
    }
}

pub struct FixedTranslation3d {
    translation: Vec3,
}

impl Translation3dDescriptor for FixedTranslation3d {
    fn translation(&self, _t: f32) -> Vec3 {
        self.translation
    }
}

impl FixedTranslation3d {
    pub fn new(translation: Vec3) -> Self {
        Self {
            translation,
        }
    }
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self::new(Vec3::new(x, y, z))
    }
}
