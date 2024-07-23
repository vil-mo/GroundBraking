use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_consumable_event::*;

use crate::action_behaviour::{actions::emit_projectile::EmitProjectileAction, ActionBehaviourApp};

pub(self) mod utils;

pub mod dasher;
pub mod player;

pub struct RegisterActors;
impl Plugin for RegisterActors {
    fn build(&self, app: &mut App) {
        app.add_plugins((utils::ActorUtilsPlugin, player::RegisterPlayer));
    }
}

pub trait Actor: Send + Sync + 'static {
    type Param<'w, 's>: SystemParam;

    fn spawn(self, param: <Self::Param<'_, '_> as SystemParam>::Item<'_, '_>);
}

#[derive(Event, Default, Deref, DerefMut)]
pub struct SpawnActor<A: Actor>(pub A);

pub trait AppRegisteringActors {
    fn register_actor<T: Actor>(&mut self) -> &mut Self;
}

impl AppRegisteringActors for App {
    fn register_actor<A: Actor>(&mut self) -> &mut Self {
        // Idk better way to do it, but this is exceptional case and this architecture is pretty neat
        self.register_action::<EmitProjectileAction<A>>();

        self.add_persistent_consumable_event::<SpawnActor<A>>()
            .add_systems(Update, spawn_actor_system::<A>)
    }
}

fn spawn_actor_system<A: Actor>(
    mut events: ConsumableEventReader<SpawnActor<A>>,
    mut param: ParamSet<(A::Param<'_, '_>,)>,
) {
    for event in events.read_and_consume_all() {
        event.0.spawn(param.p0())
    }
}
