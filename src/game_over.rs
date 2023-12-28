use crate::{
    assets::{audio, GameAssets},
    high_scores::*,
    score::Score,
    style, utils,
};
use bevy::prelude::*;
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
        app.add_event::<GameOver>()
            .add_systems(Update, (listen_for_game_over, change_state));
    }
}

fn listen_for_game_over(
    mut event: EventReader<GameOver>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    time: Res<Time>,
    score: Res<Score>,
    mut high_scores: ResMut<HighScores>,
) {
    for _ in event.read() {
        high_scores.save(score.value);
        commands.spawn(audio(assets.game_over_audio.clone(), style::VOICE_VOLUME));
        commands.spawn(ChangeState {
            elapsed: time.elapsed_seconds(),
        });
    }
}

fn change_state(mut commands: Commands, query: Query<(Entity, &ChangeState)>, time: Res<Time>) {
    if let Ok((entity, change_state)) = query.get_single() {
        if change_state.elapsed + 0.5 < time.elapsed_seconds() {
            commands.insert_resource(NextState(Some(Simulation::Paused)));
            commands.entity(entity).despawn();
        }
    }
}
