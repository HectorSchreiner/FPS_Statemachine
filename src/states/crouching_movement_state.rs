use godot::builtin::Vector3;
use godot::classes::Input;
use godot::global::godot_print;

use crate::FPSController;
use crate::FSMState;
use crate::MovementState;
pub struct CrouchingMovementState;

impl FSMState for CrouchingMovementState {
    fn enter(player: &mut FPSController) {
        player.speed = 2.5;
        godot_print!("Entered Crouching State")
    }

    fn exit(player: &mut FPSController) {
        godot_print!("Exited Crouching State")
    }

    fn update(player: &mut FPSController, delta: f64) {}

    fn handle_input(
        player: &mut FPSController,
        event: godot::prelude::Gd<godot::classes::InputEvent>,
    ) {
    }

    fn physics_update(player: &mut FPSController, delta: f64) {
        player.default_movement_listener();
    }
}
