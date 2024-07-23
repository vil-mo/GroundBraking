use bevy::prelude::*;

pub mod player;

pub struct RegisterBehaviours;

impl Plugin for RegisterBehaviours {
    fn build(&self, app: &mut App) {
        player::register_player_behvaiour(app);
    }
}
