#![allow(invalid_type_param_default)]

use crate::flights::prelude::*;

#[derive(Default)]
pub struct DefaultFlightsPlugin<Reference: TimeReference + Sync + Send + 'static = bevy_time::Time>
(std::marker::PhantomData<Reference>);
impl<Reference: TimeReference + Sync + Send + 'static> bevy_app::Plugin for DefaultFlightsPlugin<Reference> {
    fn build(&self, app: &mut bevy_app::App) {
        app
            .add_system(translation2d_system::<LinearFlight2d, Reference>)
            .add_system(translation3d_system::<LinearFlight3d, Reference>)
            .add_system(translation2d_system::<AffineFlight2d, Reference>)
            .add_system(translation3d_system::<AffineFlight3d, Reference>)

            .add_system(translation2d_system::<CircleFlight<f32, f32, f32>, Reference>)
        ;
    }
}

use bevy_time::Time;
use bevy_transform::prelude::Transform;
use bevy_ecs::prelude::*;

use crate::timereference::TimeReference;
use crate::traits::*;
use crate::wrappers::*;

pub fn flight_system<F: FlightComponent + Component, T: TimeReference + Resource = Time>(
    mut query: Query<(&mut Transform, &F)>,
    time_res: Res<T>,
) {
    let time = time_res.instant();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}

pub fn translation2d_system<F: Translation2dDescriptor + 'static, T: TimeReference + Resource = Time>(
    mut query: Query<(&mut Transform, &Translation2dDescriptorWrapper<F>)>,
    time_res: Res<T>,
) {
    let time = time_res.instant();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}

pub fn translation3d_system<F: Translation3dDescriptor + 'static, T: TimeReference + Resource = Time>(
    mut query: Query<(&mut Transform, &Translation3dDescriptorWrapper<F>)>,
    time_res: Res<T>,
) {
    let time = time_res.instant();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}
