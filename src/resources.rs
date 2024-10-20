use bevy::prelude::*;
use cgmath::Vector3;
use smallvec::SmallVec;

type SpatialHashCell = smallvec::SmallVec<[Entity; 6]>;

#[derive(Resource)]
pub struct GameTextures {
    pub swordman: Handle<Image>,
    pub archer: Handle<Image>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SpatialHash {
    #[deref]
    pub sh: spatial_hash_3d::SpatialHashGrid<SpatialHashCell>,
    pub cell_size: f32,
}

impl SpatialHash {
    pub fn new(xcells: usize, ycells: usize, cell_size: f32) -> Self {
        Self {
            sh: spatial_hash_3d::SpatialHashGrid::new(xcells, ycells, 1, SmallVec::new),
            cell_size,
        }
    }

    pub fn to_grid_coords(&self, position: Vec3) -> Vector3<u32> {
        let sz = self.sh.size();
        let x = (position[0] / self.cell_size) as u32 + sz[0] as u32 / 2;
        let y = (position[1] / self.cell_size) as u32 + sz[1] as u32 / 2;
        Vector3 { x, y, z: 0 }
    }
}

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
