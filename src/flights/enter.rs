use bevy_math::Vec2;

use crate::{composites::{prelude::{Ease, Feed2d}, sums::TranslationSum2d}, prelude::{prelude::Sine, LinearFlight2d, Accelerate, TimeOffset, VariableDescriptor}};

/// Describes the movement of an entity entering the screen
/// vertically ( from the top or bottom )
/// according to an easing function

pub fn enter_vertically<O: VariableDescriptor, A: VariableDescriptor> (
    begin_height: f32,
    target_position: Vec2,
    time: O,
    speed: A,
) -> ScreenEnter<O, A> {
    let mut begin_position = target_position; begin_position.y = begin_height;
    let slope = target_position - begin_position;

    Feed2d::new(
        TimeOffset(time),
        Feed2d::new(
            Accelerate(speed),
            TranslationSum2d::sum(
                Feed2d::new(
                    Ease::<Sine>::default(), 
                    LinearFlight2d { slope }
                ),
                begin_position
            )
        )
    )
}

pub type ScreenEnter<O, A> = 
    Feed2d< 
        TimeOffset<O>, 
        Feed2d<
            Accelerate<A>,
            TranslationSum2d<
                Feed2d<
                    Ease<Sine>,
                    LinearFlight2d
                >,
                Vec2
            >
        >
    >
;