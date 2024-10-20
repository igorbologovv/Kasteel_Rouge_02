use bevy::prelude::*;
use cgmath::Vector3;
#[derive(Component)]
pub struct Soldier {
    pub pos: cgmath::Vector3<f32>,
    pub velocity: (i8, i8, i8),
}

#[derive(Component)]
pub struct Condition {
    pub morale: i8,
    pub stamina: i8,
    pub strength: i8,
    pub danger_perception: i8,
}

impl Soldier {
    pub fn new(_pos: Vector3<f32>, vx: i8, vy: i8, vz: i8) -> Self {
        Soldier {
            pos: _pos,
            velocity: (vx, vy, vz),
        }
    }
}

#[derive(Component)]
pub struct Team(pub u8);

#[derive(Component)]
pub struct Squad(pub u8);
