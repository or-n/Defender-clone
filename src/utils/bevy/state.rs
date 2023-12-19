use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Simulation {
    Running,
    #[default]
    Paused,
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .add_state::<Simulation>()
            .add_systems(Update, toggle_simulation_state);
    }
}

fn toggle_simulation_state(
    key: Res<Input<KeyCode>>,
    mut commands: Commands,
    state: Res<State<Simulation>>
) {
    if key.just_pressed(KeyCode::Escape) {
        let next_state = match state.get() {
            Simulation::Running => Simulation::Paused,
            Simulation::Paused => Simulation::Running,
        };
        commands.insert_resource(NextState(Some(next_state)));
    }
}