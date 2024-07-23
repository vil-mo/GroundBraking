use bevy::prelude::*;

use crate::{
    action_behaviour::{actions::movement::MovementAction, ActionBehaviourApp, Behaviour},
    input_map::InputMap,
};

pub(super) fn register_player_behvaiour(app: &mut App) {
    app.register_behaviour::<PlayerBehaviour>();
}

#[derive(Component)]
pub struct PlayerBehaviour;

impl Behaviour for PlayerBehaviour {
    fn systems() -> bevy::ecs::schedule::SystemConfigs {
        player_movement.into_configs()
    }
}

fn player_movement(
    mut query: Query<&mut MovementAction, With<PlayerBehaviour>>,
    input: Res<InputMap>,
) {
    for mut movement in query.iter_mut() {
        movement.direction = input.movement_direction;
        // info!("Player movement: {}", input.movement_direction);
    }
}
