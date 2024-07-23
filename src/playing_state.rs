use bevy::prelude::*;
use bevy_consumable_event::ConsumableEventWriter;

use crate::{
    actors::{player::Player, SpawnActor},
    GameState,
};

pub struct PlayingPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_layout)
            .add_systems(OnExit(GameState::Playing), cleanup_layout);
    }
}

fn setup_layout(mut player_spawn: ConsumableEventWriter<SpawnActor<Player>>) {
    info!("Playing");
    player_spawn.send(SpawnActor(Player {
        position: Vec2::new(0.0, 0.0),
    }));
}

fn cleanup_layout() {}
