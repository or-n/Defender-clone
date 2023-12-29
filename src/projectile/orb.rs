use bevy::prelude::*;

use crate::{
    assets::{GameAssets, MyTexture, MyTransform},
    style, utils,
};
use utils::bevy::hit::*;

#[derive(Component)]
pub struct Orb;

pub const SPEED: f32 = 300.0;

impl MyTransform for Orb {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle)).with_scale(style::ORB_SCALE.extend(1.0))
    }
}

impl MyTexture for Orb {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.orb_texture.clone()
    }
}

impl Bound for Orb {
    fn bound() -> Vec2 {
        style::ORB_BOUND
    }
}
