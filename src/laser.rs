use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};

use crate::{map, style, utils};
use utils::bevy::{projectile::Projectile, state::Simulation};

pub const SPEED: f32 = 2400.0;

#[derive(Bundle)]
pub struct Bundle {
    projectile: Projectile,
    sprite_bundle: SpriteBundle,
    scroll: map::Scroll,
}

pub fn sprite(
    asset_server: &Res<AssetServer>,
    position: Vec3,
    rotation: f32,
    color: Color,
    scale: f32,
) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: position,
            rotation: utils::bevy::angle(0.25 + rotation),
            scale: (style::LASER_SCALE * scale).extend(1.0),
            ..default()
        },
        texture: asset_server.load(style::LASER_TEXTURE),
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

pub fn orb_sprite(
    asset_server: &Res<AssetServer>,
    position: Vec3,
    rotation: f32,
    color: Color,
    scale: f32,
) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: position,
            rotation: utils::bevy::angle(rotation),
            scale: (style::ORB_SCALE * scale).extend(1.0),
            ..default()
        },
        texture: asset_server.load(style::ORB_TEXTURE),
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

impl Bundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        position: Vec3,
        angle: f32,
        speed: f32,
        color: Color,
        is_laser: bool,
        is_damaging: bool,
    ) -> Bundle {
        let scale = if is_damaging { 1.0 } else { 0.2 };
        Bundle {
            projectile: Projectile {
                velocity: utils::bevy::clock(angle).extend(0.0) * speed,
                bound: if is_laser {
                    style::LASER_BOUND
                } else {
                    style::ORB_BOUND
                },
                is_damaging,
            },
            sprite_bundle: if is_laser { sprite } else { orb_sprite }(
                &asset_server,
                position,
                angle,
                color,
                scale,
            ),
            scroll: map::Scroll,
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_outside_window.run_if(in_state(Simulation::Running)),
        );
    }
}

fn despawn_outside_window(
    query: Query<(Entity, &Transform), (With<Projectile>, Without<Camera>)>,
    camera_query: Query<&Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let camera_position = camera_query.get_single().unwrap().translation;
    let window_bound = utils::bevy::size(window);
    for (entity, transform) in query.iter() {
        if let None = collide(
            camera_position,
            window_bound,
            transform.translation,
            style::LASER_BOUND,
        ) {
            commands.entity(entity).despawn();
        }
    }
}
