mod fsm_state;
mod states;

use godot::classes::{CharacterBody3D, ICharacterBody3D, Input, InputEvent};
use godot::obj::WithBaseField;
use godot::prelude::*;

use fsm_state::FSMState;
use states::{
    crouching_movement_state::*, idle_movement_state::*, jumping_movement_state::*,
    running_movement_state::*, walking_movement_state::*,
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct FPSController {
    speed: f32,
    velocity: Vector3,
    acceleration: f32,
    decceleration: f32,
    jump_force: f32,
    gravity: f32,
    state: MovementState,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for FPSController {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 0.0,
            velocity: Vector3::ZERO,
            acceleration: 0.25,
            decceleration: 0.25,
            jump_force: 5.0,
            gravity: 0.25,
            state: MovementState::Idle,
            base,
        }
    }

    fn ready(&mut self) {
        self.change_state(MovementState::Idle);
    }

    fn process(&mut self, delta: f64) {
        match self.state {
            MovementState::Idle => IdleMovementState::update(self, delta),
            MovementState::Walking => WalkingMovementState::update(self, delta),
            MovementState::Running => RunningMovementState::update(self, delta),
            MovementState::Crouching => CrouchingMovementState::update(self, delta),
            MovementState::Jumping => JumpingMovementState::update(self, delta),
        }
    }

    fn physics_process(&mut self, delta: f64) {
        godot_print!("{:?}", self.velocity);
        match self.state {
            MovementState::Idle => IdleMovementState::physics_update(self, delta),
            MovementState::Walking => WalkingMovementState::physics_update(self, delta),
            MovementState::Running => RunningMovementState::physics_update(self, delta),
            MovementState::Crouching => CrouchingMovementState::physics_update(self, delta),
            MovementState::Jumping => JumpingMovementState::physics_update(self, delta),
        }
        self.apply_gravity(delta);
        self.move_player(delta);
    }

    fn input(&mut self, input: godot::prelude::Gd<InputEvent>) {
        match self.state {
            MovementState::Idle => IdleMovementState::handle_input(self, input),
            MovementState::Walking => WalkingMovementState::handle_input(self, input),
            MovementState::Running => RunningMovementState::handle_input(self, input),
            MovementState::Crouching => CrouchingMovementState::handle_input(self, input),
            MovementState::Jumping => JumpingMovementState::handle_input(self, input),
        }
    }
}

impl FPSController {
    pub fn change_state(&mut self, new_state: MovementState) {
        let prev = &self.state;
        // is state is new, change the state
        if !prev.eq(&new_state) {
            match prev {
                MovementState::Idle => IdleMovementState::exit(self),
                MovementState::Walking => WalkingMovementState::exit(self),
                MovementState::Running => RunningMovementState::exit(self),
                MovementState::Crouching => CrouchingMovementState::exit(self),
                MovementState::Jumping => JumpingMovementState::exit(self),
            }

            match new_state {
                MovementState::Idle => IdleMovementState::enter(self),
                MovementState::Walking => WalkingMovementState::enter(self),
                MovementState::Running => RunningMovementState::enter(self),
                MovementState::Crouching => CrouchingMovementState::enter(self),
                MovementState::Jumping => JumpingMovementState::enter(self),
            }

            self.state = new_state;
        }
    }

    pub fn default_movement_listener(&mut self) {
        let input = Input::singleton();
        self.velocity = Vector3::ZERO;

        if input.is_action_pressed("left") {
            self.velocity.x -= 1.0;
        }
        if input.is_action_pressed("right") {
            self.velocity.x += 1.0;
        }
        if input.is_action_pressed("forward") {
            self.velocity.z -= 1.0;
        }
        if input.is_action_pressed("backward") {
            self.velocity.z += 1.0;
        }

        if self.velocity.length() > 0.0 {
            self.velocity = self.velocity.normalized() * self.speed;
        } else {
            self.velocity = Vector3::ZERO;
            self.change_state(MovementState::Idle);
        }
        if input.is_action_pressed("sprint") && self.velocity.length() > 0.0 {
            self.change_state(MovementState::Running);
        }
    }

    pub fn default_jump_listener(&mut self) {
        let input = Input::singleton();
        if input.is_action_just_pressed("jump") && self.base().is_on_floor() {
            self.change_state(MovementState::Jumping);
        }
    }

    pub fn move_player(&mut self, delta: f64) {
        let velocity = self.velocity;
        self.base_mut().move_and_collide(velocity * delta as f32);
        self.base_mut().move_and_slide();
        // self.velocity = Vector3::ZERO;

        // debugging
        /*
        godot_print!(
            "Position: {}\n Velocity: {}\nSpeed: {}\nState: {:?}",
            position,
            self.velocity,
            self.speed,
            self.state,
        )*/
    }

    pub fn apply_gravity(&mut self, delta: f64) {
        if self.base().is_on_floor() {
            self.velocity.y = 0.0;
        }
        if !self.base().is_on_floor() {
            self.velocity.y -= self.gravity;
        }
        self.move_player(delta);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MovementState {
    Idle,
    Walking,
    Running,
    Crouching,
    Jumping,
}
