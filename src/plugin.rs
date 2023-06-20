use crate::flights::prelude::*;

pub struct DefaultFlightsPlugin;
impl bevy_app::Plugin for DefaultFlightsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app
            .add_system(translation2d_system::<LinearFlight>)
            .add_system(translation3d_system::<LinearFlight3d>)
        ;
    }
}

use bevy_transform::prelude::Transform;
use bevy_ecs::prelude::*;
use bevy_time::Time;

use crate::traits::*;
use crate::wrappers::*;

pub fn flight_system<T: FlightComponent + Component>(
    mut query: Query<(&mut Transform, &T)>,
    time_res: Res<Time>,
) {
    let time = time_res.elapsed_seconds();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}

pub fn translation2d_system<T: Translation2dDescriptor + 'static>(
    mut query: Query<(&mut Transform, &Translation2dDescriptorWrapper<T>)>,
    time_res: Res<Time>,
) {
    let time = time_res.elapsed_seconds();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}

pub fn translation3d_system<T: Translation3dDescriptor + 'static>(
    mut query: Query<(&mut Transform, &Translation3dDescriptorWrapper<T>)>,
    time_res: Res<Time>,
) {
    let time = time_res.elapsed_seconds();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.apply(time, &mut transform);
    }
}
