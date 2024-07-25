use bevy::prelude::*;

use crate::dynamic_initialization::EntitySystem;

pub struct ShowUp;

impl EntitySystem for ShowUp {
    type Data = (
        Option<&'static mut Sprite>,
        Option<&'static Handle<ColorMaterial>>,
    );
    type Filter = ();
    type Param = ResMut<'static, Assets<ColorMaterial>>;

    type In = f32;
    type Out = ();

    fn run(
        input: Self::In,
        data: crate::dynamic_initialization::DataItem<'_, Self>,
        param: crate::dynamic_initialization::ParamItem<'_, '_, Self>,
    ) -> Self::Out {
        let (sprite, material_handle) = data;
        let mut materials = param;

        let new_alpha = input;

        if let Some(mut sprite) = sprite {
            sprite.color.set_alpha(new_alpha);
        }

        if let Some(material_handle) = material_handle {
            if let Some(material) = materials.get_mut(material_handle) {
                material.color.set_alpha(new_alpha);
            }
        }
    }
}
