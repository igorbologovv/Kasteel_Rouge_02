use crate::components::{Condition, Soldier, SpriteSize, Team};
use crate::resources::{SpatialHash, WinSize};
use bevy::prelude::*;
use cgmath::Vector3;

pub struct SpatialHashPlugin;

impl Plugin for SpatialHashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, initialize_spatial_hash) // Run at startup to populate the spatial hash
            .add_systems(Update, movable_system); // Update soldier positions each frame
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

        push_entity_to_sh(&mut spatial_hash, soldier_entity, initial_position);
    }
}
//This function is checking the position of a soldier after move and if the pos has changed it updates the spatial hash
fn update_entity_in_sh(spatial_hash: &mut SpatialHash, entt: Entity, old_pos: Vec3, new_pos: Vec3) {
    let old_coords = spatial_hash.to_grid_coords(old_pos);
    let new_coords = spatial_hash.to_grid_coords(new_pos);

    // if entity changed pos
    if old_coords != new_coords {
        let old_index = spatial_hash
            .pos_to_index(old_coords)
            .expect("Position out of bounds");
        let old_cellref = spatial_hash
            .get_mut(old_index)
            .expect("Index out of bounds");
        old_cellref.retain(|e| *e != entt);
    }

    // Add if entt is not yet there
    let new_index = spatial_hash
        .pos_to_index(new_coords)
        .expect("Position out of bounds");
    let new_cellref = spatial_hash
        .get_mut(new_index)
        .expect("Index out of bounds");

    if !new_cellref.contains(&entt) {
        new_cellref.push(entt);
    }
}

fn push_entity_to_sh(spatial_hash: &mut SpatialHash, entt: Entity, pos: Vec3) {
    let t = spatial_hash.to_grid_coords(pos);
    // Based on position in SpatialHash, get the index
    let index = spatial_hash
        .pos_to_index(t)
        .expect("Position out of bounds");
    let cellref = spatial_hash.get_mut(index).expect("Index out of bounds");
    cellref.push(entt); // Direct addition, without checking
    println!(
        "Entity with index {} and coordinates {} added \n",
        index, pos
    );
}

pub fn movable_system(
    win_size: Res<WinSize>,
    mut spatial_hash: ResMut<crate::SpatialHash>,
    mut soldier: Query<(Entity, &mut Transform, &SpriteSize, &mut Soldier, &Team)>,
) {
    //print!("...........Movable system call..........");
    let min = Vector3 { x: 0, y: 0, z: 0 };
    let max = Vector3 {
        x: 800,
        y: 600,
        z: 1,
    };
    // Borrow spatial_hash as mutable only in this block to avoid conflicts
    {
        let spatial_hash = &mut *spatial_hash;
        for (soldier_entity, mut transform, _sprite_size, mut _soldier_component, team) in
            soldier.iter_mut()
        {
            let old_position = transform.translation;

            //TODO MOVE LOGIC HERE IS A MOVE LOGIC AND ASSIGN NEW VALUES TO POS
            // soldier_comppnent.pos.x+=1;
            //pos.y +=1;
            //etc.....
            let new_position = old_position + Vec3::new(0.0, 0.0, 0.0);
            //let new_position = old_position + Vec3::new(0.0, 0.0, 0.0);
            transform.translation = new_position;
            update_entity_in_sh(
                spatial_hash,
                soldier_entity,
                old_position,
                transform.translation,
            );
        }
    }

    // Now it's safe to mutably borrow spatial_hash again
    // for (_pos, _idx, cell_content) in spatial_hash.iter_cubes_mut(min, max) {
    //     if cell_content.len() > 0 {
    //         println!("Содержимое ячейки {:?}: {:?}", _idx, cell_content);
    //     }
    // }
}
