use std::time::Duration;

use bevy::prelude::*;
use bevy_consumable_event::ConsumableEventWriter;

use crate::{
    action_behaviour::{actions::movement::MovementAction, ActionBehaviourApp, Behaviour},
    input_map::InputMap,
    playing_state::RemoveTile,
};

pub(super) fn register_player_behvaiour(app: &mut App) {
    app.register_behaviour::<PlayerBehaviour>();
}

#[derive(Component)]
pub struct PlayerBehaviour;

const REMOVE_TILE_DURATION: Duration = Duration::from_millis(2000);

impl Behaviour for PlayerBehaviour {
    fn systems() -> bevy::ecs::schedule::SystemConfigs {
        (player_movement, player_tile_destruction).into_configs()
    }
}

fn player_movement(
    mut query: Query<&mut MovementAction, With<PlayerBehaviour>>,
    input: Res<InputMap>,
) {
    for mut movement in query.iter_mut() {
        movement.direction = input.movement_direction();
    }
}

fn player_tile_destruction(
    query: Query<&Transform, With<PlayerBehaviour>>,
    mut input: ResMut<InputMap>,
    mut event: ConsumableEventWriter<RemoveTile>,
) {
    for transform in query.iter() {
        if input.destroy_tile() {
            let pos = transform.translation.xy();
            event.send(RemoveTile(pos, REMOVE_TILE_DURATION));
        }
    }
}
