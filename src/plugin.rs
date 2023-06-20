use crate::flights::prelude::*;

pub struct DefaultFlightsPlugin;
impl bevy_app::Plugin for DefaultFlightsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app
            .add_system(translation2d_system::<LinearFlight, Time>)
            .add_system(translation3d_system::<LinearFlight3d, Time>)
        ;
    }
}

use bevy_time::Time;
use bevy_transform::prelude::Transform;
use bevy_ecs::prelude::*;

use crate::timereference::TimeReference;
use crate::traits::*;
use crate::wrappers::*;

pub fn flight_system<F: FlightComponent + Component, T: TimeReference + Resource>(
    mut query: Query<(&mut Transform, &F)>,
    time_res: Res<T>,
) {
    let time = time_res.instant();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}

pub fn translation2d_system<F: Translation2dDescriptor + 'static, T: TimeReference + Resource>(
    mut query: Query<(&mut Transform, &Translation2dDescriptorWrapper<F>)>,
    time_res: Res<T>,
) {
    let time = time_res.instant();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}

pub fn translation3d_system<F: Translation3dDescriptor + 'static, T: TimeReference + Resource>(
    mut query: Query<(&mut Transform, &Translation3dDescriptorWrapper<F>)>,
    time_res: Res<T>,
) {
    let time = time_res.instant();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}
