use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};
use super::state::Simulation;

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
    pub bound: Vec2,
    pub is_damaging: bool,
}

fn movement(
    mut query: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.velocity * time.delta_seconds();
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            movement.run_if(in_state(Simulation::Running))
        );
    }
}

pub fn hit(
    object_position: Vec3,
    object_bound: Vec2,
    projectile_position: Vec3,
    projectile: &Projectile,
    camera_position: Vec3,
    width: f32,
) -> bool {
    let camera_x = camera_position.x;
    let x = projectile_position.x;
    let bound_x = projectile.bound.x;
    let sub_r = ((x + bound_x) - (camera_x + width * 0.5)).max(0.0);
    let sub_l = ((camera_x - width * 0.5) - (x - bound_x)).max(0.0);
    collide(
        object_position,
        object_bound,
        projectile_position,
        projectile.bound - Vec2::X * (sub_l + sub_r)
    ).is_some()
}