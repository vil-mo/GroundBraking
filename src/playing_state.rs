use std::time::Duration;

use bevy::prelude::*;
use bevy_consumable_event::{ConsumableEventApp, ConsumableEventReader, ConsumableEventWriter};

use crate::{
    actors::{dasher::Dasher, player::Player, SpawnActor},
    common::animation::{disable::Disable, fade_away::FadeAway, show_up::ShowUp, Animation},
    GameState,
};

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (setup_layout, setup_tiles))
            .add_systems(
                Update,
                (remove_tiles, tick_and_restore_tiles).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_layout)
            .init_resource::<LandTiles>()
            .add_consumable_event::<RemoveTile>();
    }
}

fn setup_layout(
    mut player_spawn: ConsumableEventWriter<SpawnActor<Player>>,
    mut dasher_spawn: ConsumableEventWriter<SpawnActor<Dasher>>,
    mut tiles: ResMut<LandTiles>,
) {
    tiles.reset();

    player_spawn.send(SpawnActor(Player {
        position: Vec2::new(0.0, 0.0),
    }));

    dasher_spawn.send(SpawnActor(Dasher {
        position: Vec2::new(30.0, 30.0),
    }))
}

/// Tile size in pixels
const TILE_SIZE: f32 = 40.;
const TILES_WIDTH: usize = 4;
const TILES_HEIGHT: usize = 4;

/// Tiles between -80 and 80
#[derive(Resource, Default)]
pub struct LandTiles {
    tiles: [[LandTile; TILES_WIDTH]; TILES_HEIGHT],
}

#[derive(Default)]
pub enum LandTile {
    #[default]
    Alive,
    Destroyed {
        until_alive: Timer,
    },
}

impl LandTiles {
    fn world_to_array(pos: Vec2) -> Option<IVec2> {
        let scaled_array_position =
            pos + Vec2::new(TILES_WIDTH as f32, TILES_HEIGHT as f32) * TILE_SIZE / 2.0;

        let array_pos = (scaled_array_position / TILE_SIZE).floor().as_ivec2();

        if array_pos.x < 0
            || array_pos.y < 0
            || array_pos.x >= TILES_WIDTH as i32
            || array_pos.y >= TILES_HEIGHT as i32
        {
            None
        } else {
            Some(array_pos)
        }
    }

    fn array_to_world(pos: IVec2) -> Vec2 {
        Vec2::new(
            pos.x as f32 - TILES_WIDTH as f32 / 2.0,
            pos.y as f32 - TILES_HEIGHT as f32 / 2.0,
        ) * TILE_SIZE
            + Vec2::splat(TILE_SIZE / 2.0)
    }
}

impl LandTiles {
    pub fn reset(&mut self) {
        *self = default();
    }

    /// Returns true if postition is on alive tile, false othervise
    pub fn on_ground(&self, pos: Vec2) -> bool {
        let array_position = LandTiles::world_to_array(pos);

        let Some(array_position) = array_position else {
            return false;
        };

        match self.tiles[array_position.x as usize][array_position.y as usize] {
            LandTile::Alive => true,
            LandTile::Destroyed { .. } => false,
        }
    }
}

#[derive(Component)]
pub struct TileSprite {
    x: usize,
    y: usize,
}

const FALL_ANIMATION_DURATION: Duration = Duration::from_millis(300);
const RESTORE_ANIMATION_DURATION: Duration = Duration::from_millis(150);

pub type FadeAwayAnimation = Animation<FadeAway, Disable<FadeAway>>;
pub type ShowUpAnimation = Animation<ShowUp, Disable<ShowUp>>;

fn setup_tiles(mut commands: Commands, loader: Res<AssetServer>) {
    for x in 0..TILES_WIDTH {
        for y in 0..TILES_HEIGHT {
            let fade_away_timer = Timer::new(FALL_ANIMATION_DURATION, TimerMode::Once);
            let show_up_timer = Timer::new(RESTORE_ANIMATION_DURATION, TimerMode::Once);

            commands.spawn((
                SpriteBundle {
                    texture: loader.load("textures/ground_tile.png"),
                    transform: Transform::from_translation(
                        LandTiles::array_to_world(IVec2::new(x as i32, y as i32)).extend(-10.0),
                    ),
                    ..default()
                },
                TileSprite { x, y },
                FadeAwayAnimation::new_disabled(fade_away_timer),
                ShowUpAnimation::new_disabled(show_up_timer),
            ));
        }
    }
}

#[derive(Event)]
pub struct RemoveTile(pub Vec2, pub Duration);

fn remove_tiles(
    mut tiles: ResMut<LandTiles>,
    mut remove_event: ConsumableEventReader<RemoveTile>,
    mut tile_sprite_query: Query<(&TileSprite, &mut FadeAwayAnimation, &mut ShowUpAnimation)>,
) {
    for RemoveTile(pos, duration) in remove_event.read_and_consume_all() {
        let Some(array_pos) = LandTiles::world_to_array(pos) else {
            return;
        };

        let tile: &mut LandTile = &mut tiles.tiles[array_pos.x as usize][array_pos.y as usize];

        match tile {
            LandTile::Alive => {
                *tile = LandTile::Destroyed {
                    until_alive: Timer::new(duration, TimerMode::Once),
                };

                let animation = tile_sprite_query.iter_mut().find(|(tile_sprite, _, _)| {
                    tile_sprite.x as i32 == array_pos.x && tile_sprite.y as i32 == array_pos.y
                });

                if let Some((_, mut fade_animation, mut show_animation)) = animation {
                    show_animation.disable();
                    fade_animation.enable();
                }
            }
            LandTile::Destroyed { until_alive } => {
                until_alive.set_duration(duration);
                until_alive.reset();
            }
        }
    }
}

fn tick_and_restore_tiles(
    time: Res<Time>,
    mut tiles: ResMut<LandTiles>,
    mut tile_sprite_query: Query<(&TileSprite, &mut FadeAwayAnimation, &mut ShowUpAnimation)>,
) {
    for x in 0..TILES_WIDTH {
        for y in 0..TILES_HEIGHT {
            let tile: &mut LandTile = &mut tiles.tiles[x][y];

            match tile {
                LandTile::Alive => (),
                LandTile::Destroyed { until_alive } => {
                    until_alive.tick(time.delta());
                    if until_alive.finished() {
                        *tile = LandTile::Alive;

                        let animation = tile_sprite_query
                            .iter_mut()
                            .find(|(tile_sprite, _, _)| tile_sprite.x == x && tile_sprite.y == y);

                        if let Some((_, mut fade_animation, mut show_animation)) = animation {
                            fade_animation.disable();
                            show_animation.enable();
                        }
                    }
                }
            }
        }
    }
}

fn cleanup_layout() {}
