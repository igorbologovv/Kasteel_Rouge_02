use bevy::{prelude::*, text};
use rand::Rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, BufReader};
use cgmath::Vector3;
use crate::components::{Condition, Soldier, SpriteSize, Team, Squad};
use crate::resources::{SpatialHash, WinSize, SquadVec};

fn update_center_of_mass<'a>(
    squads: &'a Vec<(Vec<Entity>, u16)>, 
    soldiers_query: &mut Query<(&mut Soldier, &mut Transform)>
) {
    for (squad, _id) in squads {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        if !squad.is_empty() {
            for &entity in squad.iter() {
                if let Ok((_s,t) ) = soldiers_query.get(entity) {
                    sum_x += t.translation.x;
                    sum_y += t.translation.y;
                }
            }

            let count = squad.len() as f32;
            if count > 0.0 {
                let center_of_mass = Vector3::new(sum_x / count, sum_y / count, 0.0);
                
                // Обновляем центр масс для всех солдат в отряде
                for &entity in squad.iter() {
                    if let Ok((mut soldier,_tr)) = soldiers_query.get_mut(entity) {
                        soldier.center_of_mass = center_of_mass;
                    }
                }
            }
        }
    }
}







pub fn modify_soldier_properties(){

}

pub fn moving_action(){



}