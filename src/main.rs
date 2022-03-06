#![feature(default_free_fn)]
use std::default::default;

use bevy::{
    input::{keyboard::KeyCode, Input},
    window::WindowMode,
    prelude::*,
};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component)]
struct Player {
    velocity: Vec2
}

impl Default for Player {
    fn default() -> Self {
        Player {
            velocity: Vec2::new(0., 0.)
        }
    }
}

const PLAYER_MAXSPEED: f32 = 300.;
const PLAYER_ACCEL: f32 = 2000.;
const DRAG_ACCEL: f32 = 1500.;

fn player_movement_system(
    mut query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
) {
    for (mut player, mut transform) in query.iter_mut() {
        let mut acceleration = Vec2::new(0., 0.);

        if keyboard.pressed(KeyCode::W) {
            acceleration.y += 1.;
        }

        if keyboard.pressed(KeyCode::S) {
            acceleration.y -= 1.;
        }

        if keyboard.pressed(KeyCode::D) {
            acceleration.x += 1.;
        }

        if keyboard.pressed(KeyCode::A) {
            acceleration.x -= 1.;
        }

        acceleration = acceleration.normalize_or_zero() * PLAYER_ACCEL;

        let speed = player.velocity.length();

        let drag = -player.velocity;
        let drag = drag.normalize_or_zero() * DRAG_ACCEL;
        let drag = drag.clamp_length_max(speed);

        acceleration += drag;
        acceleration *= time.delta_seconds();

        let accel = acceleration.length();

        if speed + accel > PLAYER_MAXSPEED {
            acceleration = acceleration.clamp_length_max(PLAYER_MAXSPEED - speed);
        }

        player.velocity += acceleration * time.delta_seconds();
        transform.translation += player.velocity.extend(0.) * time.delta_seconds();
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0.,
            0.,
            0.
        )))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            // vsync: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(player_movement_system)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1., 1., 1.),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        ..default()
    }).insert(Player::default());
}