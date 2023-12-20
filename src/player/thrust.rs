use bevy::prelude::*;

use super::{input, Player};
use crate::{assets::GameAssets, style, utils};

#[derive(Component)]
pub struct Thrust {
    volume: f32,
}

#[derive(Bundle)]
pub struct ThrustBundle {
    audio: AudioBundle,
    thrust: Thrust,
}

impl ThrustBundle {
    pub fn new(assets: &GameAssets) -> ThrustBundle {
        ThrustBundle {
            audio: AudioBundle {
                source: assets.thrust_audio.clone(),
                settings: PlaybackSettings::LOOP.with_volume(utils::bevy::volume(0.0)),
            },
            thrust: Thrust { volume: 0.0 },
        }
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_particle, set_volume));
    }
}

fn spawn_particle(
    player_query: Query<(Entity, &Transform, &Player)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    controls: Res<input::Controls>,
) {
    if let Ok((player_entity, transform, player)) = player_query.get_single() {
        if controls.thrust() {
            let offset = {
                const SPREAD: f32 = 5.0;
                let x = player.facing.sign() * style::THRUST_OFFSET;
                let y = (rand::random::<f32>() * 2.0 - 1.0) * SPREAD;
                Vec3::new(x, y, 0.0)
            };
            let mut translation = transform.translation + offset;
            translation.z = -1.0;
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation,
                        scale: Vec3 {
                            x: 0.1,
                            y: 0.05,
                            z: 1.0,
                        },
                        ..default()
                    },
                    texture: asset_server.load(utils::variant(
                        style::SMOKE_TEXTURE,
                        format!("{:02}", rand::random::<u32>() % 25),
                    )),
                    sprite: Sprite {
                        color: utils::bevy::bloom_hue(30.0),
                        ..default()
                    },
                    ..default()
                },
                utils::bevy::Follow {
                    entity: player_entity,
                    offset,
                },
                utils::bevy::DespawnTime {
                    elapsed_seconds: time.elapsed_seconds() + 0.05,
                },
            ));
        }
    }
}

fn set_volume(
    mut thrust_query: Query<(&AudioSink, &mut Thrust)>,
    time: Res<Time>,
    controls: Res<input::Controls>,
) {
    if let Ok((audio, mut thrust)) = thrust_query.get_single_mut() {
        const ADJUST_SPEED: f32 = 4.0;
        thrust.volume = utils::range::Range {
            start: thrust.volume,
            end: if controls.thrust() { 1.0 } else { 0.0 },
        }
        .mix(ADJUST_SPEED * time.delta_seconds());
        audio.set_volume(thrust.volume * style::VOLUME);
    }
}
