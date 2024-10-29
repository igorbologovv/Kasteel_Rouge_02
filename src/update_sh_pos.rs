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
    mut soldier_query: Query<(Entity, &mut Transform, &SpriteSize, &mut Soldier, &Team)>,
) {
    let mut update_buffer: Vec<(Entity, Vec3, Vec3)> = Vec::new();

    // Moving soldier and adding to update buffer if needed
    for (soldier_entity, mut transform, _sprite_size, mut soldier, _team) in soldier_query.iter_mut() {
        let old_position = transform.translation;

        // HERE SHOULD BE a moving logic
        let new_position = old_position + Vec3::new(0.0, 0.0, 0.0); // Пример перемещения вправо
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

    // Применяем все изменения к spatial hash
    update_entity_in_sh(&mut spatial_hash, update_buffer);
}