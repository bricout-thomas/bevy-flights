use bevy_math::Vec2;

use crate::prelude::*;

#[test]
fn offset_circle() {
    let center = Vec2::new(1., 0.);
    let flight = CircleFlight::create(center, 1., 1., 0.);
    
    // cannot use assert_eq! due to f32's precision
    println!("{:?}", flight.translation(0.));
    assert!(flight.translation(0.).distance(Vec2::new(2., 0.)) < 0.001);
    println!("{:?}", flight.translation(25.));
    assert!(flight.translation(0.25).distance(Vec2::new(1., 1.)) < 0.001);
    assert!(flight.translation(0.50).distance(Vec2::new(0., 0.)) < 0.001);
    assert!(flight.translation(0.75).distance(Vec2::new(1., -1.)) < 0.001);

    let flight = CircleFlight::create(center, 2., 0.5, 0.189);

    println!("{:?}", flight.translation(0.189));
    assert!(flight.translation(0.189).distance(Vec2::new(3., 0.)) < 0.001);
    assert!(flight.translation(0.189 + 0.5).distance(Vec2::new(1., 2.)) < 0.001);
}
