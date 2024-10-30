use std::default;

use bevy::prelude::*;
use cgmath::Vector3;
#[derive(Component)]
pub struct Soldier {
    pub hit_chance: i8,
    pub dodge_chance: i8,
    pub velocity: (i8, i8, i8),
    pub squad_num: u16,
    pub sh_coords: Vector3<u32>,
    pub center_of_mass: Vector3<f32>,
    pub target_direction: Vector3<f32>
}
impl Soldier {
    pub fn new(vx: i8, vy: i8, vz: i8, squad_num: u16, sh_coords:  Vector3<u32>, cm: Vector3<f32>, td:Vector3<f32> ) -> Self {
        Soldier {
            velocity: (vx, vy, vz),
            squad_num: squad_num,
            hit_chance: 50,
            dodge_chance: 50,
            sh_coords:sh_coords,
            center_of_mass: cm,
            target_direction: td

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
    pub is_active: bool,
    pub is_moving: bool,
    pub is_dead: bool,
    pub is_wounded: bool,
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

#[derive(Component, Debug)]
pub struct AIComponent {
    pub situation_near: [i8; 8],
    pub situation_far: [i8; 8],
}

impl AIComponent {
    fn new(snear: [i8; 8], sfar: [i8; 8]) -> AIComponent {
        AIComponent {
            situation_near: snear,
            situation_far: sfar,
        }
    }
}
impl Default for AIComponent {
    fn default() -> Self {
        AIComponent {
            situation_near: [0, 0, 0, 0, 0, 0, 0, 0],
            situation_far: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}
