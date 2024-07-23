use avian2d::prelude::*;
use bevy::prelude::*;

use crate::action_behaviour::{Action, ActionBehaviourApp};

pub(super) fn register_movement_action(app: &mut App) {
    app.register_action::<MovementAction>();
}

#[derive(Component)]
pub struct MovementAction {
    pub direction: Vec2,
    pub max_speed: f32,
    pub acceleration: f32,
}

impl MovementAction {
    pub fn new(max_speed: f32, acceleration: f32) -> Self {
        Self {
            direction: Vec2::ZERO,
            max_speed,
            acceleration,
        }
    }
}

impl Action for MovementAction {
    fn systems() -> bevy::ecs::schedule::SystemConfigs {
        apply_movement.into_configs()
    }
}

fn apply_movement(mut query: Query<(&mut LinearVelocity, &MovementAction)>) {
    for (mut velocity, movement) in query.iter_mut() {
        let target = movement.direction * movement.max_speed;

        velocity.0 = velocity.0.lerp(target, movement.acceleration);
        // info!("Velocity: {}", velocity.0);
    }
}
