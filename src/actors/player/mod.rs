use super::{
    utils::{ActorEntityCommands, Alignment},
    Actor, AppRegisteringActors,
};
use crate::action_behaviour::{
    actions::movement::MovementAction, behaviours::player::PlayerBehaviour,
};
use avian2d::collision::Collider;
use bevy::{ecs::system::SystemParam, prelude::*};

pub struct RegisterPlayer;

impl Plugin for RegisterPlayer {
    fn build(&self, app: &mut App) {
        app.register_actor::<Player>();
    }
}

pub struct Player {
    pub position: Vec2,
}

impl Actor for Player {
    type Param<'w, 's> = (Commands<'w, 's>, Res<'w, AssetServer>);

    fn spawn(self, param: <Self::Param<'_, '_> as SystemParam>::Item<'_, '_>) {
        info!("Spawn player");

        let (mut commands, asset_server) = param;

        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_translation(self.position.extend(0.0)),
                    texture: asset_server.load("textures/player.png"),
                    ..default()
                },
                PlayerBehaviour,
                MovementAction::new(100.0, 0.7),
            ))
            .character_with_hurtbox(Alignment::Player, Collider::circle(4.0));
    }
}
