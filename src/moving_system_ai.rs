use crate::components::{Team, AIComponent};
use crate::resources::SpatialHash;
use bevy::prelude::*;
use cgmath::Vector3;
use std::collections:: HashSet;
/*
In this file we are collecting data for AIcomponent of each soldier
Particularly the direction to friend or enemy,
 */


pub fn define_direction_to_enemy_cmass(dir: Vec3, my_pos: Vec3) -> [u8; 8] {
    // Массив направлений: N, NE, E, SE, S, SW, W, NW
    let mut result_dir = [0u8; 8];
    let dx = dir.x - my_pos.x;
    let dy = dir.y - my_pos.y;

    if dx == 0.0 && dy == 0.0 {
        // Солдат и враг находятся в одной точке
        return result_dir; // Все нули
    }

    // Проверка на нулевой dx, чтобы избежать деления на ноль
    if dx == 0.0 {
        if dy > 0.0 {
            result_dir[0] = 1; // Север
        } else if dy < 0.0 {
            result_dir[4] = 1; // Юг
        }
        // Если dy == 0.0, то враг находится прямо на солдате
        return result_dir;
    }

    let slope = dy / dx;

    if dx > 0.0 {
        if dy >= 0.0 {
            // Первый квадрант (NE)
            if slope >= 2.4142 {
                // tan(67.5°)
                result_dir[0] = 1; // Север
            } else if slope >= 0.4142 {
                // tan(22.5°)
                result_dir[1] = 1; // NE
            } else {
                result_dir[2] = 1; // Восток
            }
        } else {
            // Четвертый квадрант (SE)
            if slope <= -2.4142 {
                // tan(-67.5°)
                result_dir[4] = 1; // Юг
            } else if slope <= -0.4142 {
                // tan(-22.5°)
                result_dir[3] = 1; // SE
            } else {
                result_dir[2] = 1; // Восток
            }
        }
    } else {
        if dy >= 0.0 {
            // Второй квадрант (NW)
            if slope <= -2.4142 {
                // tan(-67.5°)
                result_dir[0] = 1; // Север
            } else if slope <= -0.4142 {
                // tan(-22.5°)
                result_dir[7] = 1; // NW
            } else {
                result_dir[6] = 1; // Запад
            }
        } else {
            // Третий квадрант (SW)
            if slope >= 2.4142 {
                // tan(67.5°)
                result_dir[4] = 1; // Юг
            } else if slope >= 0.4142 {
                // tan(22.5°)
                result_dir[5] = 1; // SW
            } else {
                result_dir[6] = 1; // Запад
            }
        }
    }

    result_dir
}



pub fn update_directions_system(
    mut spatial_hash: ResMut<SpatialHash>,
    mut query: Query<(Entity, &Team, &Transform, &mut AIComponent)>,
) {
    // Copy of query Entities to avoid borrow checker shit
    let entities: Vec<(Entity, Team, Transform)> = query
        .iter()
        .map(|(e, t, tr, _)| (e, *t, tr.clone()))
        .collect();

    // This is the future option if we want to update my position for other entities
    // but now it works with bugs I am lazy to fix it
    //let mut other_updates: HashMap<Entity, (HashSet<usize>, HashSet<usize>)> = HashMap::new();

    for (my_entity, my_team, my_transform, mut my_ai_component) in query.iter_mut() {
        // Clear directions 
        my_ai_component.allies_directions = [0u8; 8];
        my_ai_component.enemies_directions = [0u8; 8];

        // Создаем HashSet для отслеживания обработанных сущностей
        let mut processed_entities: HashSet<Entity> = HashSet::new();

        // Определяем область поиска вокруг солдата
        let search_radius = spatial_hash.cell_size * 2.5;

        let min = my_transform.translation - Vec3::new(search_radius, search_radius, 0.0);
        let max = my_transform.translation + Vec3::new(search_radius, search_radius, 0.0);

        let min_coords = spatial_hash.to_grid_coords(min);
        let max_coords = spatial_hash.to_grid_coords(max);

        for x in min_coords.x..=max_coords.x {
            for y in min_coords.y..=max_coords.y {
                let cell_coords = Vector3::new(x, y, 0);

                // Получаем индекс ячейки
                if let Some(index) = spatial_hash.sh.pos_to_index(cell_coords) {
                  
                    //While iterating cells around my_entity we check each cell, since cell can contain several entities 
                    //we have to iterate through SmallVec inside the cell.
                    if let Some(cell_entities) = spatial_hash.sh.get(index) {
                        for &other_entity in cell_entities {
                            if other_entity == my_entity {
                                continue; // We found ourself here skip processing
                            }

                            // HashSet checks if it contains the entity if yes then we skip processing
                            if !processed_entities.insert(other_entity) {
                                continue; 
                            }

                            // If other entity in cell found in entities(vector of our query copy)
                            if let Some((_, other_team, other_transform)) =
                                entities.iter().find(|(e, _, _)| *e == other_entity)
                            {
                                //define the direction (SNWE of between)
                                let direction_to_other = define_direction_to_entity(
                                    other_transform.translation,
                                    my_transform.translation,
                                );
                                // If not the same position as my entity
                                if direction_to_other != 255 {
                                    let direction_to_other_usize = direction_to_other as usize;
                                    //Get direction from other entity to me
                                    //let direction_to_me = define_direction_to_entity(
                                    //     my_transform.translation,
                                    //     other_transform.translation,
                                    // );
                                    //let direction_to_me_usize = direction_to_me as usize;//cast to usize

                                    // Check if it is a friend
                                    if other_team.0 == my_team.0 {
                                        
                                        my_ai_component.allies_directions[direction_to_other_usize] += 1;

                                        // Update enemy pos for me
                                        // other_updates
                                        //     .entry(other_entity)
                                        //     .or_insert_with(|| (HashSet::new(), HashSet::new()))
                                        //     .0 // HashSet союзников
                                        //     .insert(direction_to_me_usize);
                                    } else {
                                        // Enemy found
                                        my_ai_component.enemies_directions[direction_to_other_usize] += 1;

                                        // Update my pos for enemy
                                        // other_updates
                                        //     .entry(other_entity)
                                        //     .or_insert_with(|| (HashSet::new(), HashSet::new()))
                                        //     .1 // HashSet врагов
                                        //     .insert(direction_to_me_usize);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }


    }

    // // Применяем сохраненные обновления к AIComponent других сущностей
    // for (entity, (ally_dirs, enemy_dirs)) in other_updates {
    //     if let Ok((_, _, _, mut ai_component)) = query.get_mut(entity) {
    //         // Обновляем направления союзников
    //         for dir in ally_dirs {
    //             ai_component.allies_directions[dir] += 1;
    //         }
    //         // Обновляем направления врагов
    //         for dir in enemy_dirs {
    //             ai_component.enemies_directions[dir] += 1;
    //         }
    //     }
    // }
}

fn define_direction_to_entity(target_pos: Vec3, my_pos: Vec3) -> u8 {
    let dx: f32 = target_pos.x - my_pos.x;
    let dy = target_pos.y - my_pos.y;

    if dx == 0.0 && dy == 0.0 {
        return 255; // Специальное значение для совпадения позиций
    }

    let abs_dx = dx.abs();
    let abs_dy = dy.abs();

    if dx >= 0.0 {
        if dy >= 0.0 {
            // Первый квадрант (NE и E и N)
            if abs_dx > abs_dy {
                return 2; // Восток
            } else if abs_dy > abs_dx {
                return 0; // Север
            } else {
                return 1; // NE
            }
        } else {
            // Четвертый квадрант (SE и E и S)
            if abs_dx > abs_dy {
                return 2; // Восток
            } else if abs_dy > abs_dx {
                return 4; // Юг
            } else {
                return 3; // SE
            }
        }
    } else {
        if dy >= 0.0 {
            // Второй квадрант (NW и W и N)
            if abs_dx > abs_dy {
                return 6; // Запад
            } else if abs_dy > abs_dx {
                return 0; // Север
            } else {
                return 7; // NW
            }
        } else {
            // Третий квадрант (SW и W и S)
            if abs_dx > abs_dy {
                return 6; // Запад
            } else if abs_dy > abs_dx {
                return 4; // Юг
            } else {
                return 5; // SW
            }
        }
    }
}

