use bevy::prelude::*;
use cgmath::Vector3;
#[derive(Component)]
pub struct Soldier {
    pub is_active: bool,
    pub is_moving: bool,
    pub pos: bevy::prelude::Vec3,
    pub velocity: (i8, i8, i8),
}



#[derive(Component)]
pub struct Mem{
    pub mem: Vec<([bool; 8], i8)>,
}

#[derive(Component)]
pub struct SpriteSize {
    pub height: i8,
    pub width: i8,
}

#[derive(Component)]
pub struct Condition {
    pub morale: i8,
    pub stamina: i8,
    pub strength: i8,
    pub danger_perception: i8,
}

impl Soldier {
    pub fn new(_pos: Vec3, vx: i8, vy: i8, vz: i8) -> Self {
        Soldier {
            is_active: false,
            is_moving: false,
            pos: _pos,
            velocity: (vx, vy, vz),
        }
    }
}

#[derive(Component)]
pub struct Team(pub u8);

#[derive(Component)]
pub struct Squad(pub u8);
