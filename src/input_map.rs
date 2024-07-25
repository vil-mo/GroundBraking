use bevy::prelude::*;

use crate::GameState;

pub struct InputMapPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for InputMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputMap>().add_systems(
            Update,
            (set_movement_direction, set_destroy_tile).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct InputMap {
    movement_direction: Vec2,
    destroy_tile: bool,
}

impl InputMap {
    #[inline]
    pub fn movement_direction(&self) -> Vec2 {
        self.movement_direction
    }

    #[inline]
    pub fn destroy_tile(&mut self) -> bool {
        if self.destroy_tile {
            self.destroy_tile = false;
            true
        } else {
            false
        }
    }
}

fn set_destroy_tile(mut map: ResMut<InputMap>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    map.destroy_tile = map.destroy_tile
        || keyboard_input.just_pressed(KeyCode::KeyK)
        || keyboard_input.just_pressed(KeyCode::KeyX);
}

fn set_movement_direction(mut map: ResMut<InputMap>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    let player_movement = Vec2::new(
        get_movement(Direction::Right, &keyboard_input)
            - get_movement(Direction::Left, &keyboard_input),
        get_movement(Direction::Up, &keyboard_input)
            - get_movement(Direction::Down, &keyboard_input),
    );

    map.movement_direction = player_movement.normalize_or_zero();
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
        match self {
            Direction::Up => {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
            }
            Direction::Down => {
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
            }
            Direction::Left => {
                keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
            }
            Direction::Right => {
                keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            }
        }
    }
}

fn get_movement(control: Direction, input: &Res<ButtonInput<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}
