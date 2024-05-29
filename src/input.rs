use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, handle_input);
    }
}


fn setup(mut commands: Commands) {
    commands.spawn(InputState {
        up: false,
        down: false,
        left: false,
        right: false,
        idle: true,
        shooting: false,
        boost: false
    });
}

fn handle_input(
    key: Res<ButtonInput<KeyCode>>,
    mut input_state: Query<&mut InputState>
) {
    let mut input_state = input_state.single_mut();
    input_state.up = key.pressed(KeyCode::KeyW);
    input_state.down = key.pressed(KeyCode::KeyS);
    input_state.left = key.pressed(KeyCode::KeyA);
    input_state.right = key.pressed(KeyCode::KeyD);
    input_state.idle = !key.pressed(KeyCode::KeyW) && !key.pressed(KeyCode::KeyS) && !key.pressed(KeyCode::KeyA) && !key.pressed(KeyCode::KeyD);
    input_state.shooting = key.pressed(KeyCode::Space);
    input_state.boost = key.pressed(KeyCode::KeyW) && key.pressed(KeyCode::ShiftLeft);
}

#[derive(Component)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub idle: bool,
    pub shooting: bool,
    pub boost: bool,
}




