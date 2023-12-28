use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{assets::GameAssets, map, style, utils};
use utils::bevy::{hit::*, projectile::Projectile, state::Simulation, window};

pub const SPEED: f32 = 2400.0 * 2.0;

#[derive(Bundle)]
pub struct Bundle {
    projectile: Projectile,
    sprite_bundle: SpriteBundle,
    scroll: map::Scroll,
}

impl Bundle {
    pub fn new(
        assets: &Res<GameAssets>,
        translation: Vec3,
        angle: f32,
        speed: f32,
        color: Color,
        is_laser: bool,
        is_damaging: bool,
    ) -> Bundle {
        let (bound, texture, rotation, scale) = if is_laser {
            (
                style::LASER_BOUND,
                assets.laser_texture.clone(),
                angle + 0.25,
                style::LASER_SCALE,
            )
        } else {
            (
                style::ORB_BOUND,
                assets.orb_texture.clone(),
                angle,
                style::ORB_SCALE,
            )
        };
        let transform = Transform {
            translation,
            rotation: utils::bevy::angle(rotation),
            scale: (scale * if is_damaging { 1.0 } else { 0.2 }).extend(1.0),
            ..default()
        };
        Bundle {
            projectile: Projectile {
                velocity: utils::bevy::clock(angle).extend(0.0) * speed,
                bound,
                is_damaging,
            },
            sprite_bundle: SpriteBundle {
                transform,
                texture,
                sprite: Sprite { color, ..default() },
                ..default()
            },
            scroll: map::Scroll,
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_outside_window, detect_hits).run_if(in_state(Simulation::Running)),
        );
    }
}

fn despawn_outside_window(
    query: Query<(Entity, &Transform), (With<Projectile>, Without<Camera>)>,
    camera_query: Query<&Transform, With<Camera>>,
    window_size: Res<window::Size>,
    mut commands: Commands,
) {
    for (entity, transform) in query.iter() {
        if let None = collide(
            camera_query.single().translation,
            window_size.0,
            transform.translation,
            style::LASER_BOUND,
        ) {
            commands.entity(entity).despawn();
        }
    }
}

fn detect_hits(
    query: Query<(Entity, &Transform, &Projectile)>,
    mut hittable_query: Query<(&Transform, &mut Hittable<Projectile>)>,
    camera_query: Query<&Transform, With<Camera>>,
    window_size: Res<window::Size>,
) {
    for (transform, mut hittable) in hittable_query.iter_mut() {
        for (entity, projectile_transform, projectile) in query.iter() {
            if !projectile.is_damaging {
                continue;
            }
            if hit(
                projectile_transform.translation,
                projectile.bound,
                transform.translation,
                hittable.hitbox,
                camera_query.single().translation,
                window_size.0,
            )
            .is_some()
            {
                hittable.hit_entity = Some(entity);
                break;
            }
        }
    }
}
