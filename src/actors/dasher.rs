use crate::{
    action_behaviour::actions::emit_projectile::EmitProjectile,
    common::{
        colliders::{Alignment, CollidersCommands},
        run_on_timer::RunOnTimer,
        CommonEntityCommands,
    },
    dynamic_initialization::{DataItem, EntitySystem, ParamItem},
};

use super::{Actor, AppRegisteringActors};
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::Duration};

pub struct RegisterDasher;

impl Plugin for RegisterDasher {
    fn build(&self, app: &mut App) {
        app.register_actor::<Dasher>()
            .register_actor::<DasherAttack>();
    }
}

const ATTACK_COLOR: Color = Color::linear_rgb(1., 0., 0.);
const ATTACK_LIFETIME: Duration = Duration::from_millis(500); // 0.5 secs
const ATTACK_HITBOX_LIFETIME: Duration = Duration::from_millis(75); // 0.075 secs
const DASHER_ATTACK_RADIUS: f32 = 20.0;
const DASHER_ATTACK_PERIOD: Duration = Duration::from_secs(3);

struct DasherAttack {
    position: Vec2,
    radius: f32,
}

impl Actor for DasherAttack {
    type Param = (
        Commands<'static, 'static>,
        ResMut<'static, Assets<Mesh>>,
        ResMut<'static, Assets<ColorMaterial>>,
    );

    fn spawn(self, param: <Self::Param as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (mut commands, mut meshes, mut materials) = param;

        commands
            .spawn((MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(self.radius)).into(),
                material: materials.add(ColorMaterial::from_color(ATTACK_COLOR)),
                transform: Transform::from_translation(self.position.extend(0.0)),
                ..default()
            },))
            .with_hitbox_for(
                Alignment::Enemy,
                Collider::circle(self.radius),
                ATTACK_HITBOX_LIFETIME,
            )
            .fade_away(ATTACK_LIFETIME);
    }
}

struct DasherPeriodicAction;

impl EntitySystem for DasherPeriodicAction {
    type Data = (
        &'static mut EmitProjectile<DasherAttack>,
        &'static Transform,
    );
    type Filter = ();
    type Param = ();

    type In = ();
    type Out = ();

    fn run(_: Self::In, data: DataItem<'_, Self>, param: ParamItem<'_, '_, Self>) {
        let (mut emit_projectile, transform) = data;
        let () = param;

        emit_projectile.emit(DasherAttack {
            position: transform.translation.xy(),
            radius: DASHER_ATTACK_RADIUS,
        });
    }
}

pub struct Dasher {
    pub position: Vec2,
}

impl Actor for Dasher {
    type Param = (Commands<'static, 'static>, Res<'static, AssetServer>);
    fn spawn(self, param: <Self::Param as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (mut commands, asset_server) = param;

        commands
            .spawn((
                SpriteBundle {
                    transform: Transform::from_translation(self.position.extend(0.0)),
                    texture: asset_server.load("textures/dasher.png"),
                    ..default()
                },
                EmitProjectile::<DasherAttack>::default(),
                RunOnTimer::<DasherPeriodicAction>::new(Timer::new(
                    DASHER_ATTACK_PERIOD,
                    TimerMode::Repeating,
                )),
            ))
            .character_with_hurtbox(Alignment::Enemy, Collider::rectangle(12.0, 11.0));
    }
}
