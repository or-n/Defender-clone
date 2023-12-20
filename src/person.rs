use bevy::prelude::*;

use crate::assets::GameAssets;

#[derive(Component)]
pub enum CharacterState {
    Captured,
    Falling,
    Grounded,
}

#[derive(Bundle)]
pub struct Bundle {
    state: CharacterState,
    sprite_sheet: SpriteSheetBundle,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, update);
    }
}

pub fn bundle(transform: Transform, assets: &GameAssets) -> Bundle {
    Bundle {
        state: CharacterState::Captured,
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: assets.person_texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform,
            ..default()
        },
    }
}

pub fn update(mut query: Query<(&CharacterState, &mut TextureAtlasSprite)>) {
    for (state, mut sprite) in query.iter_mut() {
        match state {
            CharacterState::Captured => {
                sprite.index = 0;
            }
            CharacterState::Falling => {
                sprite.index = 4;
            }
            CharacterState::Grounded => {
                sprite.index = 0;
            }
        }
    }
}
