use bevy::prelude::*;

pub const FONT: &str = "fonts/Kenney Rocket.ttf";
pub const VOLUME: f32 = 0.2;
pub const VOICE_VOLUME: f32 = 0.3;
pub const EXPLOSION_VOLUME: f32 = 0.3;

pub const MENU_PLAY_COLOR: Color = Color::rgb(0.0, 0.75, 0.0);
pub const MENU_EXIT_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

pub const TERRAIN_COLOR: Color = Color::ORANGE_RED;
pub const BORDER_CONFINEMENT_OFFSET: f32 = 50.0;

pub const BEGIN_SOUND: &str = "audio/begin.ogg";
pub const GAME_OVER_SOUND: &str = "audio/game_over.ogg";
pub const MIN_ENEMY_COUNT: u32 = 5;
pub const MAX_ENEMY_COUNT: u32 = 15;

pub const MINIMAP_WIDTH: f32 = 0.5;
pub const MINIMAP_HEIGHT: f32 = 0.125;
pub const SCORE_FONT_SIZE: f32 = 60.0;
pub const SCORE_COLOR: Color = Color::WHITE;
pub const MINIMAP_COLOR: Color = Color::CYAN;
pub const MINIMAP_VIEW_COLOR: Color = Color::WHITE;
pub const MINIMAP_PLAYER_COLOR: Color = Color::WHITE;
pub const MINIMAP_ENEMY_COLOR: Color = Color::GREEN;
pub const MINIMAP_ZERO_MARK_COLOR: Color = Color::NONE;

pub const PLAYER_TEXTURE: &str = "sprites/ship_I.png";
pub const PLAYER_BOUND: Vec2 = Vec2::new(64.0 * 0.25, 64.0 * 0.25);
pub const PLAYER_FRONT_OFFSET: f32 = 35.0;
pub const SMOKE_TEXTURE: (&str, &str) = ("sprites/smoke/whitePuff", ".png");
pub const THRUST_SOUND: &str = "audio/boost-engine-loop.ogg";
pub const THRUST_OFFSET: f32 = -20.0;

pub const LASER_TEXTURE: &str = "sprites/laserGreen1.png";
pub const LASER_SCALE: Vec2 = Vec2::new(1.0 / 16.0, 4.0);
pub const LASER_BOUND: Vec2 = Vec2::new(100.0 * LASER_SCALE.y, 38.0 * LASER_SCALE.x);
pub const LASER_SOUND: &str = "audio/laserLarge_003.ogg";

pub const ORB_TEXTURE: &str = "sprites/star_tiny.png";
pub const ORB_SCALE: Vec2 = Vec2::new(1.0, 1.0);
pub const ORB_BOUND: Vec2 = Vec2::new(64.0 * 0.25 * ORB_SCALE.x, 64.0 * 0.25 * ORB_SCALE.y);

pub const ENEMY_TEXTURE: &str = "sprites/shipGreen_manned.png";
pub const ENEMY_SCALE: Vec2 = Vec2::new(0.375, 0.375);
pub const ENEMY_BOUND: Vec2 = Vec2::new(124.0 * ENEMY_SCALE.x, 123.0 * ENEMY_SCALE.y);

pub const COLLISION_SOUND: &str = "audio/space-explosion.ogg";

pub const PERSON_TEXTURE: &str = "sprites/character_zombie_sheet.png";
pub const PERSON_SCALE: Vec2 = Vec2::new(0.3, 0.3);
pub const PERSON_GRID_SIZE: Vec2 = Vec2::new(96.0, 128.0);
pub const PERSON_CENTER: Vec2 = Vec2::new(0.0, -20.0 * PERSON_SCALE.y);
pub const PERSON_BOUND: Vec2 = Vec2::new(
    PERSON_GRID_SIZE.x * PERSON_SCALE.x * 0.8,
    PERSON_GRID_SIZE.y * PERSON_SCALE.y * 0.8,
);

pub const RESCUE_SOUND: &str = "audio/forceField_004.ogg";
