use nalgebra::{Isometry2, Point2};
use ncollide::world::CollisionWorld;
use specs::Entity;

pub struct ColliderData {
    pub e: Entity,
}

pub type SkyCollisionWorld = CollisionWorld<Point2<f64>, Isometry2<f64>, ColliderData>;
