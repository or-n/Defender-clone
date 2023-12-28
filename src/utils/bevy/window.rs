use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource)]
pub struct Size(pub Vec2);

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, size_resource);
    }
}

fn size_resource(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();
    commands.insert_resource(Size(Vec2 {
        x: window.width(),
        y: window.height(),
    }));
}
