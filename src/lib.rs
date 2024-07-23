#![allow(clippy::type_complexity)]

pub mod action_behaviour;
pub mod actors;
pub mod dynamic_initialization;
pub mod input_map;
pub mod menu_state;
pub mod playing_state;

use crate::{
    action_behaviour::ActionBehaviourPlugin, actors::RegisterActors, input_map::InputMapPlugin,
    menu_state::MenuPlugin,
};
use avian2d::prelude::*;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use playing_state::PlayingPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // Here the menu is drawn and waiting for player interaction
    #[default]
    Menu,

    // During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            PhysicsPlugins::default(),
            MenuPlugin,
            PlayingPlugin,
            InputMapPlugin,
            ActionBehaviourPlugin,
            RegisterActors,
        ));

        #[cfg(debug_assertions)]
        {
            //app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
