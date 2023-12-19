use bevy::{prelude::*, sprite::collide_aabb::collide};

pub fn hit(
    a: Vec3,
    a_bound: Vec2,
    b: Vec3,
    b_bound: Vec2,
    camera: Vec3,
    window_width: f32,
) -> bool {
    let sub_r = ((b.x + b_bound.x) - (camera.x + window_width * 0.5)).max(0.0);
    let sub_l = ((camera.x - window_width * 0.5) - (b.x - b_bound.x)).max(0.0);
    collide(a, a_bound, b, b_bound - Vec2::X * (sub_l + sub_r)).is_some()
}
