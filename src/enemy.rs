use bevy::{prelude::*, sprite::collide_aabb::collide, window::PrimaryWindow};

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
use utils::bevy::{hit::hit, projectile, state::Simulation};

#[derive(Component)]
pub struct Enemy {
    desired_position: Vec3,
    position: Vec3,
    next_shot: f32,
    next_desired_position: f32,
    last_outside: f32,
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
                    shoot_player.after(movement),
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

const ORB_SPEED: f32 = 450.0;

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
            let angle = Vec2::X.angle_between(dir) / 3.14 * 0.5;
            let visible = visible(position.x, camera_position.x, window.width());
            if !visible {
                enemy.last_outside = elapsed;
            }
            if enemy.next_shot < elapsed && enemy.last_outside + 0.5 < elapsed {
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
                enemy.next_shot = elapsed + 2.0;
            }
        }
    }
}

fn movement(
    mut query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    map_scroll: Res<map::MapScroll>,
) {
    let elapsed = time.elapsed_seconds();
    let window = window_query.single();
    for (mut transform, mut enemy) in query.iter_mut() {
        if enemy.next_desired_position < elapsed {
            let dx = rand::random::<f32>() * 2.0 - 1.0;
            let mut dy = rand::random::<f32>() * 2.0 - 1.0;
            let offset = style::BORDER_CONFINEMENT_OFFSET;
            let h = window.height() * (1.0 - style::MINIMAP_HEIGHT);
            let mut p = enemy.position + Vec3::new(dx, dy, 0.0) * 800.0;
            while p.y < offset || p.y > h - offset {
                dy = rand::random::<f32>() * 2.0 - 1.0;
                p = enemy.position + Vec3::new(dx, dy, 0.0) * 800.0;
            }
            enemy.desired_position = p;
            enemy.next_desired_position = elapsed + 1.0;
        }
        let start = enemy.position;
        let end = enemy.desired_position;
        let dx = if (end.x - start.x).abs() > 0.5 {
            start.x - end.x
        } else {
            end.x - start.x
        };
        let dy = end.y - start.y;
        let d = Vec2::new(dx, dy).normalize().extend(0.0);
        let step = 200.0 * time.delta_seconds();
        enemy.position += d * step;
        let mut p = enemy.position;
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
    commands.spawn(AudioBundle {
        source: assets.begin_wave_audio.clone(),
        settings: PlaybackSettings::DESPAWN.with_volume(utils::bevy::volume(style::VOICE_VOLUME)),
    });
    for person in person_query.iter() {
        commands.entity(person).despawn();
    }
    for _ in 0..enemies.wave.min(style::MAX_ENEMY_COUNT) {
        let mut x = rand::random::<f32>() * map::SIZE;
        x = map_scroll.update(x);
        while visible(x, camera_position.x, window.width() * 1.5) {
            x = rand::random::<f32>() * map::SIZE;
            x = map_scroll.update(x);
        }
        let y = 100.0 + rand::random::<f32>() * 400.0;
        let desired_position = Vec3::new(x, y, 0.0);
        let enemy_entity = commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: desired_position,
                        scale: style::ENEMY_SCALE.extend(1.0),
                        ..default()
                    },
                    texture: assets.enemy_texture.clone(),
                    ..default()
                },
                Enemy {
                    position: desired_position,
                    desired_position,
                    next_shot: 0.0,
                    next_desired_position: 0.0,
                    last_outside: 0.0,
                },
                map::Scroll,
                map::Confine,
            ))
            .id();
        commands.spawn(person::bundle(
            person::CharacterState::CapturedBy(enemy_entity),
            &assets,
        ));
        enemies.count += 1;
    }
}

fn laser_hit(
    query: Query<(Entity, &Transform), (With<Enemy>, Without<Person>)>,
    laser_query: Query<(Entity, &Transform, &Projectile), (Without<Enemy>, Without<Person>)>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    camera_query: Query<&Transform, With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut explosion_event: EventWriter<explosion::At>,
    mut enemies: ResMut<EnemiesCount>,
) {
    for (enemy_entity, enemy) in query.iter() {
        for (_, laser, projectile) in laser_query.iter() {
            if !projectile.is_damaging {
                continue;
            }
            if hit(
                enemy.translation,
                style::ENEMY_BOUND,
                laser.translation,
                projectile.bound,
                camera_query.single().translation,
                utils::bevy::size(window_query.single()),
            )
            .is_some()
            {
                score.value += 1;
                commands.entity(enemy_entity).despawn();
                enemies.count -= 1;
                explosion_event.send(explosion::At {
                    position: enemy.translation,
                });
                break;
            }
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
            if let Some(_) = collide(
                player_position,
                style::PLAYER_BOUND,
                enemy_position,
                style::ENEMY_BOUND,
            ) {
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
