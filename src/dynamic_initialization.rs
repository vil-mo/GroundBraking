use std::any::TypeId;

use bevy::{
    ecs::{
        component::ComponentHooks,
        schedule::SystemConfigs,
        world::DeferredWorld,
    },
    prelude::*,
};
use bevy_consumable_event::{ConsumableEventApp, ConsumableEventReader, ConsumableEvents};

pub struct DynamicInitializationPlugin;

impl Plugin for DynamicInitializationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DinamicInitializationRegistry>()
            .add_consumable_event::<AddSystemsToUpdate>()
            .add_consumable_event::<AddSystemsToFixedUpdate>()
            .add_systems(
                Last,
                (
                    add_dynamic_systems_to_fixed_update,
                    add_dynamic_systems_to_update,
                ),
            );
    }
}

type TypeIdSet = bevy::utils::hashbrown::HashSet<TypeId, bevy::utils::NoOpHash>;

/// Used to store, which types were dynamically initialized to the world. Usually used in generic contxets.
#[derive(Default, Resource)]
pub struct DinamicInitializationRegistry {
    pub registry: TypeIdSet,
}

// For jam hadcoded label is good enough
#[derive(Event)]
pub struct AddSystemsToUpdate(pub SystemConfigs);

fn add_dynamic_systems_to_update(
    mut to_add: ConsumableEventReader<AddSystemsToUpdate>,
    mut schedules: ResMut<Schedules>,
) {
    for AddSystemsToUpdate(systems) in to_add.read_and_consume_all() {
        schedules.add_systems(Update, systems);
    }
}

// For jam hadcoded label is good enough
#[derive(Event)]
pub struct AddSystemsToFixedUpdate(pub SystemConfigs);

fn add_dynamic_systems_to_fixed_update(
    mut to_add: ConsumableEventReader<AddSystemsToFixedUpdate>,
    mut schedules: ResMut<Schedules>,
) {
    for AddSystemsToFixedUpdate(systems) in to_add.read_and_consume_all() {
        schedules.add_systems(FixedUpdate, systems);
    }
}

/// Recommended to use in generic contexsts when you can't guarantee that system with a generic is added to the world.
///
/// Usual use case is to implement it for the component and set it's `on_add` hook to register it into the registry.
///
/// To add systems to the world, use [`AddSystemsToUpdate`] or similar events.
pub trait DinamicallyInitialized: Send + Sync + 'static {
    fn initialize(world: DeferredWorld);
}

pub enum ScheduleToAddSystems {
    Update,
    FixedUpdate,
}

pub trait DynamicallyInitializedSystems: DinamicallyInitialized {
    fn systems() -> (ScheduleToAddSystems, SystemConfigs);
}

impl<T: DynamicallyInitializedSystems> DinamicallyInitialized for T {
    fn initialize(mut world: DeferredWorld) {
        let (schedule, systems) = T::systems();

        match schedule {
            ScheduleToAddSystems::Update => {
                let mut events = world.resource_mut::<ConsumableEvents<AddSystemsToUpdate>>();
                events.send(AddSystemsToUpdate(systems));
            }
            ScheduleToAddSystems::FixedUpdate => {
                let mut events = world.resource_mut::<ConsumableEvents<AddSystemsToFixedUpdate>>();
                events.send(AddSystemsToFixedUpdate(systems));
            }
        }
    }
}

pub trait DynamicallyInitializedComponentHooks {
    fn generically_initialized<T: DinamicallyInitialized>(&mut self);
}

impl DynamicallyInitializedComponentHooks for ComponentHooks {
    fn generically_initialized<T: DinamicallyInitialized>(&mut self) {
        self.on_add(|mut world, _entity, _component_id| {
            let registry = world.resource_mut::<DinamicInitializationRegistry>();

            if !registry.registry.contains(&TypeId::of::<T>()) {
                T::initialize(world);
            }
        });
    }
}

