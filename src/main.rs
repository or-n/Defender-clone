use bevy::{app::AppExit, audio::AudioSource, prelude::*};

mod assets;
mod camera;
mod enemy;
mod explosion;
mod game_over;
mod laser;
mod map;
mod menu;
mod minimap;
mod player;
mod score;
mod style;
mod utils;

fn main() {
    App::new()
        .add_event::<minimap::Ready>()
        .add_event::<explosion::At>()
        .add_plugins((
            DefaultPlugins,
            utils::bevy::Plug,
            menu::Plug,
            map::Plug,
            player::Plug,
            enemy::Plug,
            laser::Plug,
            explosion::Plug,
            game_over::Plug,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, (assets::load, camera::spawn))
        .add_systems(Update, try_exiting)
        .run();
}

fn try_exiting(mut exit: EventWriter<AppExit>, key: Res<Input<KeyCode>>) {
    if key.pressed(KeyCode::Q) {
        exit.send(AppExit)
    }
}
