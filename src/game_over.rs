use bevy::prelude::*;
use crate::{utils, style};
use utils::bevy::state::Simulation;

#[derive(Event)]
pub struct GameOver;

#[derive(Component)]
pub struct ChangeState {
    pub elapsed: f32,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameOver>()
            .add_systems(Update, (listen_for_game_over, change_state));
    }
}

fn listen_for_game_over(
    mut event: EventReader<GameOver>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for _ in event.read() {
        commands.spawn(AudioBundle {
            source: asset_server.load(style::GAME_OVER_SOUND),
            settings: PlaybackSettings::DESPAWN
                .with_volume(utils::bevy::volume(style::VOICE_VOLUME)),
        });
        commands.spawn(ChangeState { elapsed: time.elapsed_seconds() });
    }
}

fn change_state(
    mut commands: Commands,
    query: Query<(Entity, &ChangeState)>,
    time: Res<Time>,
) {
    if let Ok((entity, change_state)) = query.get_single() {
        if change_state.elapsed + 0.5 < time.elapsed_seconds() {
            commands.insert_resource(NextState(Some(Simulation::Paused)));
            commands.entity(entity).despawn();
        }
    }
}