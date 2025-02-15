use crate::components::{AIComponent, Condition, Soldier, SpriteSize, Squad, Team};
use crate::resources::{GameTextures, SpatialHash, SquadVec};
use crate::resources::WinSize;
use bevy::prelude::*;

use rand::Rng;
pub struct InitialState;
use crate::update_sh_pos::SpatialHashPlugin;


impl Plugin for InitialState {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, sprite_load_system)
            .add_systems(Startup, (spawn_squads, setup_debug_timer))
            //.add_systems(Update, print_all_entities)
            .add_plugins(SpatialHashPlugin);
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
fn spawn_squads(mut commands: Commands, game_textures: Res<GameTextures>, winsize: Res<WinSize>,
     spatial_hash: Res<SpatialHash>,
     mut squads: ResMut<SquadVec>,
     asset_server: Res<AssetServer>
     //query: Query<(&Team, &Squad, &Soldier, &Condition)>, // For debugivuging

    ) {
    // Future parameterization: allow dynamic team/squad setup
    println!("SPAWN_SQUADS");
    commands.spawn(Camera2dBundle::default());
    

    // Calculate the center of the window
    let center_x = winsize.w / 2.0;
    let center_y = winsize.h / 2.0;

    // Spawn the sprite at the center
    commands.spawn(SpriteBundle {
        texture: asset_server.load("../assets/background.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::splat(2.0),
            ..Default::default()
        },
        ..Default::default()
    });
    // Hardcoded example: Each team has a specified number of squads and each element is a team and value is the amount of squads
    let teams_squads = vec![1, 1];
    let squad_dimensions = (2, 2);
    let mut global_squad_id = 0;     
    for (team_id, &squad_count) in teams_squads.iter().enumerate() {
        let texture_handle = choose_sprite((team_id + 1) as u8, &game_textures); 

        for _i in 0..squad_count {
            let mut squad =Vec::new();
            for _i in 0..squad_dimensions.0 {
                for _j in 0..squad_dimensions.1 {
                    let p = define_position(
                        team_id as u8 + 1,
                        winsize.w,
                        winsize.h,
                    );
                    let shcoords = spatial_hash.to_grid_coords(p);
                    let entt =
                    commands
                        .spawn(SpriteBundle {
                            texture: texture_handle.clone(),
                            transform: Transform {
                                translation:p ,
                                scale: Vec3::splat(1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Team((team_id + 1) as u8)) // Store team ID as u8
                        .insert(Squad(global_squad_id))
                        .insert(Soldier {
                            velocity: (1, 0, 0),
                            squad_num: global_squad_id,
                            hit_chance: 50,
                            dodge_chance: 50,
                            sh_coords: shcoords,
                            current_direction:  Vec3::new(1.0, 2.0, 0.0),
                            center_of_mass: Vec3::new(0.0, 0.0, 0.0),
                            target_direction: Vec3::new(0.0, 0.0, 0.0)
                        })
                        .insert(SpriteSize {
                            height: 32,
                            width: 32,
                        })
                        .insert(Condition {
                            morale: 0,
                            stamina: 0,
                            strength: 0,
                            danger_perception: 0,
                            is_active: false,
                            is_moving: false,
                            is_dead: false,
                            is_wounded: false,
                        })
                        .insert(AIComponent::new()).id();

                        squad.push(entt);
                }
                
            }
            println!("squad id: {}", global_squad_id);
            squads.add_squad((squad, global_squad_id));
            global_squad_id += 1;
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
#[allow(dead_code)]
#[derive(Resource)]
struct DebugTimer(Timer);

fn setup_debug_timer(mut commands: Commands) {
    commands.insert_resource(DebugTimer(Timer::from_seconds(0.1, TimerMode::Once)));
}
#[allow(dead_code)]
fn print_all_entities(
    squads: Res<SquadVec>,
    query: Query<(&Team, &Squad, &Soldier, &Condition)>,
) {
    println!("----------------- DEBUG INFO -----------------");
    for (squad_index, squad) in squads.get_squads().iter().enumerate() {
        let (entities, squad_id) = squad;  // Деструктуризация кортежа для получения вектора сущностей и идентификатора отряда
        println!("Squad {} (ID: {}):", squad_index, squad_id);

        for &entt in entities {
            if let Ok((team, squad, soldier, condition)) = query.get(entt) {
                println!(
                    "  Entity ID: {:?} | Team: {} | Squad: {} | Position: {:?} | Health: {} | Stamina: {}",
                    entt,
                    team.0,
                    squad.0,
                    soldier.sh_coords,
                    condition.strength,
                    condition.stamina
                );
            } else {
                println!("  Entity {:?} does not have all required components.", entt);
            }
        }
    }
    println!("----------------------------------------------");
}

