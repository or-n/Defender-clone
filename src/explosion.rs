use crate::{
    assets::{audio, GameAssets, MyTexture, MyTransform},
    projectile, style, utils,
};
use bevy::prelude::*;

#[derive(Event)]
pub struct At {
    pub position: Vec3,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, try_spawning);
    }
}

#[derive(Component)]
pub struct Explosion;

impl MyTransform for Explosion {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle + 0.25))
            .with_scale((style::ORB_SCALE * 0.5).extend(1.0))
    }
}

impl MyTexture for Explosion {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.laser_texture.clone()
    }
}

fn try_spawning(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut event: EventReader<At>,
) {
    for explosion in event.read() {
        commands.spawn(audio(
            assets.collision_audio.clone(),
            style::EXPLOSION_VOLUME,
        ));
        let n = 16;
        let speed = 400.0;
        for i in 0..n {
            let angle = i as f32 / (n - 1) as f32;
            let clock = utils::bevy::clock(angle).extend(0.0);
            let hue = rand::random::<f32>() * 360.0;
            commands.spawn((
                projectile::Bundle::new(
                    &assets,
                    explosion.position + clock * style::ORB_BOUND.x * 0.5,
                    angle,
                    speed,
                    utils::bevy::bloom_hue(hue),
                    Explosion,
                ),
                utils::bevy::DespawnTime {
                    elapsed_seconds: time.elapsed_seconds() + 0.5,
                },
            ));
        }
    }
}
