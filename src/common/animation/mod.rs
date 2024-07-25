use crate::dynamic_initialization::{
    DynamicallyInitializedComponentHooks, DynamicallyInitializedSystems, EntitySystem,
    ScheduleToAddSystems,
};
use bevy::{ecs::component::StorageType, prelude::*};
use std::marker::PhantomData;

pub mod destroy;
pub mod disable;
pub mod fade_away;
pub mod show_up;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, _app: &mut App) {}
}

pub struct Animation<
    Tick: EntitySystem<In = f32, Out = ()>,
    Finished: EntitySystem<In = (), Out = ()>,
    Marker: Send + Sync + 'static = (),
> {
    disabled: bool,
    timer: Timer,
    _pd: PhantomData<(Tick, Finished, Marker)>,
}

impl<
        Tick: EntitySystem<In = f32, Out = ()>,
        Finished: EntitySystem<In = (), Out = ()>,
        Marker: Send + Sync + 'static,
    > Component for Animation<Tick, Finished, Marker>
{
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.dynamically_initialized::<Self>();
    }
}

impl<
        Tick: EntitySystem<In = f32, Out = ()>,
        Finished: EntitySystem<In = (), Out = ()>,
        Marker: Send + Sync + 'static,
    > DynamicallyInitializedSystems for Animation<Tick, Finished, Marker>
{
    fn systems() -> (ScheduleToAddSystems, bevy::ecs::schedule::SystemConfigs) {
        (
            ScheduleToAddSystems::Update,
            (
                tick_animation::<Tick, Finished>,
                tick_system_animation::<Tick, Finished>,
                animation_finished::<Tick, Finished>,
            )
                .chain(),
        )
    }
}

impl<
        Tick: EntitySystem<In = f32, Out = ()>,
        Finished: EntitySystem<In = (), Out = ()>,
        Marker: Send + Sync + 'static,
    > Animation<Tick, Finished, Marker>
{
    pub fn new(timer: Timer) -> Self {
        Self {
            disabled: false,
            timer,
            _pd: PhantomData,
        }
    }

    pub fn new_disabled(timer: Timer) -> Self {
        Self {
            disabled: false,
            timer,
            _pd: PhantomData,
        }
    }

    #[inline]
    pub fn disable(&mut self) {
        self.disabled = true;
    }

    #[inline]
    pub fn enable(&mut self) {
        self.disabled = false;
        self.timer.reset();
    }
}

fn tick_animation<
    Tick: EntitySystem<In = f32, Out = ()>,
    Finished: EntitySystem<In = (), Out = ()>,
>(
    time: Res<Time>,
    mut query: Query<&mut Animation<Tick, Finished>>,
) {
    for mut timer in query.iter_mut() {
        if !timer.disabled {
            timer.timer.tick(time.delta());
        }
    }
}

fn tick_system_animation<
    Tick: EntitySystem<In = f32, Out = ()>,
    Finished: EntitySystem<In = (), Out = ()>,
>(
    mut query: Query<(&Animation<Tick, Finished>, Tick::Data), Tick::Filter>,
    mut param: ParamSet<(Tick::Param,)>,
) {
    for (timer, data) in query.iter_mut() {
        if !timer.disabled {
            Tick::run(timer.timer.fraction(), data, param.p0());
        }
    }
}

fn animation_finished<
    Tick: EntitySystem<In = f32, Out = ()>,
    Finished: EntitySystem<In = (), Out = ()>,
>(
    mut query: Query<(&Animation<Tick, Finished>, Finished::Data), Finished::Filter>,
    mut param: ParamSet<(Finished::Param,)>,
) {
    for (timer, data) in query.iter_mut() {
        // It's a bug if timer finished several times this tick
        // But I can't put this system in a loop, because `run` requires ovnership of the data
        // And there is no such trait as Reborrow so that I can reborrow data safely
        // (I tried creating one in utils module, look at it if you interested :))
        //
        // For game jam this bug is negligible
        if timer.timer.just_finished() {
            Finished::run((), data, param.p0());
        }
    }
}
