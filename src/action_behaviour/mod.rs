use crate::GameState;
use bevy::{
    ecs::schedule::{ScheduleLabel, SystemConfigs},
    prelude::*,
};

pub mod actions;
pub mod behaviours;

pub struct ActionBehaviourPlugin;
impl Plugin for ActionBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            action_behaviour_schedule(),
            (
                ActionSet.after(BehaviourSet),
                (ActionSet, BehaviourSet).run_if(in_state(GameState::Playing)),
            ),
        );

        app.add_plugins((actions::RegisterActions, behaviours::RegisterBehaviours));
    }
}

pub trait Behaviour: Component {
    fn systems() -> SystemConfigs;
}

pub trait Action: Component {
    fn systems() -> SystemConfigs;
}

#[inline(always)]
fn action_behaviour_schedule() -> impl ScheduleLabel {
    FixedUpdate
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BehaviourSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionSet;

pub trait ActionBehaviourApp {
    fn register_behaviour<B: Behaviour>(&mut self) -> &mut Self;
    fn register_action<A: Action>(&mut self) -> &mut Self;
}

impl ActionBehaviourApp for App {
    #[inline]
    fn register_behaviour<B: Behaviour>(&mut self) -> &mut Self {
        self.add_systems(
            action_behaviour_schedule(),
            B::systems().in_set(BehaviourSet),
        )
    }

    #[inline]
    fn register_action<A: Action>(&mut self) -> &mut Self {
        self.add_systems(action_behaviour_schedule(), A::systems().in_set(ActionSet))
    }
}
