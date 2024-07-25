use bevy::{
    ecs::system::{CombinatorSystem, Combine},
    prelude::*,
};

// At least I tried
// It's a waste of time
//
// pub trait Reborrow<'a> {
//     fn reborrow(&'a mut self) -> Self;
// }

// No clone???? Why??????
// impl <'w, T: Resource>  Reborrow<'w> for Ref<'w, T> {
//     fn reborrow(&'w mut self) -> Self {
//         Ref::clone(self)
//     }
// }

// impl<'w, T: ?Sized> Reborrow<'w> for Mut<'w, T> {
//     fn reborrow(&'w mut self) -> Self {
//         Mut::reborrow(self)
//     }
// }

// And this has clone oh yeah ))) Love consistency
// impl <'w, T: Resource>  Reborrow<'w> for Res<'w, T> {
//     fn reborrow(&'w mut self) -> Self {
//         Res::clone(self)
//     }
// }

// Returns Mut
//
// Why no version of ResMut reborrow??????
// impl <'w, T: Resource>  Reborrow<'w> for ResMut<'w, T> {
//     fn reborrow(&'w mut self) -> Self {
//         ResMut::reborrow(self)
//     }
// }

pub struct RunIfTrue;

impl<BOut: Default, A: System<Out = bool>, B: System<In = (), Out = BOut>> Combine<A, B>
    for RunIfTrue
{
    type In = A::In;
    type Out = BOut;

    fn combine(
        input: Self::In,
        a: impl FnOnce(<A as System>::In) -> <A as System>::Out,
        b: impl FnOnce(<B as System>::In) -> <B as System>::Out,
    ) -> Self::Out {
        if a(input) {
            b(())
        } else {
            default()
        }
    }
}

pub fn run_if_true<
    AIn,
    AMarker,
    BOut: Default,
    BMarker,
    A: IntoSystem<AIn, bool, AMarker>,
    B: IntoSystem<(), BOut, BMarker>,
>(
    a: A,
    b: B,
) -> CombinatorSystem<
    RunIfTrue,
    <A as IntoSystem<AIn, bool, AMarker>>::System,
    <B as IntoSystem<(), BOut, BMarker>>::System,
> {
    let a = A::into_system(a);
    let b = B::into_system(b);

    let a_name = a.name();
    let b_name = b.name();

    CombinatorSystem::new(
        a,
        b,
        format!("Run {} if {} returns true", b_name, a_name).into(),
    )
}
