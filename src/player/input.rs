use crate::utils;
use bevy::prelude::*;
use utils::{bevy::state::Simulation, Side};

#[derive(Clone, Copy)]
enum Bind {
    Key(KeyCode),
    _Button(MouseButton),
}

#[derive(Resource)]
pub struct Bindings {
    move_up: Bind,
    move_down: Bind,
    move_left: Bind,
    move_right: Bind,
    shoot: Bind,
    rescue: Bind,
}

#[derive(Resource)]
pub struct Controls {
    pub move_up: bool,
    pub move_down: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub shoot: bool,
    pub rescue: bool,
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
        (if self.move_up { 1.0 } else { 0.0 }) + (if self.move_down { -1.0 } else { 0.0 })
    }
}

pub struct Plug;

impl Plugin for Plug {
    fn build(&self, app: &mut App) {
        app.insert_resource(init_bindings())
            .insert_resource(init_controls())
            .add_systems(Update, input.run_if(in_state(Simulation::Running)));
    }
}

fn init_bindings() -> Bindings {
    use Bind::*;
    Bindings {
        move_up: Key(KeyCode::W),
        move_down: Key(KeyCode::S),
        move_left: Key(KeyCode::A),
        move_right: Key(KeyCode::D),
        shoot: Key(KeyCode::Space),
        rescue: Key(KeyCode::ControlLeft),
    }
}

fn init_controls() -> Controls {
    Controls {
        move_up: false,
        move_down: false,
        move_left: false,
        move_right: false,
        shoot: false,
        rescue: false,
    }
}

fn input(
    key: Res<Input<KeyCode>>,
    button: Res<Input<MouseButton>>,
    bindings: Res<Bindings>,
    mut commands: Commands,
) {
    let get = |x| match x {
        Bind::Key(v) => key.pressed(v),
        Bind::_Button(v) => button.pressed(v),
    };
    commands.insert_resource(Controls {
        move_up: get(bindings.move_up),
        move_down: get(bindings.move_down),
        move_left: get(bindings.move_left),
        move_right: get(bindings.move_right),
        shoot: get(bindings.shoot),
        rescue: get(bindings.rescue),
    });
}
