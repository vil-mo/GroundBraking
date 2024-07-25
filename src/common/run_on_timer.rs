use std::marker::PhantomData;

use bevy::{ecs::component::ComponentHooks, prelude::*};

use crate::dynamic_initialization::{
    DynamicallyInitializedComponentHooks, DynamicallyInitializedSystems, EntitySystem,
    ScheduleToAddSystems,
};

pub struct RunOnTimer<T: EntitySystem> {
    pub timer: Timer,
    _pd: PhantomData<T>,
}

impl<T: EntitySystem> RunOnTimer<T> {
    #[inline]
    pub fn new(timer: Timer) -> Self {
        Self {
            timer,
            _pd: default(),
        }
    }
}

impl<T: EntitySystem<In = (), Out = ()>> Component for RunOnTimer<T> {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.dynamically_initialized::<Self>();
    }
}

impl<T: EntitySystem<In = (), Out = ()>> DynamicallyInitializedSystems for RunOnTimer<T> {
    fn systems() -> (ScheduleToAddSystems, bevy::ecs::schedule::SystemConfigs) {
        (
            ScheduleToAddSystems::Update,
            tick_run_on_timer::<T>.into_configs(),
        )
    }
}

fn tick_run_on_timer<T: EntitySystem<In = (), Out = ()>>(
    time: Res<Time>,
    mut query: Query<(&mut RunOnTimer<T>, T::Data), T::Filter>,
    mut param: ParamSet<(T::Param,)>,
) {
    for (mut timer, data) in query.iter_mut() {
        // It's a bug if timer finished several times this tick
        // But I can't put this system in a loop, because `run` requires ovnership of the data
        // And there is no such trait as Reborrow so that I can reborrow data safely
        // (I tried creating one in utils module, look at it if you interested :))
        //
        // For game jam this bug is negligible
        if timer.timer.just_finished() {
            T::run((), data, param.p0());
        }

        timer.timer.tick(time.delta());
    }
}
