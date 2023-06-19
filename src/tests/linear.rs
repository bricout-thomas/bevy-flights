use bevy_math::{Vec2, Vec3};
use bevy_transform::prelude::Transform;

use crate::prelude::*;

#[test]
fn test_from_velocity() {
    // test 2d flights
    let lf = LinearFlight::from_velocity(Vec2::X, Vec2::NEG_Y*10., 10.);
    let mut transform = Transform::default();
    lf.update(15., &mut transform);
    assert_eq!(transform, Transform::from_xyz(5., -10., 0.));
    // test 3d flights
    let lf = LinearFlight3d::from_velocity(Vec3::X, Vec3::NEG_Y*10., 10.);
    let mut transform = Transform::default();
    lf.update(15., &mut transform);
    assert_eq!(transform, Transform::from_xyz(5., -10., 0.));
}
