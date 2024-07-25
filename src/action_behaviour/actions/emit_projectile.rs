use bevy::prelude::*;
use bevy_consumable_event::ConsumableEventWriter;

use crate::{
    action_behaviour::Action,
    actors::{Actor, SpawnActor},
};

// Already registered in `register_actor`

#[derive(Component)]
pub struct EmitProjectile<T: Actor> {
    should_emit: Vec<T>,
}

impl<T: Actor> Default for EmitProjectile<T> {
    fn default() -> Self {
        EmitProjectile {
            should_emit: Vec::new(),
        }
    }
}

impl<T: Actor> EmitProjectile<T> {
    pub fn emit(&mut self, projectile: T) {
        self.should_emit.push(projectile);
    }
}

impl<T: Actor> Action for EmitProjectile<T> {
    fn systems() -> bevy::ecs::schedule::SystemConfigs {
        apply_emit_projectile::<T>.into_configs()
    }
}

fn apply_emit_projectile<T: Actor>(
    mut query: Query<&mut EmitProjectile<T>>,
    mut event: ConsumableEventWriter<SpawnActor<T>>,
) {
    for mut action in query.iter_mut() {
        for p in action.should_emit.drain(..) {
            event.send(SpawnActor(p));
        }
    }
}
