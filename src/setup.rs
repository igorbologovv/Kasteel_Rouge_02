use crate::components::{Condition, Soldier, SpriteSize, Squad, SquadOrder, Team};
use crate::resources::WinSize;
use crate::resources::{GameTextures, Squads};
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
    //TODO in the future this function will get such params as the amount of players etc
    // For now the team belonging is hardcoded
    print!("SPAWN_SQUADS");
    commands.spawn(Camera2dBundle::default());

    // this locig is hardcoded now. Each team has certain amount of squads, so we need to know which squad belongs to each team
    //the first parameter of squad in team_squads is the players ID second is the amount of squads he has.
    let teams_squads = vec![(1, 2), (2, 3)];

    for team in teams_squads {
        //here we choose which sprite wwould be spawned

        for squad_id in 0..team.1 {
            commands
                .spawn(SpriteBundle {
                    texture: choose_sprite(team.0, &game_textures),
                    transform: Transform {
                        translation: define_position(team.0, winsize.w, winsize.h),
                        scale: Vec3::new(1., 1., 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Team(team.0))
                .insert(Squad(squad_id))
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
