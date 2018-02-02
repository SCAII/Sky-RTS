use ndarray::Array2;
use specs::Entity;

use engine::components::AABB;

use std::collections::BTreeSet;

#[derive(Clone)]
pub struct SpatialHasher {
    grid: Array2<BTreeSet<Entity>>,
    cell_size: f64,
}

impl SpatialHasher {
    pub fn new(cell_size: f64) -> Self {
        SpatialHasher {
            grid: Array2::default((500, 500)),
            cell_size,
        }
    }

    pub fn add(&mut self, shape: &AABB, entity: Entity) {
        let ((min_x, max_x), (min_y, max_y)) = aabb_extents(&shape, self.cell_size);

        for i in min_y..max_y {
            for j in min_x..max_x {
                self.grid[(i, j)].insert(entity);
            }
        }
    }

    pub fn remove(&mut self, shape: &AABB, entity: Entity) {
        let ((min_x, max_x), (min_y, max_y)) = aabb_extents(&shape, self.cell_size);

        for i in min_y..max_y {
            for j in min_x..max_x {
                self.grid[(i, j)].remove(&entity);
            }
        }
    }

    /// A basic near phase collision test
    pub fn collisions(&self) -> BTreeSet<(Entity, Entity)> {
        let mut out = BTreeSet::new();

        for cell in self.grid.iter() {
            for (i, e1) in cell.iter().enumerate() {
                for e2 in cell.iter().skip(i) {
                    if out.contains(&(*e1, *e2)) || out.contains(&(*e2, *e1)) {
                        continue;
                    }

                    out.insert((*e1, *e2));
                }
            }
        }

        out
    }

    pub fn intersects(&self, shape: &AABB) -> BTreeSet<Entity> {
        let ((min_x, max_x), (min_y, max_y)) = aabb_extents(&shape, self.cell_size);

        let mut out = BTreeSet::new();

        for i in min_y..max_y {
            for j in min_x..max_x {
                let cell = &self.grid[(i, j)];
                for e1 in cell.iter() {
                    out.insert(*e1);
                }
            }
        }

        out
    }

    pub fn intersects_point(&self, x: f64, y: f64) -> &BTreeSet<Entity> {
        let x = (x / self.cell_size) as usize;
        let y = (y / self.cell_size) as usize;

        &self.grid[(y, x)]
    }
}

#[inline]
fn aabb_extents(aabb: &AABB, cell_size: f64) -> ((usize, usize), (usize, usize)) {
    let min_x = (aabb.pos.0[0] / cell_size) as usize;
    let max_x = ((aabb.pos.0[0] + aabb.width).ceil() / cell_size) as usize;

    let min_y = (aabb.pos.0[1] / cell_size) as usize;
    let max_y = ((aabb.pos.0[1] + aabb.height).ceil() / cell_size) as usize;

    ((min_x, max_x), (min_y, max_y))
}
