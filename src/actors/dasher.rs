use super::{
    utils::{ActorEntityCommands, Alignment},
    Actor, AppRegisteringActors,
};
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::Duration};

pub struct RegisterDasher;

impl Plugin for RegisterDasher {
    fn build(&self, app: &mut App) {
        app.register_actor::<Dasher>();
    }
}

const ATTACK_COLOR: Color = Color::linear_rgb(1., 0., 0.);
const ATTACK_LIFETIME: Duration = Duration::from_millis(500); // 0.5 secs
const ATTACK_HITBOX_LIFETIME: Duration = Duration::from_millis(75); // 0.075 secs

struct DasherAttack {
    position: Vec2,
    radius: f32,
}

impl Actor for DasherAttack {
    type Param<'w, 's> = (
        Commands<'w, 's>,
        ResMut<'w, Assets<Mesh>>,
        ResMut<'w, Assets<ColorMaterial>>,
    );

    fn spawn(self, param: <Self::Param<'_, '_> as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (mut commands, mut meshes, mut materials) = param;
        commands
            .spawn((MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(self.radius)).into(),
                material: materials.add(ColorMaterial::from_color(ATTACK_COLOR)),
                transform: Transform::from_translation(self.position.extend(0.0)),
                ..default()
            },))
            .with_hitbox_for(
                super::utils::Alignment::Enemy,
                Collider::circle(self.radius),
                ATTACK_HITBOX_LIFETIME,
            )
            .fade_away(ATTACK_LIFETIME);
    }
}

pub struct Dasher {
    position: Vec2,
}

impl Actor for Dasher {
    type Param<'w, 's> = (Commands<'w, 's>, ResMut<'w, AssetServer>);
    fn spawn(self, param: <Self::Param<'_, '_> as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (mut commands, mut asset_server) = param;

        commands
            .spawn((SpriteBundle {
                transform: Transform::from_translation(self.position.extend(0.0)),
                texture: asset_server.load("textures/dasher.png"),
                ..default()
            },))
            .character_with_hurtbox(Alignment::Enemy, Collider::rectangle(12.0, 11.0));
    }
}
