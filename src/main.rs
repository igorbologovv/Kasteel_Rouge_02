mod components;
pub mod move_logic;
mod resources;
mod setup;
pub mod winit;
use crate::resources::WinSize;
use crate::setup::InitialState;
use crate::winit::winit;
use bevy::prelude::*;
use resources::SpatialHash;
use std::time::Duration;

const WIN_X: f32 = 1200.0;
const WIN_Y: f32 = 600.0;

#[derive(Resource)]
struct GameConstants {
    physics_time_step: Duration,
    squad_size: usize,
    round_duration: Duration,
    map_size: f32,
    spatial_hash_cell_size: f32,
}

impl Default for GameConstants {
    fn default() -> Self {
        GameConstants {
            physics_time_step: Duration::from_secs_f32(1. / 60.),
            squad_size: 10,
            round_duration: Duration::from_secs(10),
            map_size: 4000.0,
            spatial_hash_cell_size: 16.0,
        }
    }
}

fn main() {
    let game_consts = GameConstants::default();
    let sh_cells = (game_consts.map_size / game_consts.spatial_hash_cell_size).ceil() as usize;
    let spatial_hash = SpatialHash::new(sh_cells, sh_cells, game_consts.spatial_hash_cell_size);
    App::new()
        .insert_resource(WinSize { w: WIN_X, h: WIN_Y })
        .insert_resource(spatial_hash)
        .add_plugins((InitialState, DefaultPlugins.set(winit(WIN_X, WIN_Y))))
        .run();
}
