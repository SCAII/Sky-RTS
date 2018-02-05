use nalgebra::{Isometry2, Point2};
use ncollide::world::CollisionWorld;

pub struct ColliderData {}

pub type SkyCollisionWorld = CollisionWorld<Point2<f64>, Isometry2<f64>, ColliderData>;
