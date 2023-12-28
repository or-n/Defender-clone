use crate::{
    style,
    utils::{self, bevy::window},
};
use bevy::prelude::*;

pub mod terrain;

pub const SIZE: f32 = terrain::SEGMENTS as f32 * terrain::SEGMENT_LENGTH;

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_plugins(terrain::Plug)
            .insert_resource(MapScroll::new(0.0))
            .add_systems(Update, scroll)
            .add_systems(PostUpdate, confine);
    }
}

#[derive(Component)]
pub struct Confine;

fn confine(mut query: Query<&mut Transform, With<Confine>>, window_size: Res<window::Size>) {
    let offset = style::BORDER_CONFINEMENT_OFFSET;
    for mut transform in query.iter_mut() {
        let position = transform.translation;
        transform.translation.y = position.y.clamp(
            offset,
            window_size.0.y * (1.0 - style::MINIMAP_SIZE.y) - offset,
        );
    }
}

#[derive(Resource)]
pub struct MapScroll {
    map_index: f32,
    camera_x: f32,
    real_camera_x: f32,
}

impl MapScroll {
    pub fn new(camera_x: f32) -> Self {
        let real_camera_x = camera_x;
        let normalized = camera_x / SIZE;
        let map_index = normalized.floor();
        let camera_x = utils::my_fract(normalized);
        MapScroll {
            map_index,
            camera_x,
            real_camera_x,
        }
    }

    pub fn update(&self, x: f32) -> f32 {
        let xfract = utils::my_fract(x / SIZE);
        let offset = if xfract > self.camera_x + 0.5 {
            -1.0
        } else if xfract < self.camera_x - 0.5 {
            1.0
        } else {
            0.0
        };
        (xfract + self.map_index + offset) * SIZE
    }
}

#[derive(Component)]
pub struct Scroll;

fn scroll(
    mut query: Query<&mut Transform, With<Scroll>>,
    camera_query: Query<&Transform, (With<Camera>, Without<Scroll>)>,
    mut commands: Commands,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let camera_x = camera_transform.translation.x;
        let map_scroll = MapScroll::new(camera_x);
        for mut transform in query.iter_mut() {
            transform.translation.x = map_scroll.update(transform.translation.x);
        }
        commands.insert_resource(map_scroll);
    }
}
