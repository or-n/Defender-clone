use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    assets::{GameAssets, MyTexture, MyTransform},
    map, style, utils,
};
use utils::bevy::{hit::*, projectile::Projectile, state::Simulation, window};

pub mod laser;
pub mod orb;

#[derive(Bundle)]
pub struct Bundle<T: Send + Sync + Component> {
    projectile: Projectile,
    sprite_bundle: SpriteBundle,
    scroll: map::Scroll,
    variant: T,
}

impl<T: Component + MyTexture + MyTransform> Bundle<T> {
    pub fn new(
        assets: &GameAssets,
        translation: Vec3,
        angle: f32,
        speed: f32,
        color: Color,
        variant: T,
    ) -> Self {
        Bundle {
            projectile: Projectile {
                velocity: utils::bevy::clock(angle).extend(0.0) * speed,
            },
            sprite_bundle: SpriteBundle {
                transform: T::transform(angle).with_translation(translation),
                texture: T::texture(assets),
                sprite: Sprite { color, ..default() },
                ..default()
            },
            scroll: map::Scroll,
            variant,
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_outside_window,
                detect_hits::<laser::Laser>,
                detect_hits::<orb::Orb>,
            )
                .run_if(in_state(Simulation::Running)),
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
