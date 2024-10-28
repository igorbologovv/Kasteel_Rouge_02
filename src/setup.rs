use crate::components::{Condition, Soldier, SpriteSize, Squad, SquadOrder, Team};
use crate::resources::GameTextures;
use crate::resources::WinSize;
use bevy::prelude::*;

use cgmath::Vector3;
use rand::Rng;
pub struct InitialState;

impl Plugin for InitialState {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sprite_load_system)
            .add_systems(PostStartup, spawn_squads);
    }
}

pub fn sprite_load_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    print!("Loading textures...");
    let game_textures = GameTextures {
        swordman: asset_server.load("../assets/swordsman.png"),
        archer: asset_server.load("../assets/archer.png"),
    };

    commands.insert_resource(game_textures);
}

fn choose_sprite(id: u8, game_textures: &Res<GameTextures>) -> Handle<Image> {
    if id == 1 {
        game_textures.swordman.clone()
    } else {
        game_textures.archer.clone()
    }
}
fn spawn_squads(mut commands: Commands, game_textures: Res<GameTextures>, winsize: Res<WinSize>) {
    // Future parameterization: allow dynamic team/squad setup
    println!("SPAWN_SQUADS");
    commands.spawn(Camera2dBundle::default());

    // Hardcoded example: Each team has a specified number of squads
    let teams_squads = vec![2, 3]; // Number of squads for each team
    let squad_dimensions = (2, 2);

    for (team_id, &squad_count) in teams_squads.iter().enumerate() {
        let texture_handle = choose_sprite((team_id + 1) as u8, &game_textures); // Team IDs start from 1

        for squad_id in 0..squad_count {
            for _i in 0..squad_dimensions.0 {
                for _j in 0..squad_dimensions.1 {
                    commands
                        .spawn(SpriteBundle {
                            texture: texture_handle.clone(),
                            transform: Transform {
                                translation: define_position(
                                    team_id as u8 + 1,
                                    winsize.w,
                                    winsize.h,
                                ),
                                scale: Vec3::splat(1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Team((team_id + 1) as u8)) // Store team ID as u8
                        .insert(Squad(squad_id))
                        .insert(Soldier{ is_active: false, is_moving: false, velocity: (1,1, 0), squad_num: squad_id })
                        .insert(SpriteSize {
                            height: 32,
                            width: 32,
                        })
                        .insert(Condition {
                            morale: 0,
                            stamina: 0,
                            strength: 0,
                            danger_perception: 0,
                        });
                }
            }
        }
    }
}

// Depending on the team number choosing a position range
fn define_position(squad_num: u8, w: f32, h: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    // the id is hardcoded
    if squad_num == 1 {
        Vec3::new(
            rng.gen_range(-w / 2.0..0.0),
            rng.gen_range(-h / 2.0..0.0),
            0.0,
        )
    } else {
        Vec3::new(
            rng.gen_range(0.0..w / 2.0),
            rng.gen_range(0.0..h / 2.0),
            0.0,
        )
    }
}
