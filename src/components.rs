use bevy::prelude::*;
use cgmath::Vector3;
#[derive(Component)]
pub struct Soldier {
    pub is_active: bool,
    pub is_moving: bool,
    pub velocity: (i8, i8, i8),
    pub squad_num: u16,
}
impl Soldier {
    pub fn new(_pos: Vec3, vx: i8, vy: i8, vz: i8, squad_num: u16) -> Self {
        Soldier {
            is_active: false,
            is_moving: false,
            velocity: (vx, vy, vz),
            squad_num: squad_num,
        }
    }
}
#[derive(Component)]
pub struct SharedMemory {
    pub alies: [bool; 8],
    pub enimies: [bool; 8],
    pub action_probability: [f32; 4],
}

//Shred memory componen should be associated with each squad. It means only soldiers of certain squad can interact with this struct
impl SharedMemory {
    pub fn new() -> SharedMemory {
        SharedMemory {
            alies: [false; 8],
            enimies: [false; 8],
            action_probability: [0.1; 4],
        }
    }
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

#[derive(Component)]
pub struct Team(pub u8);

#[derive(Component, Debug)]
pub struct PlayerId {
    pub id: u32,
}

#[derive(Component, Debug)]
pub enum UnitType {
    Archers,
    Swordsman,
}

impl Default for UnitType {
    fn default() -> Self {
        UnitType::Swordsman
    }
}
#[derive(Component, Debug, Copy, Clone)]
pub enum SquadOrder {
    MoveTo(f32, f32),
    Defence(bool),
    Attack(bool),
}

#[derive(Component, Debug)]
pub struct Squad(pub u16);
