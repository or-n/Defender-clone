use crate::{style, utils};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub begin_wave_audio: Handle<AudioSource>,
    pub game_over_audio: Handle<AudioSource>,
    pub collision_audio: Handle<AudioSource>,
    pub laser_audio: Handle<AudioSource>,
    pub thrust_audio: Handle<AudioSource>,
    pub player_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
    pub orb_texture: Handle<Image>,
    pub laser_texture: Handle<Image>,
}

pub fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(GameAssets {
        begin_wave_audio: asset_server.load(style::BEGIN_SOUND),
        game_over_audio: asset_server.load(style::GAME_OVER_SOUND),
        collision_audio: asset_server.load(style::COLLISION_SOUND),
        laser_audio: asset_server.load(style::LASER_SOUND),
        thrust_audio: asset_server.load(style::THRUST_SOUND),
        player_texture: asset_server.load(style::PLAYER_TEXTURE),
        enemy_texture: asset_server.load(style::ENEMY_TEXTURE),
        orb_texture: asset_server.load(style::ORB_TEXTURE),
        laser_texture: asset_server.load(style::LASER_TEXTURE),
    })
}

pub fn audio(source: Handle<AudioSource>, volume: f32) -> AudioBundle {
    AudioBundle {
        source,
        settings: PlaybackSettings::DESPAWN.with_volume(utils::bevy::volume(volume)),
    }
}
