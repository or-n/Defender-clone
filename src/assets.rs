use crate::{style, utils};
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub begin_wave_audio: Handle<AudioSource>,
    pub game_over_audio: Handle<AudioSource>,
    pub collision_audio: Handle<AudioSource>,
    pub laser_audio: Handle<AudioSource>,
    pub thrust_audio: Handle<AudioSource>,
    pub capture_audio: Handle<AudioSource>,
    pub player_texture: Handle<Image>,
    pub enemy_texture: Handle<Image>,
    pub mutant_texture: Handle<Image>,
    pub _bomber_texture: Handle<Image>,
    pub orb_texture: Handle<Image>,
    pub laser_texture: Handle<Image>,
    pub person_texture_atlas: Handle<TextureAtlas>,
}

pub fn texture_atlas(handle: Handle<Image>, tile_size: Vec2, size: (usize, usize)) -> TextureAtlas {
    TextureAtlas::from_grid(handle, tile_size, size.0, size.1, None, None)
}

pub fn load(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.insert_resource(GameAssets {
        begin_wave_audio: asset_server.load(style::BEGIN_SOUND),
        game_over_audio: asset_server.load(style::GAME_OVER_SOUND),
        collision_audio: asset_server.load(style::COLLISION_SOUND),
        laser_audio: asset_server.load(style::LASER_SOUND),
        thrust_audio: asset_server.load(style::THRUST_SOUND),
        capture_audio: asset_server.load(style::CAPTURE_SOUND),
        player_texture: asset_server.load(style::PLAYER_TEXTURE),
        enemy_texture: asset_server.load(style::ENEMY_TEXTURE),
        mutant_texture: asset_server.load(style::MUTANT_TEXTURE),
        _bomber_texture: asset_server.load(style::BOMBER_TEXTURE),
        orb_texture: asset_server.load(style::ORB_TEXTURE),
        laser_texture: asset_server.load(style::LASER_TEXTURE),
        person_texture_atlas: texture_atlases.add(texture_atlas(
            asset_server.load(style::PERSON_TEXTURE),
            style::PERSON_GRID_SIZE,
            (9, 5),
        )),
    })
}

pub fn audio(source: Handle<AudioSource>, volume: f32) -> AudioBundle {
    AudioBundle {
        source,
        settings: PlaybackSettings::DESPAWN.with_volume(utils::bevy::volume(volume)),
    }
}

pub trait MyTransform {
    fn transform(angle: f32) -> Transform;
}

pub trait MyTexture {
    fn texture(assets: &GameAssets) -> Handle<Image>;
}
