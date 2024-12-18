use crate::components::{ Soldier, SpriteSize, Team, AIComponent};
use crate::resources::{SpatialHash, SquadVec, TargetSquads};
use crate::moving_system_ai::update_directions_system;

use bevy::prelude::*;

use rand::Rng;


pub struct SpatialHashPlugin;


impl Plugin for SpatialHashPlugin {
    fn build(&self, app: &mut App) {

        app.insert_resource(TargetSquads {
            cmass_id: Vec::new(),
        })         .insert_resource(DirectionUpdateTimer {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
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
#[derive(Resource)]
pub struct DirectionUpdateTimer {
    pub timer: Timer,
}
pub fn movable_system(
    mut spatial_hash: ResMut<crate::SpatialHash>,
    mut soldier_query: Query<(Entity, &mut Transform, &SpriteSize, &mut Soldier, &Team, &AIComponent)>,
    cmass_id: Res<TargetSquads>,
    time: Res<Time>,
    team_query: Query<(Entity, &Team)>,
    mut timer: ResMut<DirectionUpdateTimer>,
) {
    let mut rng = rand::thread_rng();
    let mut update_buffer: Vec<(Entity, Vec3, Vec3)> = Vec::new();

    // Обновляем таймер
    timer.timer.tick(time.delta());

    // Проверяем, нужно ли обновить направление
    let should_update_direction = timer.timer.finished();

    for (soldier_entity, mut transform, _sprite_size, mut soldier, team, ai) in soldier_query.iter_mut() {
        let old_position = transform.translation;

        // Обновляем направление, если таймер завершён
        if should_update_direction {
            let random_direction = Vec3::new(
                rng.gen_range(-1.0..=1.0),
                rng.gen_range(-1.0..=1.0),
                0.0,
            )
            .normalize();

            // Определение цели на основе центров масс вражеских отрядов
            let mut target_direction = Vec3::ZERO;
            let mut min_distance = f32::MAX;

            for (center_of_mass, squad_id) in &cmass_id.cmass_id {
                if *squad_id == soldier.squad_num as u16 {
                    continue;
                }

                let direction_to_enemy = *center_of_mass - old_position;
                let distance: f32 = direction_to_enemy.length();

                // Ищем ближайший вражеский отряд
                if distance < min_distance {
                    min_distance = distance;
                    target_direction = direction_to_enemy.normalize();
                }
            }

            soldier.current_direction = (random_direction * 0.2) + (target_direction * 0.8);
        }

        // Вычисление нового положения с учетом направления и скорости
        let move_direction = soldier.current_direction;
        let new_position = old_position + move_direction * soldier.velocity.0 as f32;

        // Проверяем, занята ли клетка
        let entities_in_cell = spatial_hash.get_entities_in_cell(new_position);

        let mut should_stop = false;
        if !entities_in_cell.is_empty() {
            for entity in entities_in_cell {
                // Пропускаем самого себя
                if entity == soldier_entity {
                    continue;
                }

                // Проверяем команду
                if let Ok((_entity, other_team)) = team_query.get(entity) {
                    if team.0 == other_team.0 {
                        println!("Союзник найден: {:?}", entity);
                        soldier.current_direction = Vec3::new(
                            rng.gen_range(-1.0..=1.0),
                            rng.gen_range(-1.0..=1.0),
                            0.0,
                        )
                        .normalize();
                        
                    } else {
                        println!("Враг найден: {:?}", entity);
                        // Изменяем направление при обнаружении врага
                        should_stop = true; // Останавливаемся, если нашли врага
                    }
                }
            }
        }

        // Если должны остановиться, пропускаем обновление позиции
        if should_stop {
            continue;
        }

        // Обновляем позицию
        transform.translation = new_position;

        // Transform positions to shcoords
        let old_coords = spatial_hash.to_grid_coords(old_position);
        let new_coords = spatial_hash.to_grid_coords(new_position);

        // Check if position was changed
        if old_coords != new_coords {
            // Обновляем координаты в структуре Soldier
            soldier.sh_coords = new_coords;
            // Добавляем запись в буфер для обновления spatial hash
            update_buffer.push((soldier_entity, old_position, new_position));
        }
    }

    // Применяем изменения к spatial hash
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