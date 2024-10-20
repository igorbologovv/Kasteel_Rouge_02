use crate::components::{Condition, Soldier};
use crate::resources::{SpatialHash, WinSize};
use bevy::prelude::*;
use cgmath::Vector3;

// TODO Use spatial hash to understand relative position of the soldier.
//TODO
pub fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform, &mut Condition)>,
    mut spatial_hash: ResMut<crate::SpatialHash>,
) {
}
