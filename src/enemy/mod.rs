use bevy::prelude::*;

use crate::{
    assets::{audio, GameAssets, MyTexture, MyTransform},
    explosion, game_over, map, minimap,
    person::{self, Person},
    player::{self, HORIZONTAL_SPEED},
    projectile, score, style, utils,
};
use game_over::GameOver;
use player::Player;
use score::Score;
use utils::bevy::{hit::*, state::Simulation, window};

pub mod lander;
pub mod mutant;

#[derive(Component)]
pub struct Enemy {
    desired_position: Vec3,
    next_shot: f32,
    next_desired_position: f32,
    last_outside: f32,
    has_person: bool,
}

pub trait Variant {
    fn data() -> VariantData;
}

#[derive(Component)]
pub struct VariantData {
    orb_color: Color,
    shot_delay: f32,
    minimap_color: Color,
}

#[derive(Resource)]
pub struct EnemiesCount {
    pub count: u32,
    pub wave: u32,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemiesCount { count: 0, wave: 0 })
            .add_plugins(score::Plug)
            .add_systems(
                Update,
                (
                    movement,
                    laser_hit,
                    player_hit,
                    (shoot_player, mutant_transform).after(movement),
                )
                    .run_if(in_state(Simulation::Running)),
            )
            .add_systems(
                PostUpdate,
                (
                    try_drawing_on_minimap,
                    spawn_enemies.run_if(in_state(Simulation::Running)),
                ),
            );
    }
}

pub fn visible(x: f32, camera_x: f32, window_width: f32) -> bool {
    let half_screen_x = window_width * 0.5;
    let x = x - camera_x;
    x >= -half_screen_x && x <= half_screen_x
}

fn shoot_player(
    mut query: Query<(&Transform, &VariantData, &mut Enemy)>,
    player_query: Query<(&Transform, &Player)>,
    window_size: Res<window::Size>,
    camera_query: Query<&Transform, With<Camera>>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_seconds();
    let camera_position = camera_query.single().translation;
    if let Ok((player_transform, player)) = player_query.get_single() {
        let player_position = player_transform.translation;
        for (transform, variant, mut enemy) in query.iter_mut() {
            let position = transform.translation;
            let delta = player_position - position;
            let d = delta.length() / (2.5 * HORIZONTAL_SPEED);
            let dir = delta.normalize();
            let d = player_position + Vec3::X * player.horizontal_speed * d
                - position
                - dir * d * projectile::orb::SPEED;
            let dir = Vec2::new(d.x, d.y).normalize();
            let mut angle = Vec2::X.angle_between(dir) / 3.14 * 0.5;
            if angle < 0.0 {
                angle += 1.0;
            }
            if !visible(position.x, camera_position.x, window_size.0.x) {
                enemy.last_outside = elapsed;
            }
            let v = angle.min(1.0 - angle).min((angle - 0.5).abs()) * 4.0;
            if enemy.next_shot < elapsed && enemy.last_outside + 0.5 < elapsed {
                if rand::random::<f32>() < v.powf(0.25) {
                    commands.spawn(projectile::Bundle::new(
                        &assets,
                        position + dir.extend(0.0) * 50.0,
                        angle,
                        projectile::orb::SPEED,
                        variant.orb_color,
                        projectile::orb::Orb,
                    ));
                    commands.spawn(audio(assets.laser_audio.clone(), style::VOLUME));
                }
                enemy.next_shot = elapsed + variant.shot_delay;
            }
        }
    }
}

fn movement(
    mut query: Query<(Entity, &mut Transform, &mut Enemy)>,
    window_size: Res<window::Size>,
    time: Res<Time>,
    map_scroll: Res<map::MapScroll>,
    mut person_query: Query<
        (&Transform, &mut person::CharacterState),
        (With<Person>, Without<Enemy>),
    >,
    assets: Res<GameAssets>,
    mut commands: Commands,
) {
    let elapsed = time.elapsed_seconds();
    for (entity, mut transform, mut enemy) in query.iter_mut() {
        if enemy.next_desired_position < elapsed {
            let mut person_data = vec![];
            if !enemy.has_person {
                for (person_transform, person_state) in person_query.iter_mut() {
                    if matches!(*person_state, person::CharacterState::Grounded) {
                        let p = person_transform.translation - person::ENEMY_OFFSET.extend(0.0);
                        let d = p - transform.translation;
                        person_data.push((d.length(), p, person_state));
                    }
                }
            }
            let x = person_data
                .iter_mut()
                .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            let h = window_size.0.y * (1.0 - style::MINIMAP_SIZE.y);
            let offset = style::BORDER_CONFINEMENT_OFFSET;
            let r = |factor: f32| (rand::random::<f32>() * 2.0 - 1.0) * factor;
            let random = |position: Vec3, factor: f32| {
                let (dx, mut dy) = (r(factor), r(factor));
                while dy < -position.y + offset || dy > h - position.y - offset {
                    dy = r(factor);
                }
                position + Vec3::new(dx, dy, 0.0)
            };
            let max_change = 400.0;
            enemy.desired_position = match x {
                Some(data) => {
                    let p = data.1;
                    let d = (p - transform.translation).length();
                    if d < 10.0 {
                        *data.2 = person::CharacterState::CapturedBy(entity, person::ENEMY_OFFSET);
                        enemy.has_person = true;
                        commands.spawn(audio(assets.capture_audio.clone(), style::VOLUME));
                        random(transform.translation, max_change)
                    } else {
                        random(p, if d < max_change { 0.0 } else { max_change })
                    }
                }
                _ => random(transform.translation, max_change),
            };
            if enemy.has_person {
                enemy.desired_position.y = h;
            }
            enemy.next_desired_position = elapsed + 1.0;
        }
        let mut start = transform.translation;
        start.x = utils::my_fract(start.x / map::SIZE);
        let mut end = enemy.desired_position;
        end.x = utils::my_fract(end.x / map::SIZE);
        let dx = if (end.x - start.x).abs() > 0.5 {
            start.x - end.x
        } else {
            end.x - start.x
        };
        let dy = end.y - start.y;
        let d = Vec2::new(dx * map::SIZE, dy).normalize().extend(0.0);
        let step = 100.0 * time.delta_seconds();
        let mut p = transform.translation + d * step;
        p.x = map_scroll.update(p.x);
        transform.translation = p;
    }
}

fn try_drawing_on_minimap(
    mut gizmos: Gizmos,
    enemy_query: Query<(&Transform, &VariantData)>,
    mut minimap_event: EventReader<minimap::Ready>,
) {
    for minimap in minimap_event.read() {
        for (enemy_transform, variant) in enemy_query.iter() {
            let p = minimap.normalize(enemy_transform.translation);
            let p = minimap.f()(&p);
            gizmos.circle(p, Vec3::Z, 2., variant.minimap_color);
        }
    }
}

#[derive(Bundle)]
pub struct Bundle<T: Send + Sync + Component> {
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    scroll: map::Scroll,
    confine: map::Confine,
    laser_hit: Hittable<projectile::laser::Laser>,
    player_hit: Hittable<Player>,
    variant: T,
    variant_data: VariantData,
}

pub fn bundle<T: Component + MyTexture + MyTransform + Bound + Variant>(
    translation: Vec3,
    has_person: bool,
    variant: T,
    assets: &GameAssets,
) -> Bundle<T> {
    Bundle {
        sprite_bundle: SpriteBundle {
            transform: T::transform(0.0).with_translation(translation),
            texture: T::texture(assets),
            ..default()
        },
        enemy: Enemy {
            desired_position: translation,
            next_shot: 0.0,
            next_desired_position: 0.0,
            last_outside: 0.0,
            has_person,
        },
        scroll: map::Scroll,
        confine: map::Confine,
        laser_hit: Hittable::new(T::bound()),
        player_hit: Hittable::new(T::bound()),
        variant,
        variant_data: T::data(),
    }
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    window_size: Res<window::Size>,
    camera_query: Query<&Transform, With<Camera>>,
    mut enemies: ResMut<EnemiesCount>,
    mut score: ResMut<Score>,
    map_scroll: Res<map::MapScroll>,
    player_query: Query<With<Player>>,
    person_query: Query<Entity, With<Person>>,
) {
    if enemies.count > 0 || player_query.get_single().is_err() {
        return;
    }
    let camera_position = camera_query.single().translation;
    score.value += enemies.wave * 10;
    if enemies.wave == 0 {
        enemies.wave = style::MIN_ENEMY_COUNT;
    } else {
        enemies.wave += 1;
    }
    commands.spawn(audio(assets.begin_wave_audio.clone(), style::VOICE_VOLUME));
    let n = person_query.iter().count();
    score.value += 50 * n as u32;
    for _ in 0..(8 - n) {
        let bound = style::PERSON_BOUND.y + style::PERSON_CENTER.y;
        commands.spawn(person::bundle(
            Vec2::new(rand::random::<f32>() * map::SIZE, bound),
            person::CharacterState::Grounded,
            &assets,
        ));
    }
    for _ in 0..enemies.wave.min(style::MAX_ENEMY_COUNT) {
        let mut x = rand::random::<f32>() * map::SIZE;
        x = map_scroll.update(x);
        while visible(x, camera_position.x, window_size.0.x * 1.5) {
            x = rand::random::<f32>() * map::SIZE;
            x = map_scroll.update(x);
        }
        let y = 100.0 + rand::random::<f32>() * 400.0;
        let position = Vec3::new(x, y, 0.0);
        let has_person = false;
        let enemy_entity = commands
            .spawn(bundle(position, has_person, lander::Lander, &assets))
            .id();
        if has_person {
            commands.spawn(person::bundle(
                position.xy(),
                person::CharacterState::CapturedBy(enemy_entity, person::ENEMY_OFFSET),
                &assets,
            ));
        }
        enemies.count += 1;
    }
}

fn laser_hit(
    query: Query<(Entity, &Transform, &Hittable<projectile::laser::Laser>), With<Enemy>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut explosion_event: EventWriter<explosion::At>,
    mut enemies: ResMut<EnemiesCount>,
) {
    for (enemy_entity, enemy, hittable) in query.iter() {
        if let Some(_) = hittable.hit_entity {
            score.value += 1;
            commands.entity(enemy_entity).despawn();
            enemies.count -= 1;
            explosion_event.send(explosion::At {
                position: enemy.translation,
            });
        }
    }
}

fn player_hit(
    query: Query<(Entity, &Transform, &Hittable<Player>), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut explosion_event: EventWriter<explosion::At>,
    mut commands: Commands,
    mut enemies: ResMut<EnemiesCount>,
    mut game_over_event: EventWriter<GameOver>,
) {
    for (enemy_entity, enemy_transform, hittable) in query.iter() {
        if let Some(player_entity) = hittable.hit_entity {
            if let Ok(player_transform) = player_query.get(player_entity) {
                commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
                enemies.count -= 1;
                explosion_event.send(explosion::At {
                    position: player_transform.translation,
                });
                explosion_event.send(explosion::At {
                    position: enemy_transform.translation,
                });
                game_over_event.send(GameOver);
                break;
            }
        }
    }
}

fn mutant_transform(
    query: Query<(Entity, &Transform, &Enemy)>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    window_size: Res<window::Size>,
) {
    let h = window_size.0.y * (1.0 - style::MINIMAP_SIZE.y);
    let offset = style::BORDER_CONFINEMENT_OFFSET;
    for (entity, transform, enemy) in query.iter() {
        let position = transform.translation;
        if enemy.has_person && position.y > h - (offset + 1.0) {
            commands.entity(entity).despawn();
            let has_person = false;
            commands.spawn(bundle(position, has_person, mutant::Mutant, &assets));
        }
    }
}
