use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
};

use std::f32::consts::TAU;

pub mod hit;
pub mod projectile;
pub mod state;
pub mod window;

pub fn grey(v: f32, alpha: f32) -> Color {
    Color::rgba(v, v, v, alpha)
}

pub fn bloom_hue(hue: f32) -> Color {
    Color::Hsla {
        hue,
        saturation: 1.0,
        lightness: 2.0,
        alpha: 1.0,
    }
}

pub fn volume(level: f32) -> Volume {
    Volume::Absolute(VolumeLevel::new(level))
}

pub fn angle(ratio: f32) -> Quat {
    Quat::from_rotation_z(ratio * TAU)
}

pub fn clock(ratio: f32) -> Vec2 {
    let scale = ratio * TAU;
    Vec2::new(scale.cos(), scale.sin())
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((state::Plug, projectile::Plug, window::Plug))
            .add_systems(PreUpdate, try_despawning)
            .add_systems(PostUpdate, (follow, try_despawning));
    }
}

#[derive(Component)]
pub struct Follow {
    pub entity: Entity,
    pub offset: Vec3,
}

fn follow(
    mut query: Query<(&mut Transform, &Follow)>,
    transform_query: Query<&Transform, Without<Follow>>,
) {
    for (mut transform, follow) in query.iter_mut() {
        if let Ok(new) = transform_query.get(follow.entity) {
            transform.translation = new.translation + follow.offset;
            transform.translation.z = -1.0;
        }
    }
}

#[derive(Component)]
pub struct DespawnTime {
    pub elapsed_seconds: f32,
}

fn try_despawning(mut commands: Commands, query: Query<(Entity, &DespawnTime)>, time: Res<Time>) {
    for (entity, despawn_time) in query.iter() {
        if time.elapsed_seconds() >= despawn_time.elapsed_seconds {
            commands.entity(entity).despawn_recursive();
        }
    }
}
