use bevy::prelude::*;

use crate::{
    assets::{audio, GameAssets},
    enemy::Enemy,
    map, minimap,
    player::*,
    projectile,
    score::Score,
    style,
    utils::{bevy::hit::*, bevy::state::Simulation},
};

#[derive(Component)]
pub struct Person;

#[derive(Resource)]
pub struct AnimationTimer {
    timer: Timer,
}

#[derive(Component)]
pub enum CharacterState {
    CapturedBy(Entity, Vec2),
    Falling,
    Grounded,
}

#[derive(Bundle)]
pub struct Bundle {
    state: CharacterState,
    sprite_sheet: SpriteSheetBundle,
    person: Person,
    laser_hit: Hittable<projectile::laser::Laser>,
    player_hit: Hittable<Player>,
    scroll: map::Scroll,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimationTimer {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        })
        .add_systems(
            PostUpdate,
            (
                try_drawing_on_minimap,
                (update, laser_hit, player_hit).run_if(in_state(Simulation::Running)),
            ),
        );
    }
}

fn try_drawing_on_minimap(
    mut gizmos: Gizmos,
    query: Query<&GlobalTransform, With<Person>>,
    mut minimap_event: EventReader<minimap::Ready>,
) {
    for minimap in minimap_event.read() {
        for transform in query.iter() {
            let p = minimap.normalize(transform.translation());
            let p = minimap.f()(&p);
            gizmos.circle(p, Vec3::Z, 2., style::MINIMAP_PERSON_COLOR);
        }
    }
}

pub fn bundle(position: Vec2, state: CharacterState, assets: &GameAssets) -> Bundle {
    Bundle {
        state,
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: assets.person_texture_atlas.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(position.extend(0.0))
                .with_scale(style::PERSON_SCALE.extend(1.0)),
            ..default()
        },
        person: Person,
        laser_hit: Hittable::new(style::PERSON_BOUND),
        player_hit: Hittable::new(style::PERSON_BOUND),
        scroll: map::Scroll,
    }
}

pub const ENEMY_OFFSET: Vec2 = Vec2::new(0.0, -40.0);
pub const PLAYER_OFFSET: Vec2 = Vec2::new(0.0, -20.0);

pub fn update(
    mut query: Query<(&mut CharacterState, &mut Transform, &mut TextureAtlasSprite), With<Person>>,
    captor_query: Query<&Transform, Without<Person>>,
    time: Res<Time>,
    mut timer: ResMut<AnimationTimer>,
) {
    timer.timer.tick(time.delta());
    for (mut state, mut transform, mut sprite) in query.iter_mut() {
        if let CharacterState::CapturedBy(entity, _) = *state {
            if let Err(_) = captor_query.get(entity) {
                *state = CharacterState::Falling;
            }
        }
        match *state {
            CharacterState::CapturedBy(entity, offset) => {
                sprite.index = 0;
                if let Ok(captor) = captor_query.get(entity) {
                    transform.translation = captor.translation + offset.extend(0.0);
                }
            }
            CharacterState::Falling => {
                sprite.index = 4;
                transform.translation.y -= 50.0 * time.delta_seconds();
                let bound = style::PERSON_BOUND.y + style::PERSON_CENTER.y;
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

fn laser_hit(
    query: Query<(Entity, &CharacterState, &Hittable<projectile::laser::Laser>), With<Person>>,
    mut enemy_query: Query<&mut Enemy>,
    mut commands: Commands,
) {
    for (person_entity, state, hittable) in query.iter() {
        if let Some(_) = hittable.hit_entity {
            commands.entity(person_entity).despawn();
            if let CharacterState::CapturedBy(entity, _) = state {
                if let Ok(mut enemy) = enemy_query.get_mut(*entity) {
                    enemy.person = None;
                }
            }
        }
    }
}

fn player_hit(
    mut query: Query<(&Hittable<Player>, &mut CharacterState), With<Person>>,
    mut commands: Commands,
    controls: Res<input::Controls>,
    mut score: ResMut<Score>,
    assets: Res<GameAssets>,
) {
    for (hittable, mut state) in query.iter_mut() {
        if let Some(player_entity) = hittable.hit_entity {
            if controls.rescue && !matches!(*state, CharacterState::CapturedBy(_, _)) {
                score.value += 100;
                *state = CharacterState::CapturedBy(player_entity, PLAYER_OFFSET);
                commands.spawn(audio(assets.capture_audio.clone(), style::VOLUME));
            }
        }
    }
}
