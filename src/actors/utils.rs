use std::ops::Not;

use avian2d::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, utils::Duration};

pub struct ActorUtilsPlugin;

impl Plugin for ActorUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tick_fade_away, tick_disable_collider_on_time));
    }
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Player,
    Enemy,
}

impl Alignment {
    #[inline]
    fn hitbox(self) -> CollisionMask {
        match self {
            Alignment::Player => CollisionMask::PlayerHitbox,
            Alignment::Enemy => CollisionMask::EnemyHitbox,
        }
    }

    #[inline]
    fn hurtbox(self) -> CollisionMask {
        match self {
            Alignment::Player => CollisionMask::PlayerHurtbox,
            Alignment::Enemy => CollisionMask::EnemyHurtbox,
        }
    }
}

impl Not for Alignment {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Alignment::Player => Alignment::Enemy,
            Alignment::Enemy => Alignment::Player,
        }
    }
}

#[derive(PhysicsLayer)]
pub enum CollisionMask {
    Character,
    Projectile,

    PlayerHitbox,
    EnemyHitbox,

    PlayerHurtbox,
    EnemyHurtbox,
}

pub trait ActorEntityCommands {
    fn with_hitbox(&mut self, alignment: Alignment, hitbox: Collider) -> &mut Self;

    fn with_hitbox_for(
        &mut self,
        alignment: Alignment,
        hitbox: Collider,
        hitbox_lifetime: Duration,
    ) -> &mut Self;

    fn with_hurtbox(&mut self, alignment: Alignment, hurtbox: Collider) -> &mut Self;

    /// Characters collide with walls (don't pass through them).
    /// Dynamic Rigidbodies
    fn character_with_hurtbox(&mut self, alignment: Alignment, hurtbox: Collider) -> &mut Self;

    /// Projectiles detect collisoions with walls (may pass through them).
    /// Kinematic Rigidbodies
    fn projectile_with_hitbox(&mut self, alignment: Alignment, hitbox: Collider) -> &mut Self;

    fn fade_away(&mut self, lifetime: Duration) -> &mut Self;
}

impl ActorEntityCommands for EntityCommands<'_> {
    fn with_hitbox(&mut self, alignment: Alignment, hitbox: Collider) -> &mut Self {
        let hitbox_collision_layers =
            CollisionLayers::new(alignment.hitbox(), (!alignment).hurtbox());

        self.insert((
            RigidBody::Kinematic,
            hitbox,
            Sensor,
            hitbox_collision_layers,
        ))
    }

    /// Uses [`DisableColliderOnTimer`]
    fn with_hitbox_for(
        &mut self,
        alignment: Alignment,
        hitbox: Collider,
        hitbox_lifetime: Duration,
    ) -> &mut Self {
        let hitbox_collision_layers =
            CollisionLayers::new(alignment.hitbox(), (!alignment).hurtbox());

        self.insert((
            RigidBody::Kinematic,
            hitbox,
            Sensor,
            hitbox_collision_layers,
            DisableColliderOnTimer {
                timer: Timer::new(hitbox_lifetime, TimerMode::Once),
            },
        ))
    }

    fn with_hurtbox(&mut self, alignment: Alignment, hurtbox: Collider) -> &mut Self {
        let hurtbox_collision_layers =
            CollisionLayers::new(alignment.hurtbox(), (!alignment).hitbox());

        self.insert((
            RigidBody::Kinematic,
            hurtbox,
            Sensor,
            hurtbox_collision_layers,
        ))
    }

    fn character_with_hurtbox(&mut self, alignment: Alignment, hurtbox: Collider) -> &mut Self {
        let hurtbox_collision_layers = CollisionLayers::new(
            [CollisionMask::Character, alignment.hurtbox()],
            (!alignment).hitbox(),
        );

        self.insert((
            RigidBody::Kinematic,
            hurtbox,
            Sensor,
            hurtbox_collision_layers,
        ))
    }

    fn projectile_with_hitbox(&mut self, alignment: Alignment, hitbox: Collider) -> &mut Self {
        let hitbox_collision_layers = CollisionLayers::new(
            [CollisionMask::Projectile, alignment.hitbox()],
            (!alignment).hurtbox(),
        );

        self.insert((
            RigidBody::Kinematic,
            hitbox,
            Sensor,
            hitbox_collision_layers,
        ))
    }

    #[inline]
    fn fade_away(&mut self, lifetime: Duration) -> &mut Self {
        self.insert(FadeAway {
            timer: Timer::new(lifetime, TimerMode::Once),
        })
    }
}

#[derive(Component)]
pub struct FadeAway {
    timer: Timer,
}

fn tick_fade_away(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &mut FadeAway,
        Option<&mut Sprite>,
        Option<&Handle<ColorMaterial>>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (entity, mut fade, sprite, material_handle) in query.iter_mut() {
        if fade.timer.finished() {
            commands.entity(entity).despawn_recursive();
            return;
        }

        fade.timer.tick(time.delta());
        let new_alpha = 1. - fade.timer.fraction();
        if let Some(mut sprite) = sprite {
            sprite.color.set_alpha(new_alpha);
        }
        if let Some(material_handle) = material_handle {
            if let Some(material) = materials.get_mut(material_handle) {
                material.color.set_alpha(new_alpha);
            }
        }
    }
}

/// Disales collider by switching it's layers to [`CollisionLayers::NONE`]
#[derive(Component)]
pub struct DisableColliderOnTimer {
    timer: Timer,
}

fn tick_disable_collider_on_time(
    time: Res<Time>,
    mut query: Query<(&mut DisableColliderOnTimer, &mut CollisionLayers)>,
) {
    for (mut timer, mut layers) in query.iter_mut() {
        if timer.timer.finished() {
            if timer.timer.just_finished() {
                *layers = CollisionLayers::NONE;
            }
            return;
        }

        timer.timer.tick(time.delta());
    }
}
