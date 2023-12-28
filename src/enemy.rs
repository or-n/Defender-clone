use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    assets::{audio, GameAssets},
    explosion, game_over, laser, map, minimap,
    person::{self, Person},
    player::{self, HORIZONTAL_SPEED},
    score, style, utils,
};
use game_over::GameOver;
use player::Player;
use projectile::Projectile;
use score::Score;
use utils::bevy::{hit::*, projectile, state::Simulation};

#[derive(Component)]
pub struct Enemy {
    desired_position: Vec3,
    next_shot: f32,
    next_desired_position: f32,
    last_outside: f32,
    has_person: bool,
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

const ORB_SPEED: f32 = 300.0;

fn shoot_player(
    mut query: Query<(&Transform, &mut Enemy)>,
    player_query: Query<(&Transform, &Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<&Transform, With<Camera>>,
    assets: Res<GameAssets>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let elapsed = time.elapsed_seconds();
    let window = window_query.single();
    let camera_position = camera_query.single().translation;
    if let Ok((player_transform, player)) = player_query.get_single() {
        let player_position = player_transform.translation;
        for (transform, mut enemy) in query.iter_mut() {
            let position = transform.translation;
            let delta = player_position - position;
            let d = delta.length() / (2.5 * HORIZONTAL_SPEED);
            let dir = delta.normalize();
            let d = player_position + Vec3::X * player.horizontal_speed * d
                - position
                - dir * d * ORB_SPEED;
            let dir = Vec2::new(d.x, d.y).normalize();
            let mut angle = Vec2::X.angle_between(dir) / 3.14 * 0.5;
            if angle < 0.0 {
                angle += 1.0;
            }
            let visible = visible(position.x, camera_position.x, window.width());
            if !visible {
                enemy.last_outside = elapsed;
            }
            let v = angle.min(1.0 - angle).min((angle - 0.5).abs()) * 4.0;
            if enemy.next_shot < elapsed && enemy.last_outside + 0.5 < elapsed {
                if rand::random::<f32>() < v.powf(0.25) {
                    commands.spawn(laser::Bundle::new(
                        &assets,
                        position + dir.extend(0.0) * 50.0,
                        angle,
                        ORB_SPEED,
                        utils::bevy::bloom_hue(360.0),
                        false,
                        true,
                    ));
                    commands.spawn(audio(assets.laser_audio.clone(), style::VOLUME));
                }
                enemy.next_shot = elapsed + 1.0;
            }
        }
    }
}

fn movement(
    mut query: Query<(Entity, &mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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
    let window = window_query.single();
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
            let h = window.height() * (1.0 - style::MINIMAP_HEIGHT);
            let offset = style::BORDER_CONFINEMENT_OFFSET;
            let random = |position: Vec3, factor: f32| {
                let dx = rand::random::<f32>() * 2.0 - 1.0;
                let mut dy = rand::random::<f32>() * 2.0 - 1.0;
                let mut p = position + Vec3::new(dx, dy, 0.0) * factor;
                while p.y < offset || p.y > h - offset {
                    dy = rand::random::<f32>() * 2.0 - 1.0;
                    p = position + Vec3::new(dx, dy, 0.0) * factor;
                }
                p
            };
            let max_change = 400.0;
            let p = {
                match x {
                    Some(data) => {
                        let p = data.1;
                        let d = (p - transform.translation).length();
                        if d < 10.0 {
                            *data.2 =
                                person::CharacterState::CapturedBy(entity, person::ENEMY_OFFSET);
                            enemy.has_person = true;
                            commands.spawn(audio(assets.capture_audio.clone(), style::VOLUME));
                            random(transform.translation, max_change)
                        } else {
                            random(p, if d < max_change { 0.0 } else { max_change })
                        }
                    }
                    _ => random(transform.translation, max_change),
                }
            };
            enemy.desired_position = p;
            if enemy.has_person {
                enemy.desired_position.y = h - offset;
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
    enemy_query: Query<&Transform, With<Enemy>>,
    mut minimap_event: EventReader<minimap::Ready>,
) {
    for minimap in minimap_event.read() {
        for enemy_transform in enemy_query.iter() {
            let p = minimap.normalize(enemy_transform.translation);
            let p = minimap.f()(&p);
            gizmos.circle(p, Vec3::Z, 2., style::MINIMAP_ENEMY_COLOR);
        }
    }
}

#[derive(Bundle)]
pub struct Bundle {
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    scroll: map::Scroll,
    confine: map::Confine,
    laser_hit: Hittable<Projectile>,
}

pub fn bundle(position: Vec3, has_person: bool, is_mutant: bool, assets: &GameAssets) -> Bundle {
    let (scale, texture) = if is_mutant {
        (style::MUTANT_SCALE, assets.mutant_texture.clone())
    } else {
        (style::ENEMY_SCALE, assets.enemy_texture.clone())
    };
    Bundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: position,
                scale: scale.extend(1.0),
                ..default()
            },
            texture,
            ..default()
        },
        enemy: Enemy {
            desired_position: position,
            next_shot: 0.0,
            next_desired_position: 0.0,
            last_outside: 0.0,
            has_person,
        },
        scroll: map::Scroll,
        confine: map::Confine,
        laser_hit: Hittable::<Projectile>::new(style::ENEMY_BOUND),
    }
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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
    let window = window_query.single();
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
        while visible(x, camera_position.x, window.width() * 1.5) {
            x = rand::random::<f32>() * map::SIZE;
            x = map_scroll.update(x);
        }
        let y = 100.0 + rand::random::<f32>() * 400.0;
        let position = Vec3::new(x, y, 0.0);
        let has_person = false;
        let is_mutant = false;
        let enemy_entity = commands
            .spawn(bundle(position, has_person, is_mutant, &assets))
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
    query: Query<(Entity, &Transform, &Hittable<Projectile>), With<Enemy>>,
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
    query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    mut explosion_event: EventWriter<explosion::At>,
    mut commands: Commands,
    mut enemies: ResMut<EnemiesCount>,
    mut game_over_event: EventWriter<GameOver>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        let player_position = player_transform.translation;
        for (enemy_entity, enemy_transform) in query.iter() {
            let enemy_position = enemy_transform.translation;
            if box_intersection(
                player_position.xy(),
                style::PLAYER_BOUND,
                enemy_position.xy(),
                style::ENEMY_BOUND,
            )
            .is_some()
            {
                commands.entity(player_entity).despawn();
                commands.entity(enemy_entity).despawn();
                enemies.count -= 1;
                explosion_event.send(explosion::At {
                    position: player_position,
                });
                explosion_event.send(explosion::At {
                    position: enemy_position,
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
) {
    for (entity, transform, enemy) in query.iter() {
        let position = transform.translation;
        if enemy.has_person && position.y > 500.0 {
            commands.entity(entity).despawn();
            let has_person = false;
            let is_mutant = true;
            commands.spawn(bundle(position, has_person, is_mutant, &assets));
        }
    }
}
