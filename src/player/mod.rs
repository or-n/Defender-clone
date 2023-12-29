use bevy::prelude::*;

use crate::{
    assets::{audio, GameAssets},
    camera, explosion, game_over, laser, map, minimap, style, utils,
};
use game_over::GameOver;
use utils::bevy::{hit::*, state::Simulation};
use utils::{range::Range, Side};

pub mod input;
mod thrust;

#[derive(Component)]
pub struct Player {
    pub facing: Side,
    pub horizontal_speed: f32,
    next_shot_time: f32,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins((input::Plug, thrust::Plug))
            .add_systems(
                Update,
                (
                    (
                        laser_hit,
                        detect_hits::<Player>,
                        movement,
                        try_shooting,
                        camera::follow_player,
                    )
                        .chain()
                        .run_if(in_state(Simulation::Running)),
                    minimap::redraw,
                )
                    .chain(),
            )
            .add_systems(PostUpdate, try_drawing_on_minimap);
    }
}

fn try_drawing_on_minimap(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<Player>>,
    mut minimap_event: EventReader<minimap::Ready>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for minimap in minimap_event.read() {
            let p = minimap.normalize(player_transform.translation);
            let p = minimap.f()(&p);
            gizmos.circle(p, Vec3::Z, 2., style::MINIMAP_PLAYER_COLOR);
        }
    }
}

pub fn spawn(
    commands: &mut Commands,
    assets: &Res<GameAssets>,
    camera_query: &Query<&Transform, With<Camera>>,
) {
    let camera = camera_query.single();
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: camera.translation,
                rotation: utils::bevy::angle(-0.25),
                ..default()
            },
            texture: assets.player_texture.clone(),
            ..default()
        },
        Player {
            facing: Side::Right,
            horizontal_speed: 0.0,
            next_shot_time: 0.0,
        },
        thrust::ThrustBundle::new(assets),
        map::Confine,
        Hittable::<crate::laser::Orb>::new(style::PLAYER_BOUND),
    ));
}

pub const HORIZONTAL_SPEED: f32 = 600.0;
const VERTICAL_SPEED: f32 = 400.0;

const ACCELERATION: f32 = 1600.0;
const DECELERATION: f32 = 100.0;

fn movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    controls: Res<input::Controls>,
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        if let Some(side) = controls.facing() {
            if player.facing != side {
                player.facing = side;
                transform.rotate(utils::bevy::angle(0.5));
            }
        }
        let start = player.horizontal_speed;
        player.horizontal_speed = if let Some(side) = controls.facing() {
            let end = side.sign() * HORIZONTAL_SPEED;
            Range { start, end }.step(ACCELERATION * time.delta_seconds())
        } else {
            let end = 0.0;
            let speed_ratio = player.horizontal_speed.abs() / HORIZONTAL_SPEED;
            let t = speed_ratio.powf(1.0);
            Range { start, end }.step(DECELERATION * t * time.delta_seconds())
        };
        let dy = controls.vertical() * VERTICAL_SPEED;
        transform.translation += time.delta_seconds() * Vec3::new(player.horizontal_speed, dy, 0.0);
    }
}

const SHOOT_DELAY: f32 = 0.3;

fn try_shooting(
    mut player_query: Query<(&Transform, &mut Player)>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    controls: Res<input::Controls>,
) {
    let elapsed = time.elapsed_seconds();
    if let Ok((transform, mut player)) = player_query.get_single_mut() {
        if controls.shoot && player.next_shot_time <= elapsed {
            commands.spawn(audio(assets.laser_audio.clone(), style::VOLUME));
            let angle = match player.facing {
                Side::Left => 0.5,
                Side::Right => 0.0,
            };
            let direction = Vec3::X * player.facing.sign();
            let off = style::PLAYER_FRONT_OFFSET + style::LASER_BOUND.x * 0.5;
            let position = transform.translation + off * direction;
            let speed = laser::SPEED;
            let color = utils::bevy::bloom_hue((elapsed * 120.0) % 360.0);
            commands.spawn(laser::Bundle::new(
                &assets,
                position,
                angle,
                speed,
                color,
                laser::Laser,
            ));
            player.next_shot_time = elapsed + SHOOT_DELAY;
        }
    }
}

fn laser_hit(
    query: Query<(Entity, &Transform, &Hittable<crate::laser::Orb>), With<Player>>,
    mut commands: Commands,
    mut explosion_event: EventWriter<explosion::At>,
    mut game_over_event: EventWriter<GameOver>,
) {
    for (player_entity, player, hittable) in query.iter() {
        if let Some(_) = hittable.hit_entity {
            commands.entity(player_entity).despawn();
            explosion_event.send(explosion::At {
                position: player.translation,
            });
            game_over_event.send(GameOver);
        }
    }
}

impl Bound for Player {
    fn bound() -> Vec2 {
        style::PLAYER_BOUND
    }
}
