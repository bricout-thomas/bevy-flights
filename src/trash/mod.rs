use bevy_transform::prelude::Transform;
use bevy_ecs::prelude::Component;

use crate::flights::FlightDescriptor;

/// This struct first applies A then B. if B works by setting some fields and leaving some others
/// alone, this will result in B overwriting some modifications done by A and leaving the rest
/// alone. This shouldn't result in worse performance if compiled with optimizations flags because
/// the associated function will compile removing the function calls to A and B. though testing is
/// needed to confirm this.
#[derive(Component)]
pub struct OverWrite<A, B>
where 
    A: FlightDescriptor,
    B: FlightDescriptor,
{
    a: A,
    b: B,
}

impl<A: FlightDescriptor, B: FlightDescriptor> FlightDescriptor for OverWrite<A, B> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        self.a.apply(t, transform);
        self.b.apply(t, transform);
    }
}

/// This struct applies the result of A followed by the result of B.
/// The result is a little bit like A is inserted on a parent entity of B
/// If there are lots of entities with the same A, it is more efficient to use a parent Entity
/// Unfortunately, This deleted all info about the former transform
#[derive(Component)]
pub struct Follow<A: FlightDescriptor, B: FlightDescriptor> {
    a: A,
    b: B,
}

impl<A: FlightDescriptor, B: FlightDescriptor> FlightDescriptor for Follow<A, B> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        *transform = self.a.transform(t).mul_transform(self.b.transform(t))
    }
}

/// This struct adds up two translations from flight descriptors
/// And changes the x and y coordinates
#[derive(Component)]
pub struct AddTranslations2d<A: FlightDescriptor, B: FlightDescriptor> {
    a: A,
    b: B,
}

impl<A: FlightDescriptor, B: FlightDescriptor> FlightDescriptor for AddTranslations2d<A, B> {
    fn apply(&self, t: f32, transform: &mut Transform) {
        let result = self.a.transform(t).translation + self.b.transform(t).translation;
        transform.translation.x = result.x;
        transform.translation.y = result.y;
    }
}
