use crate::flights::*;

pub struct DefaultFlightsPlugin;
impl bevy_app::Plugin for DefaultFlightsPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app
            .add_system(flight_system::<LinearFlight>)
            .add_system(flight_system::<LinearFlight3d>)
        ;
    }
}

use bevy_transform::prelude::Transform;
use bevy_ecs::prelude::*;
use bevy_time::Time;
use crate::flights::TrajectoryDescriptor;
pub fn flight_system<T: TrajectoryDescriptor + Component>(
    mut query: Query<(&mut Transform, &T)>,
    time_res: Res<Time>,
) {
    let time = time_res.elapsed_seconds();
    for (mut transform, trajectory) in query.iter_mut() {
        trajectory.update(time, &mut transform);
    }
}
