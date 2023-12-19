use crate::{style, utils};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub begin_wave_audio: Handle<AudioSource>,
    pub laser_audio: Handle<AudioSource>,
    pub enemy_texture: Handle<Image>,
}

pub fn load(asset_server: Res<AssetServer>, mut commands: Commands) {
    let begin_wave_audio = asset_server.load(style::BEGIN_SOUND);
    let laser_audio = asset_server.load(style::LASER_SOUND);
    let enemy_texture = asset_server.load(style::ENEMY_TEXTURE);
    commands.insert_resource(GameAssets {
        begin_wave_audio,
        laser_audio,
        enemy_texture,
    })
}

pub fn audio(source: Handle<AudioSource>) -> AudioBundle {
    let volume = utils::bevy::volume(style::VOLUME);
    AudioBundle {
        source,
        settings: PlaybackSettings::DESPAWN.with_volume(volume),
    }
}
