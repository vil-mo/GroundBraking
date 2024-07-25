use std::time::Duration;

use animation::Animation;
use bevy::{ecs::system::EntityCommands, prelude::*};

pub mod animation;
pub mod colliders;
pub mod run_on_timer;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((colliders::CollidersPlugin, animation::AnimationPlugin));
    }
}

pub trait CommonEntityCommands {
    fn fade_away(&mut self, lifetime: Duration) -> &mut Self;
}

impl CommonEntityCommands for EntityCommands<'_> {
    #[inline]
    fn fade_away(&mut self, lifetime: Duration) -> &mut Self {
        self.insert(Animation::<
            animation::fade_away::FadeAway,
            animation::destroy::Destroy,
        >::new(Timer::new(lifetime, TimerMode::Once)))
    }
}
