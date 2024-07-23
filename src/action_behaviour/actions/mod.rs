use bevy::prelude::*;

pub mod movement;
// Already registered in `register_actor`
pub mod emit_projectile;

pub struct RegisterActions;

impl Plugin for RegisterActions {
    fn build(&self, app: &mut App) {
        movement::register_movement_action(app);
    }
}
