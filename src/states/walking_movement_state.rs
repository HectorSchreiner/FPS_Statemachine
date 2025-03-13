use godot::builtin::Vector3;
use godot::classes::Input;
use godot::global::godot_print;
use godot::obj::WithBaseField;

use crate::FPSController;
use crate::FSMState;
use crate::MovementState;
pub struct WalkingMovementState;

impl FSMState for WalkingMovementState {
    fn enter(player: &mut FPSController) {
        player.speed = 5.0;
        godot_print!("Entered Walking State")
    }

    fn exit(player: &mut FPSController) {
        godot_print!("Exited Walking State")
    }

    fn update(player: &mut FPSController, delta: f64) {}

    fn handle_input(
        player: &mut FPSController,
        event: godot::prelude::Gd<godot::classes::InputEvent>,
    ) {
    }

    fn physics_update(player: &mut FPSController, delta: f64) {
        player.default_movement_listener();
        player.default_jump_listener();
    }
}
