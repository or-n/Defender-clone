use bevy::core_pipeline::bloom::BloomSettings;
use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use crate::{player::Player, utils};

pub fn spawn(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_translation(
                (utils::bevy::size(window) / 2.0).extend(0.0)
            ),
            ..default()
        },
        BloomSettings::default()
    ));
}

const CAMERA_OFFSET: f32 = 0.25 * 0.618;
const CAMERA_SPEED: f32 = 500.0;

pub fn follow_player(
    player_query: Query<(&Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok((transform, player)) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            let start = camera_transform.translation.x;
            let offset = player.facing.sign() * CAMERA_OFFSET * window.width();
            let end = transform.translation.x + offset;
            let amount = CAMERA_SPEED * time.delta_seconds();
            let x = utils::range::Range { start, end }.step(amount);
            camera_transform.translation.x = x;
        }
    }
}