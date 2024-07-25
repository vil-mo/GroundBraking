use crate::dynamic_initialization::EntitySystem;
use std::marker::PhantomData;

use super::Animation;

pub struct Disable<Tick: EntitySystem<In = f32, Out = ()>, Marker: Send + Sync + 'static = ()>(
    pub PhantomData<(Tick, Marker)>,
);

impl<Tick: EntitySystem<In = f32, Out = ()>, Marker: Send + Sync + 'static> Default
    for Disable<Tick, Marker>
{
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Tick: EntitySystem<In = f32, Out = ()>, Marker: Send + Sync + 'static> EntitySystem
    for Disable<Tick, Marker>
{
    type Data = &'static mut Animation<Tick, Disable<Tick, Marker>, Marker>;
    type Filter = ();
    type Param = ();

    type In = ();
    type Out = ();

    fn run(
        _: Self::In,
        data: crate::dynamic_initialization::DataItem<'_, Self>,
        param: crate::dynamic_initialization::ParamItem<'_, '_, Self>,
    ) -> Self::Out {
        let mut animation = data;
        let () = param;

        animation.disable();
    }
}
