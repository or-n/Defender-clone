use super::state::Simulation;
use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub velocity: Vec3,
}

fn movement(mut query: Query<(&mut Transform, &Projectile)>, time: Res<Time>) {
    for (mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.velocity * time.delta_seconds();
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement.run_if(in_state(Simulation::Running)));
    }
}
