use godot::builtin::Vector3;
use godot::classes::Input;
use godot::global::godot_print;
use godot::obj::WithBaseField;

use crate::FPSController;
use crate::FSMState;
use crate::MovementState;
pub struct JumpingMovementState;

impl FSMState for JumpingMovementState {
    fn enter(player: &mut FPSController) {
        player.speed = 2.0;
        player.velocity.y = player.jump_force;
        godot_print!("Entered Jumping State");
    }

    fn exit(player: &mut FPSController) {
        godot_print!("Exited Jumping State")
    }

    fn update(player: &mut FPSController, delta: f64) {}

    fn handle_input(
        player: &mut FPSController,
        event: godot::prelude::Gd<godot::classes::InputEvent>,
    ) {
    }

    fn physics_update(player: &mut FPSController, delta: f64) {
        //player.default_movement_listener();
    }
}
