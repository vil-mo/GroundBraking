#![allow(clippy::type_complexity)]

pub mod action_behaviour;
pub mod actors;
pub mod common;
pub mod dynamic_initialization;
pub mod input_map;
pub mod menu_state;
pub mod playing_state;
pub mod utils;

use crate::{
    action_behaviour::ActionBehaviourPlugin, actors::RegisterActors, input_map::InputMapPlugin,
    menu_state::MenuPlugin,
};
use avian2d::prelude::*;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use common::CommonPlugin;
use dynamic_initialization::DynamicInitializationPlugin;
use playing_state::PlayingPlugin;

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
            DynamicInitializationPlugin,
            CommonPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            //app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
