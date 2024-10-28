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
    // It is importatnt to cast to i32 and not u32 (position[0] / self.cell_size).floor() because negative numbers
    pub fn to_grid_coords(&self, position: Vec3) -> Vector3<u32> {
        let sz: Vector3<usize> = self.sh.size();
        let x = (position[0] / self.cell_size).floor() as i32 + (sz[0] as i32 / 2);
        let y = (position[1] / self.cell_size).floor() as i32 + (sz[1] as i32 / 2);

        // Обеспечить, чтобы индексы не выходили за пределы
        let x = x.max(0) as u32;
        let y = y.max(0) as u32;

        Vector3 { x, y, z: 0 }
    }

    // Добавление и удаление объектов из ячеек
    pub fn add_entity(&mut self, pos: Vec3, entt: Entity) {
        let t = self.to_grid_coords(pos);
        let index = self
            .pos_to_index(t)
            .expect("Position out of bounds");
        let cellref = self.get_mut(index).expect("Index out of bounds");

        println!("Current entities in cell: {:?}", cellref);

        // Проверяем наличие сущности в ячейке
        if cellref.iter().find(|&&e| e == entt).is_none() {
            println!(
                "Entity with SHindex {} and coordinates shX:{} and shY: {}, world: {} added ENTITY: {} \n",
                index, t.x, t.y, pos, &entt
            );
            cellref.push(entt);
        } else {
            println!("Entity {} already exists in new cell!", entt);
        }
    }

    pub fn remove_entity(&mut self, pos: Vec3, entt: Entity) {
        let coords = self.to_grid_coords(pos);
        let index = self
            .pos_to_index(coords)
            .expect("Position out of bounds");
        let cellref = self.get_mut(index).expect("Index out of bounds");

        // Удаление объекта из ячейки
        cellref.retain(|e| *e != entt);
    }
}

#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}
