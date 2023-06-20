use bevy_math::{Vec2, Vec3};

use crate::prelude::*;

#[test]
fn test_from_velocity() {
    // test 2d flights
    let lf = LinearFlight::from_velocity(Vec2::X, Vec2::NEG_Y*10., 10.);
    assert_eq!(lf.translation(15.), Vec2::new(5., -10.));
    // test 3d flights
    let lf = LinearFlight3d::from_velocity(Vec3::X, Vec3::NEG_Y*10., 10.);
    assert_eq!(lf.translation(15.), Vec3::new(5., -10., 0.));
}
