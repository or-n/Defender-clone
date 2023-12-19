use bevy::{
    prelude::*,
    window::PrimaryWindow,
};

use noise::{NoiseFn, Perlin};
use std::f64::consts::TAU;

use crate::{style, minimap, utils};
use super::MapScroll;

pub const POINTS: usize = 500;
pub const SEGMENTS: usize = POINTS - 1;
pub const SEGMENT_LENGTH: f32 = 10.0;
pub const SEGMENT_HEIGHT: f32 = 400.0;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(gen_terrain())
            .add_systems(Update, draw)
            .add_systems(PostUpdate, try_drawing_on_minimap);
    }
}

#[derive(Resource)]
pub struct Terrain {
    pub heights: Vec<f32>,
}

fn try_drawing_on_minimap(
    mut gizmos: Gizmos,
    terrain: Res<Terrain>,
    mut minimap_event: EventReader<minimap::Ready>,
) {
    for minimap in minimap_event.read() {
        let mut points = vec![];
        for i in 0..terrain.heights.len() {
            let x = (i as f32) / SEGMENTS as f32;
            points.push(Vec2::new(
                utils::my_fract(x + minimap.offset),
                minimap.map_y(terrain.heights[i]),
            ));
        }
        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
        gizmos.linestrip(points.iter().map(minimap.f()), style::TERRAIN_COLOR);
    }
}

fn gen_terrain() -> Terrain {
    let seed = 2137;
    let perlin = Perlin::new(seed);
    let mut heights = vec![];
    for i in 0..SEGMENTS {
        let t = i as f64 / SEGMENTS as f64 * TAU;
        let x = t.cos() * 32.0;
        let y = t.sin() * 32.0;
        let value = perlin.get([x, y]) * 1.0
            + perlin.get([x / 2.0, y / 2.0]) * 2.0
            + perlin.get([x / 4.0, y / 4.0]) * 4.0
            + perlin.get([x / 8.0, y / 8.0]) * 8.0;
        let t = value as f32 / 16.0 * 0.5 + 0.5;
        heights.push(t * t * t * SEGMENT_HEIGHT);
    }
    heights.push(heights[0]);
    Terrain { heights }
}

fn draw(
    mut gizmos: Gizmos,
    terrain: Res<Terrain>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    map_scroll: Res<MapScroll>,
) {
    let window = window_query.get_single().unwrap();
    let mut points = vec![];
    for i in 0..terrain.heights.len() {
        let x = map_scroll.update((i as f32) * SEGMENT_LENGTH);
        if (x - map_scroll.real_camera_x).abs() < window.width() {
            let y = terrain.heights[i];
            points.push(Vec3 { x, y, z: 0.0 });
        }
    }
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    gizmos.linestrip(points, style::TERRAIN_COLOR);
}