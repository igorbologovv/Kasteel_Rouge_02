use crate::components::Condition;
use crate::resources::WinSize;
use bevy::prelude::*;

pub fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &mut Transform, &mut Condition)>,
    mut spatial_hash: ResMut<crate::SpatialHash>,
) {
}
