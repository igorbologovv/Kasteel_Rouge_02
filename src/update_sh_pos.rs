use crate::components::{ Soldier, SpriteSize, Team, AIComponent};
use crate::resources::{SpatialHash, SquadVec, TargetSquads};
use crate::moving_system_ai::update_directions_system ;

use bevy::prelude::*;

use rand::Rng;
#[derive(bevy::ecs::schedule::ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]

pub struct SpatialHashPlugin;


impl Plugin for SpatialHashPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetSquads {
            cmass_id: Vec::new(),
        }) 
            .add_systems(PostStartup, initialize_spatial_hash) // Run at startup to populate the spatial hash
            .add_systems(Update, update_center_of_mass.before(movable_system))
            .add_systems(Update, update_directions_system.after(update_center_of_mass))
            .add_systems(Update,movable_system); // Затем перемещаем солдат
    }
}

// TODO Use spatial hash to understand relative position of the soldier.
//TODO
// This should be the very first call to fill the spatial hash with initial positions of soldiers
pub fn initialize_spatial_hash(
    mut spatial_hash: ResMut<SpatialHash>,
    soldier_query: Query<(Entity, &Soldier, &Transform)>,
) {
    for (soldier_entity, _soldier_component, pos) in soldier_query.iter() {
        let initial_position = pos.translation;

        spatial_hash.add_entity(  initial_position, soldier_entity);
    }
}
//This function shoud TAKE a vector of buffers which contains soldiers to be updated
//Soldier has a field with an old shcoords already
fn update_entity_in_sh(spatial_hash: &mut SpatialHash, update_buffer: Vec<(Entity, Vec3, Vec3)>) {
    for (entt, old_pos, new_pos) in update_buffer {
        
        spatial_hash.remove_entity(old_pos, entt);
        spatial_hash.add_entity(new_pos, entt);
    }
}

/*This function is responsible for updating soldier position in the world and
If position was chaned in sh then add it to update buffer
*/
pub fn movable_system(
    mut spatial_hash: ResMut<crate::SpatialHash>,
    mut soldier_query: Query<(Entity, &mut Transform, &SpriteSize, &mut Soldier, &Team, &AIComponent)>,
    cmass_id: Res<TargetSquads>
) {
    
    let mut rng = rand::thread_rng();
    let mut update_buffer: Vec<(Entity, Vec3, Vec3)> = Vec::new();
    // for (center_of_mass, squad_id) in &cmass_id.cmass_id {
    //    // println!("Squad ID: {}, Center of Mass: ({}, {}, {})", squad_id, center_of_mass.x, center_of_mass.y, center_of_mass.z);
    // }
    // Moving soldier and adding to update buffer if needed
    for (soldier_entity, mut transform, _sprite_size, mut soldier, _team, ai) in soldier_query.iter_mut() {
        let old_position = transform.translation;
        println!("Friends for Soldier{}: {}{}{}{}{}{}{}{}", soldier_entity,ai.allies_directions[0],ai.allies_directions[1],ai.allies_directions[2],ai.allies_directions[3],ai.allies_directions[4],ai.allies_directions[5],ai.allies_directions[6],ai.allies_directions[7], );
        // Generate random direction
        let random_direction = Vec3::new(
            rng.gen_range(-1.0..=1.0),
            rng.gen_range(-1.0..=1.0),
            0.0
        ).normalize();

        // Определение цели на основе центров масс вражеских отрядов
        let mut target_direction = Vec3::ZERO;
        let mut min_distance = f32::MAX;

        for (center_of_mass, squad_id) in &cmass_id.cmass_id {
            //println!("My squad: {}, CenterMass Squad: {}", soldier.squad_num, squad_id);
            if *squad_id == soldier.squad_num as u16 {
               
                continue;
            }
            
            let direction_to_enemy = *center_of_mass - old_position;
            let distance: f32 = direction_to_enemy.length();

            // Ищем ближайший вражеский отряд
            if distance < min_distance {
                //println!("I found THESE BASTARDS");
                min_distance = distance;
                target_direction = direction_to_enemy.normalize();
            }
        }

        // Итоговое направление с учетом случайного направления и направления к врагу
        let move_direction = (random_direction * 0.2) + (target_direction * 0.8);

        // Вычисление нового положения с учетом направления и скорости
        let new_position = old_position + move_direction * soldier.velocity.0 as f32;
        transform.translation = new_position;

        // Transform positions to shcoords
        let old_coords = spatial_hash.to_grid_coords(old_position);
        let new_coords = spatial_hash.to_grid_coords(new_position);

        // Check if position was changed
        if old_coords != new_coords {
            //Update sh coords in struct Soldier
            soldier.sh_coords = new_coords;
            // Добавляем запись в буфер для обновления spatial hash
            update_buffer.push((soldier_entity, old_position, new_position));
        }
    }

    // apply changes to sh
    update_entity_in_sh(&mut spatial_hash, update_buffer);
}
fn update_center_of_mass(
    squads: Res<SquadVec>, 
    mut targets: ResMut<TargetSquads>, 
    mut soldiers_query: Query<(&mut Soldier, &mut Transform)>
) {
    targets.cmass_id.clear(); 
 
    for (squad, id) in squads.get_squads().iter() {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        if !squad.is_empty() {
            for &entity in squad.iter() {
                if let Ok((_s, t)) = soldiers_query.get_mut(entity) {
                    sum_x += t.translation.x;
                    sum_y += t.translation.y;
                }
            }
            
            let count = squad.len() as f32;
            if count > 0.0 {
                let center_of_mass = Vec3::new(sum_x / count, sum_y / count, 0.0);
               // println!("Adding Center of mASSS");
               //println!("Adding center mass with id: {}\n", *id);
                targets.cmass_id.push((center_of_mass, *id));
                // Обновляем центр масс для всех солдат в отряде
                for &entity in squad.iter() {
                    if let Ok((mut soldier, _tr)) = soldiers_query.get_mut(entity) {
                        soldier.center_of_mass = center_of_mass;
                    }
                }
            }
        }
    }
}
//time update