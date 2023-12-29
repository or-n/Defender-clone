use bevy::prelude::*;

use crate::{
    assets::{GameAssets, MyTexture, MyTransform},
    style, utils,
};
use utils::bevy::hit::*;

#[derive(Component)]
pub struct Laser;

pub const SPEED: f32 = 2400.0 * 2.0;

impl MyTransform for Laser {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle + 0.25))
            .with_scale(style::LASER_SCALE.extend(1.0))
    }
}

impl MyTexture for Laser {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.laser_texture.clone()
    }
}

impl Bound for Laser {
    fn bound() -> Vec2 {
        style::LASER_BOUND
    }
}
