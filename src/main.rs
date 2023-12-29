use bevy::{app::AppExit, prelude::*};

mod assets;
mod camera;
mod enemy;
mod explosion;
mod game_over;
mod high_scores;
mod map;
mod menu;
mod minimap;
mod person;
mod player;
mod projectile;
mod score;
mod style;
mod utils;

fn main() {
    App::new()
        .add_event::<minimap::Ready>()
        .add_event::<explosion::At>()
        .insert_resource(bevy::asset::AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Defender".into(),
                    mode: bevy::window::WindowMode::BorderlessFullscreen,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            utils::bevy::Plug,
            menu::Plug,
            map::Plug,
            player::Plug,
            enemy::Plug,
            person::Plug,
            projectile::Plug,
            explosion::Plug,
            game_over::Plug,
            high_scores::Plug,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, (assets::load, camera::spawn))
        .add_systems(Update, (try_exiting, camera::window_height_center))
        .run();
}

fn try_exiting(mut exit: EventWriter<AppExit>, key: Res<Input<KeyCode>>) {
    if key.pressed(KeyCode::Q) {
        exit.send(AppExit)
    }
}
