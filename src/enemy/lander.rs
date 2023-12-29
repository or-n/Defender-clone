use super::{Variant, VariantData};
use crate::{
    assets::{GameAssets, MyTexture, MyTransform},
    style,
    utils::{self, bevy::hit::Bound},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Lander;

impl MyTexture for Lander {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.enemy_texture.clone()
    }
}

impl MyTransform for Lander {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle))
            .with_scale(style::ENEMY_SCALE.extend(1.0))
    }
}

impl Bound for Lander {
    fn bound() -> Vec2 {
        style::ENEMY_BOUND
    }
}

impl Variant for Lander {
    fn data() -> VariantData {
        VariantData {
            orb_color: utils::bevy::bloom_hue(360.0),
            shot_delay: 1.0,
            minimap_color: style::MINIMAP_ENEMY_COLOR,
        }
    }
}
