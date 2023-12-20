use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::enemy::Enemy;
use crate::style;

#[derive(Component)]
pub struct Person;

#[derive(Resource)]
pub struct AnimationTimer {
    timer: Timer,
}

#[derive(Component)]
pub enum CharacterState {
    CapturedBy(Entity),
    Falling,
    Grounded,
}

#[derive(Bundle)]
pub struct Bundle {
    state: CharacterState,
    sprite_sheet: SpriteSheetBundle,
    person: Person,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .add_systems(Update, update);
    }
}

pub fn bundle(state: CharacterState, assets: &GameAssets) -> Bundle {
    Bundle {
        state,
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: assets.person_texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(style::PERSON_SCALE.extend(1.0)),
            ..default()
        },
        person: Person,
    }
}

pub const PERSON_OFFSET: Vec3 = Vec3::new(0.0, -40.0, 0.0);

pub fn update(
    mut query: Query<(&mut CharacterState, &mut Transform, &mut TextureAtlasSprite), With<Person>>,
    captor_query: Query<&Transform, (With<Enemy>, Without<Person>)>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
) {
    timer.timer.tick(time.delta());
    for (mut state, mut transform, mut sprite) in query.iter_mut() {
        if let CharacterState::CapturedBy(entity) = *state {
            if let Err(_) = captor_query.get(entity) {
                *state = CharacterState::Falling;
            }
        }
        match *state {
            CharacterState::CapturedBy(entity) => {
                sprite.index = 0;
                if let Ok(captor) = captor_query.get(entity) {
                    transform.translation = captor.translation + PERSON_OFFSET;
                }
            }
            CharacterState::Falling => {
                sprite.index = 4;
                transform.translation.y -= 100.0 * time.delta_seconds();
                let bound =
                    (style::PERSON_BOUND.y + style::PERSON_CENTER.y) * style::PERSON_SCALE.y;
                if transform.translation.y < bound {
                    transform.translation.y = bound;
                    *state = CharacterState::Grounded;
                }
            }
            CharacterState::Grounded => {
                if timer.timer.just_finished() {
                    sprite.index += 1;
                    if sprite.index > 44 {
                        sprite.index = 0;
                    }
                }
            }
        }
    }
}
