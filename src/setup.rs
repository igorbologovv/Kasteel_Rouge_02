use crate::components::{Condition, Soldier, Team, SpriteSize};
use crate::resources::GameTextures;
use crate::resources::WinSize;
use bevy::prelude::*;

use cgmath::Vector3;
use rand::Rng;
pub struct InitialState;

impl Plugin for InitialState {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sprite_load_system)
            .add_systems(PostStartup, spawn_units);
    }
}

pub fn sprite_load_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_textures = GameTextures {
        swordman: asset_server.load("../assets/swordsman.png"),
        archer: asset_server.load("../assets/archer.png"),
    };

    commands.insert_resource(game_textures);
}

fn spawn_units(mut commands: Commands, game_textures: Res<GameTextures>, win_size: Res<WinSize>) {
    //commands.spawn(Camera2dBundle::default());

    let squad_size = 15;
    let mut rng = rand::thread_rng();

    commands.spawn(Camera2dBundle::default());

    // Спавним первую команду (нижний левый угол)
    for _ in 0..squad_size {
        let (x, y) = (
            rng.gen_range(-win_size.w / 2.0..0.0),
            rng.gen_range(-win_size.h / 2.0..0.0),
        );
        commands
            .spawn(SpriteBundle {
                texture: game_textures.swordman.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(1., 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Soldier::new(Vec3::new(x, y, 0.), 1, 1, 0)) // Здесь создаем солдата с координатами
            .insert(Team(0))
            .insert(SpriteSize { height: 32, width:32 })
            .insert(Condition {
                morale: 0,
                stamina: 0,
                strength: 0,
                danger_perception: 0,
            }); // Команда 0 для первой группы
    }

    // Спавним вторую команду (верхний правый угол)
    for _ in 0..squad_size {
        let (x, y) = (
            rng.gen_range(0.0..win_size.w / 2.0),
            rng.gen_range(0.0..win_size.h / 2.0),
        );
        commands
            .spawn(SpriteBundle {
                texture: game_textures.archer.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(1.0, 1., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Soldier::new(Vec3::new(x, y, 0.), 1, 1, 0)) // Здесь создаем солдата с координатами
            .insert(Team(1))
            .insert(SpriteSize { height: 32, width:32 })
            .insert(Condition {
                morale: 0,
                stamina: 0,
                strength: 0,
                danger_perception: 0,
            }); // Команда 1 для второй группы
    }
}
