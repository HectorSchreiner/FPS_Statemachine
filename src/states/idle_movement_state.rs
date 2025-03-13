use godot::global::godot_print;

use crate::{FPSController, FSMState, MovementState};
pub struct IdleMovementState;

impl FSMState for IdleMovementState {
    fn enter(player: &mut FPSController) {
        godot_print!("Entered Idle State")
    }

    fn exit(player: &mut FPSController) {
        godot_print!("Exited Idle State")
    }

    fn update(player: &mut FPSController, delta: f64) {}

    fn handle_input(
        player: &mut FPSController,
        event: godot::prelude::Gd<godot::classes::InputEvent>,
    ) {
        if event.is_action_pressed("forward")
            || event.is_action_pressed("backward")
            || event.is_action_pressed("left")
            || event.is_action_pressed("right")
        {
            player.change_state(MovementState::Walking);
        }
    }

    fn physics_update(player: &mut crate::FPSController, delta: f64) {
        player.default_jump_listener();
    }
}
