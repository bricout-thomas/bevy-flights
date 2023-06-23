use std::time::Duration;

use bevy_flights::{prelude::*, composites::sums::TranslationSum2d};
use bevy::prelude::*;
use bevy_time::common_conditions::on_timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)

        .add_plugin(DefaultFlightsPlugin::<Time>::default())

        .add_system(translation2d_system::<CoolFlight>)
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
                CenteredCircleFlight::create(500., 0.1, time.elapsed_seconds()),
                CenteredCircleFlight::create(100., 1., time.elapsed_seconds())
            ).wrap()
        )
    ;
}

pub type CoolFlight = 
    TranslationSum2d<
        CenteredCircleFlight<f32, f32, f32>,
        CenteredCircleFlight<f32, f32, f32>
    >
;

fn camera(
    mut commands:Commands,
) {
    commands.spawn(
        Camera2dBundle::default()
    );
}