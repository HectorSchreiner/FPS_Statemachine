use godot::global::godot_print;

use crate::{FPSController, FSMState, MovementState};
pub struct RunningMovementState;

impl FSMState for RunningMovementState {
    fn enter(player: &mut FPSController) {
        player.speed = 7.0;
        godot_print!("Entered Running State")
    }

    fn exit(player: &mut FPSController) {
        godot_print!("Exited Running State")
    }

    fn update(player: &mut FPSController, delta: f64) {}

    fn handle_input(
        player: &mut FPSController,
        event: godot::prelude::Gd<godot::classes::InputEvent>,
    ) {
        if event.is_action_released("sprint") {
            player.change_state(MovementState::Walking);
        }
    }

    fn physics_update(player: &mut FPSController, delta: f64) {
        if player.velocity.length() > 0.0 {
            player.default_movement_listener();
            player.default_jump_listener();
        }
    }
}
