use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{assets::GameAssets, map, style, utils};
use utils::bevy::{hit::*, projectile::Projectile, state::Simulation, window};

pub const SPEED: f32 = 2400.0 * 2.0;

#[derive(Bundle)]
pub struct Bundle<T: Send + Sync + Component> {
    projectile: Projectile,
    sprite_bundle: SpriteBundle,
    scroll: map::Scroll,
    variant: T,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct Orb;

pub trait MyTransform {
    fn transform(angle: f32) -> Transform;
}

pub trait MyTexture {
    fn texture(assets: &GameAssets) -> Handle<Image>;
}

impl MyTransform for Laser {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle + 0.25))
            .with_scale(style::LASER_SCALE.extend(1.0))
    }
}

impl MyTexture for Laser {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.laser_texture.clone()
    }
}

impl MyTransform for Orb {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle)).with_scale(style::ORB_SCALE.extend(1.0))
    }
}

impl MyTexture for Orb {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.orb_texture.clone()
    }
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
                detect_hits::<Laser>,
                detect_hits::<Orb>,
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

impl Bound for Laser {
    fn bound() -> Vec2 {
        style::LASER_BOUND
    }
}

impl Bound for Orb {
    fn bound() -> Vec2 {
        style::ORB_BOUND
    }
}
