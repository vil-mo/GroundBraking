use std::any::TypeId;

use bevy::{
    ecs::{
        component::ComponentHooks,
        query::{QueryData, QueryFilter, WorldQuery},
        schedule::SystemConfigs,
        system::SystemParam,
        world::DeferredWorld,
    },
    prelude::*,
    utils::hashbrown::hash_set::Entry,
};
use bevy_consumable_event::{ConsumableEventApp, ConsumableEventReader, ConsumableEvents};

pub struct DynamicInitializationPlugin;

impl Plugin for DynamicInitializationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DynamicInitializationRegistry>()
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
pub struct DynamicInitializationRegistry {
    pub registry: TypeIdSet,
}

impl DynamicInitializationRegistry {
    pub fn initialize<T: DynamicallyInitialized>(mut world: DeferredWorld) {
        let mut registry = world.resource_mut::<DynamicInitializationRegistry>();

        match registry.registry.entry(TypeId::of::<T>()) {
            Entry::Occupied(_) => (),
            Entry::Vacant(vacant) => {
                vacant.insert();

                T::initialize(world);
            }
        }
    }
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
pub trait DynamicallyInitialized: Send + Sync + 'static {
    fn initialize(world: DeferredWorld);
}

pub enum ScheduleToAddSystems {
    Update,
    FixedUpdate,
}

pub trait DynamicallyInitializedSystems: DynamicallyInitialized {
    fn systems() -> (ScheduleToAddSystems, SystemConfigs);
}

impl<T: DynamicallyInitializedSystems> DynamicallyInitialized for T {
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
    fn dynamically_initialized<T: DynamicallyInitialized>(&mut self);
}

impl DynamicallyInitializedComponentHooks for ComponentHooks {
    fn dynamically_initialized<T: DynamicallyInitialized>(&mut self) {
        self.on_add(|world, _entity, _component_id| {
            DynamicInitializationRegistry::initialize::<T>(world);
        });
    }
}

pub type DataItem<'w, T> = <<T as EntitySystem>::Data as WorldQuery>::Item<'w>;
pub type ParamItem<'w, 's, T> = <<T as EntitySystem>::Param as SystemParam>::Item<'w, 's>;

/// A system that operates on a single entity
/// that should be fetched from the world using query
/// `Query<<T as EntitySystem>::Data, <T as EntitySystem>::Filter>`
pub trait EntitySystem: Send + Sync + 'static {
    type Data: QueryData;
    type Filter: QueryFilter;
    type Param: SystemParam;

    type In;
    type Out;

    fn run(input: Self::In, data: DataItem<'_, Self>, param: ParamItem<'_, '_, Self>) -> Self::Out;
}

impl EntitySystem for () {
    type Data = ();
    type Filter = ();
    type Param = ();

    type In = ();
    type Out = ();

    fn run(
        _: Self::In,
        _: crate::dynamic_initialization::DataItem<'_, Self>,
        _: crate::dynamic_initialization::ParamItem<'_, '_, Self>,
    ) -> Self::Out {
    }
}
