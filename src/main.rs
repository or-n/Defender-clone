use bevy::{
    prelude::*,
    app::AppExit,
};

mod player;
mod map;
mod minimap;
mod laser;
mod utils;
mod style;
mod camera;
mod enemy;
mod score;
mod menu;
mod explosion;
mod game_over;

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
        .add_systems(Startup, camera::spawn)
        .add_systems(Update, try_exiting)
        .run();
}

fn try_exiting(
    mut exit: EventWriter<AppExit>,
    key: Res<Input<KeyCode>>,
) {
    if key.pressed(KeyCode::Q) {
        exit.send(AppExit)
    }
}
