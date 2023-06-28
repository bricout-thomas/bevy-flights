use std::time::Duration;

use bevy_flights::{prelude::*, composites::sums::TranslationSum2d, variabledescriptors::prelude::{ONE, ZERO}};
use bevy::prelude::*;
use bevy_time::common_conditions::on_timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)

        .add_plugin(DefaultFlightsPlugin::<Time>::default())

        .add_system(translation2d_system::<Toff2d<CoolFlight>>)
        .add_system(spawn_cool_flight_bullet.run_if(on_timer(Duration::from_secs_f32(0.005))))

        .run()
    ;
}

fn spawn_cool_flight_bullet(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("bullet.png"),
            ..default()
        }
    )
        .insert(
            CoolFlight::sum(
                CenteredCircleFlight::create(BIGRADIUS, SMALLFREQUENCY, ZERO),
                CenteredCircleFlight::create(SMALLRADIUS, ONE, ZERO)
            ).wrap_with_offset(time.elapsed_seconds())
        )
    ;
}

#[derive(VariableDescriptor)]
#[value = 500.]
struct BIGRADIUS;

#[derive(VariableDescriptor)]
#[value = 100.]
struct SMALLRADIUS;

#[derive(VariableDescriptor)]
#[value = 0.1]
struct SMALLFREQUENCY;

type CoolFlight = 
    TranslationSum2d<
        CenteredCircleFlight<ZERO, SMALLFREQUENCY, BIGRADIUS>,
        CenteredCircleFlight<ZERO, ONE, SMALLRADIUS>
    >
;

fn camera(
    mut commands:Commands,
) {
    commands.spawn(
        Camera2dBundle::default()
    );
}