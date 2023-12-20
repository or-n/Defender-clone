use bevy::core_pipeline::bloom::BloomSettings;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    player::{Player, HORIZONTAL_SPEED},
    utils,
};

pub fn spawn(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation((utils::bevy::size(window) / 2.0).extend(0.0)),
            ..default()
        },
        BloomSettings::default(),
    ));
}

const CAMERA_OFFSET: f32 = 0.618;
const CAMERA_SPEED: f32 = HORIZONTAL_SPEED * 1.25;

pub fn follow_player(
    player_query: Query<(&Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let window = window_query.single();
    if let Ok((transform, player)) = player_query.get_single() {
        let start = camera.translation.x;
        let speed = player.horizontal_speed.abs();
        let normal = (speed / HORIZONTAL_SPEED).min(1.0);
        let t = normal * normal;
        let o = CAMERA_OFFSET * 0.25 * (1.0 - t) + CAMERA_OFFSET * t;
        let offset = player.facing.sign() * o * window.width() * 0.5;
        let end = transform.translation.x + offset;
        let amount = CAMERA_SPEED * time.delta_seconds();
        let x = utils::range::Range { start, end }.step(amount);
        camera.translation.x = x;
    }
}
