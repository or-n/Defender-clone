use bevy::prelude::*;
use crate::utils;
use utils::{bevy::state::Simulation, Side};

#[derive(Resource)]
pub struct Bindings {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub shoot: KeyCode,
}

#[derive(Resource)]
pub struct Controls {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub shoot: bool,
}

impl Controls {
    pub fn facing(&self) -> Option<Side> {
        if self.move_left {
            Some(Side::Left)
        } else if self.move_right {
            Some(Side::Right)
        } else {
            None
        }
    }

    pub fn thrust(&self) -> bool {
        self.move_left || self.move_right
    }

    pub fn vertical(&self) -> f32 {
        (if self.move_up { 1.0 } else { 0.0 }) +
        (if self.move_down { -1.0 } else { 0.0 })
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(init_bindings())
            .insert_resource(init_controls())
            .add_systems(Update,
                input.run_if(in_state(Simulation::Running))
            );
    }
}

fn init_bindings() -> Bindings {
    Bindings {
        move_up: KeyCode::W,
        move_down: KeyCode::S,
        move_left: KeyCode::A,
        move_right: KeyCode::D,
        shoot: KeyCode::Space,
    }
}

fn init_controls() -> Controls {
    Controls {
        move_up: false,
        move_down: false,
        move_left: false,
        move_right: false,
        shoot: false,
    }
}

fn input(
    key: Res<Input<KeyCode>>,
    bindings: Res<Bindings>,
    mut commands: Commands,
) {
    commands.insert_resource(Controls {
        move_up: key.pressed(bindings.move_up),
        move_down: key.pressed(bindings.move_down),
        move_left: key.pressed(bindings.move_left),
        move_right: key.pressed(bindings.move_right),
        shoot: key.pressed(bindings.shoot),
    });
}