use crate::{
    map, style,
    utils::{self, bevy::window},
};
use bevy::prelude::*;

#[derive(Event)]
pub struct Ready {
    transform: Transform,
    pub offset: f32,
    window_size: Vec2,
}

impl Ready {
    fn new(window_size: Vec2, camera_position: Vec3) -> Ready {
        let transform = {
            let scale = (window_size * style::MINIMAP_SIZE).extend(0.0);
            let translation = Vec3::new(
                window_size.x * style::MINIMAP_SIZE.x * (-0.5),
                window_size.y * (0.5 - style::MINIMAP_SIZE.y),
                0.0,
            );
            Transform::from_scale(scale).with_translation(camera_position + translation)
        };
        let offset = camera_position.x / map::SIZE + 0.5;
        let offset = utils::my_fract(-offset);
        Ready {
            transform,
            offset,
            window_size,
        }
    }

    pub fn f(&self) -> impl Fn(&Vec2) -> Vec3 + '_ {
        move |p| self.transform * p.extend(0.0)
    }

    pub fn minimap_x(&self, x: f32) -> f32 {
        utils::my_fract(x / map::SIZE + self.offset)
    }

    pub fn map_y(&self, y: f32) -> f32 {
        y / self.window_size.y
    }

    pub fn normalize(&self, p: Vec3) -> Vec2 {
        Vec2::new(self.minimap_x(p.x), self.map_y(p.y))
    }
}

pub fn redraw(
    mut gizmos: Gizmos,
    window_size: Res<window::Size>,
    camera_query: Query<&Transform, With<Camera>>,
    mut minimap_event: EventWriter<Ready>,
) {
    let camera_position = camera_query.single().translation;
    let m = Ready::new(window_size.0, camera_position);
    let f = m.f();
    {
        //border
        let points = vec![Vec2::ZERO, Vec2::Y, Vec2::ONE, Vec2::X];
        gizmos.linestrip(points.iter().map(&f), style::MINIMAP_COLOR);
        gizmos.line(
            f(&Vec2::new(-1.0, 0.0)),
            f(&Vec2::new(2.0, 0.0)),
            style::MINIMAP_COLOR,
        );
    }
    {
        //view
        const HEIGHT: f32 = 0.1;
        let half_screen_x = 0.5 * window_size.0.x / map::SIZE;
        let min_x = 0.5 - half_screen_x;
        let max_x = 0.5 + half_screen_x;
        let mut points = vec![
            Vec2::new(min_x, HEIGHT),
            Vec2::new(min_x, 0.0),
            Vec2::new(max_x, 0.0),
            Vec2::new(max_x, HEIGHT),
        ];
        gizmos.linestrip(points.iter().map(&f), style::MINIMAP_VIEW_COLOR);
        for point in points.iter_mut() {
            point.y = 1.0 - point.y;
        }
        gizmos.linestrip(points.iter().map(&f), style::MINIMAP_VIEW_COLOR);
    }
    {
        // 0 mark
        gizmos.line(
            f(&Vec2::new(m.offset, 0.0)),
            f(&Vec2::new(m.offset, 1.0)),
            style::MINIMAP_ZERO_MARK_COLOR,
        );
    }
    drop(f);
    minimap_event.send(m);
}
