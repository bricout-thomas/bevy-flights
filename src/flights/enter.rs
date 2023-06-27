use bevy_math::Vec2;

use crate::{composites::{prelude::{Ease, Feed2d}, sums::TranslationSum2d}, prelude::{prelude::Sine, LinearFlight2d}};

/// Describes the movement of an entity entering the screen
/// vertically ( from the top or bottom )
/// according to an easing function

pub fn enter_vertically (
    begin_height: f32,
    target_position: Vec2,
) -> ScreenEnter {
    let mut begin_position = target_position; begin_position.y = begin_height;
    let slope = target_position - begin_position;

    TranslationSum2d::sum(
        Feed2d::new(
            Ease::<Sine>::default(), 
            LinearFlight2d { slope }
        ),
        begin_position
    )
}

pub type ScreenEnter = 
    TranslationSum2d<
        Feed2d<
            Ease<Sine>,
            LinearFlight2d
        >,
        Vec2
    >
;