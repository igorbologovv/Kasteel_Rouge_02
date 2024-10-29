use bevy::{prelude::*, text};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufReader};

use crate::components::{Condition, Soldier, SpriteSize, Team, Squad};
use crate::resources::{SpatialHash, WinSize, SquadVec};



use std::rc::Rc;

fn update_center_of_mass(
    squads: &Vec<(Rc<Vec<Entity>>, u16)>, 
    soldiers_query: Query<(&Soldier, &Team, &Squad)>
) -> Vec<(Rc<Vec<Entity>>, (f32, f32), u16)> {
    let mut centers_of_mass = Vec::new();

    for (squad, id) in squads {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for &entity in squad.iter() {
            if let Ok((soldier, _team, _squad)) = soldiers_query.get(entity) {
                sum_x += soldier.sh_coords.x as f32;
                sum_y += soldier.sh_coords.y as f32;
            }
        }

        let count = squad.len() as f32;
        if count > 0.0 {
            let center_of_mass = (sum_x / count, sum_y / count);
            centers_of_mass.push((Rc::clone(squad), center_of_mass, *id)); // Используем `Rc::clone`, что не копирует данные, а лишь увеличивает счетчик ссылок
        }
    }

    centers_of_mass
}

pub fn analyze_circumstance(sh: Res<SpatialHash>, squads: Res<SquadVec>, soldiers_query: &Query<(&Soldier, &Team, &Squad)>){


    for squad in squads{
        let center_of_mass = update_center_of_mass(squad, soldiers_query);

        

    }

}

pub fn modify_soldier_properties(){

}

pub fn moving_action(){



}