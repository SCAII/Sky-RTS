use std::collections::HashMap;

use super::FactionId;
use super::components::{Pos, Shape};

use scaii_defs::protos::{Action, Viz};

use specs::World;

pub mod collision;

pub use self::collision::*;

// Recommended by ncollide
pub const COLLISION_MARGIN: f64 = 0.02;
// ncollide wants the average size of a collider to be "around" 1
// we should probably set this as a resource from Lua in the future
pub const COLLISION_SCALE: f64 = 50.0;

pub const MAX_FACTIONS: usize = 15;

lazy_static! {
    static ref SENSOR_BLACKLIST: Vec<usize> = (MAX_FACTIONS..30).collect();
}
use super::SIXTY_FPS;

pub(super) fn register_world_resources(world: &mut World) {
    use util;
    use specs::saveload::U64MarkerAllocator;

    let rng = util::make_rng();
    world.add_resource(rng);
    world.add_resource(Episode(0));
    world.add_resource(Terminal(false));
    world.add_resource(DeltaT(SIXTY_FPS));
    world.add_resource(Render::default());
    world.add_resource(NeedsKeyInfo(true));
    world.add_resource::<Vec<Player>>(Vec::new());
    world.add_resource(UnitTypeMap::default());
    world.add_resource(U64MarkerAllocator::new());
    world.add_resource(ActionInput::default());
    world.add_resource(SkyCollisionWorld::new(COLLISION_MARGIN));
}

/// The current episode, only meaningful for sequential runs.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Episode(pub usize);

/// Is this the final frame of the scenario?
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Terminal(pub bool);

/// Time since the last update, in seconds (fixed to one sixtieth of a second for our purposes).
#[derive(Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DeltaT(pub f64);

/// Any associated data with various game factions.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub color: super::components::Color,
    pub id: FactionId,
}

/// The output of the renderer, for use with Viz.
#[derive(Clone, PartialEq, Default)]
pub struct Render(pub Viz);

/// Tracks whether a FULL rerender (or total state, or whatever else)
/// is needed rather than a delta.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct NeedsKeyInfo(pub bool);

/// The actions coming from the Agent (or replay mechanism)
#[derive(Clone, PartialEq, Default, Debug)]
pub struct ActionInput(pub Option<Action>);

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct UnitType {
    pub tag: String,
    pub max_hp: usize,
    pub movable: bool,
    pub shape: Shape,
    pub kill_reward: f64,
    pub death_penalty: f64,
    pub damage_deal_reward: Option<f64>,
    pub damage_recv_penalty: Option<f64>,
    pub speed: f64,
    pub attack_range: f64,
}

impl Default for UnitType {
    fn default() -> Self {
        UnitType {
            tag: "".to_string(),
            max_hp: 100,
            movable: true,
            shape: Shape::Triangle { base_len: 10.0 },
            kill_reward: 0.0,
            death_penalty: 0.0,
            damage_deal_reward: None,
            damage_recv_penalty: None,
            speed: 20.0,
            attack_range: 10.0,
        }
    }
}

impl UnitType {
    pub fn build_entity(&self, world: &mut World, pos: Pos, faction: usize) {
        use specs::saveload::U64Marker;
        use ncollide::shape::{Ball, Cuboid, Cylinder, ShapeHandle};
        use ncollide::world::{CollisionGroups, GeometricQueryType};
        use nalgebra::{Isometry2, Vector2};
        use nalgebra;
        use std::f64;

        use engine::components::{AttackSensor, CollisionHandle, Movable, Shape, Speed, Static};

        /* Setup collision */

        let mut collider_group = CollisionGroups::new();
        collider_group.modify_membership(faction - 1, true);

        let mut sensor_group = CollisionGroups::new();
        sensor_group.modify_membership(MAX_FACTIONS + (faction - 1), true);
        sensor_group.set_blacklist(&SENSOR_BLACKLIST);

        let (collider, atk_radius): (ShapeHandle<_, _>, ShapeHandle<_, _>) = match self.shape {
            Shape::Rect { width, height } => {
                let width = width / COLLISION_SCALE;
                let height = height / COLLISION_SCALE;

                // ncollide likes half widths and heights, so divide by 2
                let collider = Cuboid::new(Vector2::new(
                    width / COLLISION_SCALE / 2.0,
                    height / COLLISION_SCALE / 2.0,
                ));
                let collider = ShapeHandle::new(collider);

                let atk_radius = width.max(height) + (self.attack_range / COLLISION_SCALE);
                let atk_sensor = Ball::new(atk_radius);
                let atk_sensor = ShapeHandle::new(atk_sensor);

                (collider, atk_sensor)
            }
            Shape::Triangle { base_len } => {
                let base_len = base_len / COLLISION_SCALE;

                // equilateral triangle dimensions
                let half_height = base_len / (2.0 as f64).sqrt() / 2.0;
                let radius = base_len / 2.0;

                // A cylinder in 2D is an isoscelese triangle in ncollide
                let collider = Cylinder::new(half_height, radius);
                let collider = ShapeHandle::new(collider);

                let atk_radius = half_height + (self.attack_range / COLLISION_SCALE);
                let atk_sensor = Ball::new(atk_radius);
                let atk_sensor = ShapeHandle::new(atk_sensor);

                (collider, atk_sensor)
            }
        };

        let color = { world.read_resource::<Vec<Player>>()[faction].color };

        // Scoping for borrow shenanigans
        let entity = {
            let entity = world
                .create_entity()
                .with(pos)
                .with(self.shape)
                .with(color)
                .with(FactionId(faction))
                .marked::<U64Marker>();

            if self.movable {
                entity.with(Movable).with(Speed(self.speed))
            } else {
                entity.with(Static)
            }
        }.build();

        // We need the entity ID for this, so do it after building the entity and then add the component.
        let (collider, atk_radius) = {
            let pos = Isometry2::new(
                Vector2::new(pos.x / COLLISION_SCALE, pos.y / COLLISION_SCALE),
                nalgebra::zero(),
            );
            let collision: &mut SkyCollisionWorld = &mut *world.write_resource();

            let q_type = GeometricQueryType::Contacts(10.0, 10.0);
            let collider = collision.add(
                pos,
                collider,
                collider_group,
                q_type,
                ColliderData { e: entity },
            );

            let atk_radius = collision.add(
                pos,
                atk_radius,
                sensor_group,
                q_type,
                ColliderData { e: entity },
            );

            (collider, atk_radius)
        };

        world
            .write::<CollisionHandle>()
            .insert(entity, CollisionHandle(collider));
        world
            .write::<AttackSensor>()
            .insert(entity, AttackSensor(atk_radius));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct UnitTypeMap {
    pub typ_vec: Vec<UnitType>,
    pub tag_map: HashMap<String, UnitType>,
}
