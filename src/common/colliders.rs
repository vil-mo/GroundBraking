use std::ops::Not;

use avian2d::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*, utils::Duration};

pub struct CollidersPlugin;

impl Plugin for CollidersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_disable_collider_on_time);
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

pub trait CollidersCommands {
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
}

impl CollidersCommands for EntityCommands<'_> {
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
        if timer.timer.just_finished() {
            *layers = CollisionLayers::NONE;
        }

        timer.timer.tick(time.delta());
    }
}
