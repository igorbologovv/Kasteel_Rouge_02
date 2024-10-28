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
pub struct Squad {
    pub unit: UnitType,
    pub size: (u8, u8),
    pub current_order: SquadOrder,
    pub belong_to_id: Option<PlayerId>,
    pub id: u8,
}

impl Squad {
    fn new(
        unit: UnitType,
        size: (u8, u8),
        current_order: SquadOrder,
        belong_to_id: Option<PlayerId>,
        id: u8,
    ) -> Self {
        Self {
            unit,
            size,
            current_order,
            belong_to_id,
            id,
        }
    }
    fn get_order(&mut self) -> SquadOrder {
        self.current_order
    }

    fn set_order(&mut self, order: SquadOrder) {
        self.current_order = order;
    }
}
