use crate::{assets::GameAssets, laser, style, utils};
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

fn try_spawning(
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    mut event: EventReader<At>,
) {
    for explosion in event.read() {
        commands.spawn(AudioBundle {
            source: assets.collision_audio.clone(),
            settings: PlaybackSettings::DESPAWN
                .with_volume(utils::bevy::volume(style::EXPLOSION_VOLUME)),
        });
        let n = 16;
        let speed = 400.0;
        for i in 0..n {
            let angle = i as f32 / (n - 1) as f32;
            let clock = utils::bevy::clock(angle).extend(0.0);
            let hue = rand::random::<f32>() * 360.0;
            commands.spawn((
                laser::Bundle::new(
                    &assets,
                    explosion.position + clock * style::ORB_BOUND.x * 0.5,
                    angle,
                    speed,
                    utils::bevy::bloom_hue(hue),
                    false,
                    false,
                ),
                utils::bevy::DespawnTime {
                    elapsed_seconds: time.elapsed_seconds() + 0.5,
                },
            ));
        }
    }
}
