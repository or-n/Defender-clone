use super::{Variant, VariantData};
use crate::{
    assets::{GameAssets, MyTexture, MyTransform},
    style,
    utils::{self, bevy::hit::Bound},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Mutant;

impl MyTexture for Mutant {
    fn texture(assets: &GameAssets) -> Handle<Image> {
        assets.mutant_texture.clone()
    }
}

impl MyTransform for Mutant {
    fn transform(angle: f32) -> Transform {
        Transform::from_rotation(utils::bevy::angle(angle))
            .with_scale(style::MUTANT_SCALE.extend(1.0))
    }
}

impl Bound for Mutant {
    fn bound() -> Vec2 {
        style::MUTANT_BOUND
    }
}

impl Variant for Mutant {
    fn data() -> VariantData {
        VariantData {
            orb_color: utils::bevy::bloom_hue(120.0),
            shot_delay: 0.25,
            minimap_color: style::MINIMAP_MUTANT_COLOR,
        }
    }
}
