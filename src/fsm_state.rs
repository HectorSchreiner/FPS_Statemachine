use crate::FPSController;
use godot::{classes::InputEvent, prelude::*};

pub trait FSMState {
    fn enter(player: &mut FPSController);
    fn exit(player: &mut FPSController);
    fn update(player: &mut FPSController, delta: f64);
    fn physics_update(player: &mut FPSController, delta: f64);
    fn handle_input(player: &mut FPSController, event: Gd<InputEvent>);
}
