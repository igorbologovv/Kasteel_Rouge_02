use bevy::{prelude::*, text};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufReader};

use crate::components::{Condition, Soldier, SpriteSize, Team, Squad};
use crate::resources::{SpatialHash, WinSize, SquadVec};

fn update_center_of_mass<'a>(
    squads: &'a Vec<(Vec<Entity>, u16)>, 
    soldiers_query: &Query<(&Soldier, &Team, &Squad)>
) -> Vec<(&'a Vec<Entity>, (f32, f32), u16)> {
    let mut centers_of_mass = Vec::new();

    for (squad, id) in squads {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        if !squad.is_empty() {
            for &entity in squad.iter() {
                if let Ok((soldier, _team, _squad)) = soldiers_query.get(entity) {
                    sum_x += soldier.sh_coords.x as f32;
                    sum_y += soldier.sh_coords.y as f32;
                }
            }

            let count = squad.len() as f32;
            if count > 0.0 {
                let center_of_mass = (sum_x / count, sum_y / count);
                centers_of_mass.push((squad, center_of_mass, *id));
            }
        }
    }

    centers_of_mass
}

pub fn analyze_circumstance(
    sh: Res<SpatialHash>, 
    squads: Res<SquadVec>, 
    soldiers_query: &Query<(&Soldier, &Team, &Squad)>
) {
    let centers_of_mass = update_center_of_mass(&squads.get_squads(), soldiers_query);

    for (squad, (center_x, center_y), squad_id) in centers_of_mass {
        println!(
            "Центр масс отряда с идентификатором {} находится в координатах ({}, {})",
            squad_id, center_x, center_y
        );

        // Дополнительная логика для анализа и взаимодействия с отрядом
    }
}



pub fn modify_soldier_properties(){

}

pub fn moving_action(){



}