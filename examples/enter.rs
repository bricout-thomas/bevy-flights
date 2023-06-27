use std::time::Duration;

use bevy::prelude::*;
use bevy_flights::{prelude::*, variabledescriptors::prelude::ONE};
use bevy_time::common_conditions::on_timer;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)

        .add_plugin(bevy_flights::plugin::DefaultFlightsPlugin::<Time>::default())
        .add_system(spawn_entering_bullets.run_if(on_timer(Duration::from_secs_f32(0.1))))
    ;
}

fn spawn_entering_bullets(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: ResMut<AssetServer>,
) {
    let target_position = Vec2::new(random(), random());
    let begin_height = 
        if random::<bool>() { -1. } else { 1. } *
        random::<f32>() * 100. + 500.
    ;

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("bullet.png"),
            ..default()
        }
    ).insert(
        enter_vertically(begin_height, target_position, time.elapsed_seconds(), ONE).wrap()
    )
    ;
}

fn camera(
    mut commands:Commands,
) {
    commands.spawn(
        Camera2dBundle::default()
    );
}