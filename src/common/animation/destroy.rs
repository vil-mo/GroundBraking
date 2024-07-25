use bevy::prelude::*;

use crate::dynamic_initialization::EntitySystem;

pub struct Destroy;

impl EntitySystem for Destroy {
    type Data = Entity;
    type Filter = ();
    type Param = Commands<'static, 'static>;

    type In = ();
    type Out = ();

    fn run(
        _: Self::In,
        data: crate::dynamic_initialization::DataItem<'_, Self>,
        param: crate::dynamic_initialization::ParamItem<'_, '_, Self>,
    ) -> Self::Out {
        let entity = data;
        let mut commands = param;

        commands.entity(entity).despawn_recursive();
    }
}
