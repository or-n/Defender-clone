use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component)]
pub struct Hittable<T> {
    pub hitbox: Vec2,
    pub hit_entity: Option<Entity>,
    phantom: PhantomData<T>,
}

impl<T> Hittable<T> {
    pub fn new(hitbox: Vec2) -> Hittable<T> {
        Hittable {
            hitbox,
            hit_entity: None,
            phantom: PhantomData,
        }
    }
}

pub fn hit(
    a: Vec3,
    a_bound: Vec2,
    b: Vec3,
    b_bound: Vec2,
    c: Vec3,
    c_bound: Vec2,
) -> Option<(Vec2, Vec2)> {
    let (ab, ab_bound) = box_intersection(a.xy(), a_bound, b.xy(), b_bound)?;
    box_intersection(ab, ab_bound, c.xy(), c_bound)
}

pub fn box_intersection(a: Vec2, a_bound: Vec2, b: Vec2, b_bound: Vec2) -> Option<(Vec2, Vec2)> {
    let (a_half, b_half) = (a_bound * 0.5, b_bound * 0.5);
    let coord = |a: f32, b: f32, a_half: f32, b_half: f32| {
        let (min, max) = ((a - a_half).max(b - b_half), (a + a_half).min(b + b_half));
        if min < max {
            Some((min, max))
        } else {
            None
        }
    };
    let process = |(min, max)| ((min + max) * 0.5, max - min);
    let (x, size_x) = process(coord(a.x, b.x, a_half.x, b_half.x)?);
    let (y, size_y) = process(coord(a.y, b.y, a_half.y, b_half.y)?);
    Some((Vec2::new(x, y), Vec2::new(size_x, size_y)))
}
