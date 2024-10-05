use bevy::prelude::*;

// system to be added to update player
fn update_player() {
    // for something in PlayerCharacterComponent
} 

#[derive(Component)]
pub struct PlayerCharacterComponent {
    state: PlayerState,
    input: PlayerInput,
    transform: bevy::prelude::Transform,
    velocity: Vec3,
    collision: PlayerCollision,
}

enum PlayerState {
    Normal
}

enum PlayerInput {
    None,
    Forward,
    Back,
    Left,
    Right,
    Jump,
}

#[derive(Component)]
struct PlayerCollision;

impl PlayerCharacterComponent {
    pub fn new() -> PlayerCharacterComponent {
         let comp = PlayerCharacterComponent{
            state: PlayerState::Normal,
            input: PlayerInput::None,
            transform: Transform{
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
                scale: Vec3::new(0.0, 0.0, 0.0)
            },
            velocity: Vec3::new(0.0, 0.0, 0.0),
            collision: PlayerCollision
        };
         comp
    }
    fn tick(&self) {
        
    }

    fn keyboard_input(
        mut self,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        self.input = PlayerInput::None;
        if keys.just_pressed(KeyCode::Space) {
            // Space was pressed
            self.input = PlayerInput::Jump;
        }
        if keys.pressed(KeyCode::KeyW) {
            // W is being held down
            self.input = PlayerInput::Forward;
        }
        if keys.pressed(KeyCode::KeyA) {
            // A is being held down
            self.input = PlayerInput::Left;
        }
        if keys.pressed(KeyCode::KeyS) {
            // S is being held down
            self.input = PlayerInput::Back;
        }
        if keys.pressed(KeyCode::KeyD) {
            // D is being held down
            self.input = PlayerInput::Right;
        }

        
        /*
        // we can check multiple at once with `.any_*`
        if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            // Either the left or right shift are being held down
        }
        if keys.any_just_pressed([KeyCode::Delete, KeyCode::Backspace]) {
            // Either delete or backspace was just pressed
        }
            if keys.just_released(KeyCode::ControlLeft) {
            // Left Ctrl was released
        }
        */
    }
}



