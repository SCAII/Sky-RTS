use specs::{Fetch, FetchMut, ReadStorage, System};
use engine::components::{FactionId, Hp, UnitTypeTag};
use engine::resources::{Reward, RtsState, SkyCollisionWorld, Terminal, UnitTypeMap};
use ndarray::Array3;

#[derive(SystemData)]
pub struct StateBuildSystemData<'a> {
    hp: ReadStorage<'a, Hp>,
    faction: ReadStorage<'a, FactionId>,
    collision_sys: Fetch<'a, SkyCollisionWorld>,
    tag: ReadStorage<'a, UnitTypeTag>,
    unit_types: Fetch<'a, UnitTypeMap>,
    terminal: Fetch<'a, Terminal>,

    state: FetchMut<'a, RtsState>,
    reward: FetchMut<'a, Reward>,
}

pub struct StateBuildSystem {
    state_cache: Array3<f64>,
}

impl StateBuildSystem {
    pub fn new() -> Self {
        StateBuildSystem {
            state_cache: Array3::zeros([100, 100, 4]),
        }
    }
}

impl<'a> System<'a> for StateBuildSystem {
    type SystemData = StateBuildSystemData<'a>;

    fn run(&mut self, mut sys_data: Self::SystemData) {
        use nalgebra::Point2;
        use ncollide::world::CollisionGroups;
        use engine::resources::{COLLISION_SCALE, SENSOR_BLACKLIST};
        use std::mem;

        let c_world = &*sys_data.collision_sys;

        let mut c_group = CollisionGroups::new();
        for i in 0..15 {
            c_group.modify_membership(i, true);
        }
        c_group.set_blacklist(&SENSOR_BLACKLIST);

        /* This is probably speed uppable using the Dead and Moved marker components */
        for i in 0..100 {
            for j in 0..100 {
                let pt = Point2::new(
                    ((i * 5) as f64) / COLLISION_SCALE,
                    ((j * 5) as f64) / COLLISION_SCALE,
                );
                let mut intersection = c_world.interferences_with_point(&pt, &c_group);

                if let Some(collider) = intersection.next() {
                    let entity = collider.data().e;
                    // Need to offset by 1 because the default is 0
                    self.state_cache[(i, j, 0)] = (entity.id() + 1) as f64;
                    self.state_cache[(i, j, 1)] = sys_data.hp.get(entity).unwrap().curr_hp as f64;

                    let u_type = sys_data
                        .unit_types
                        .typ_ids
                        .get(&sys_data.tag.get(entity).unwrap().0)
                        .unwrap();
                    self.state_cache[(i, j, 2)] = *u_type as f64;
                    self.state_cache[(i, j, 3)] = sys_data.faction.get(entity).unwrap().0 as f64;
                } else {
                    for k in 0..4 {
                        self.state_cache[(i, j, k)] = 0.0;
                    }
                }
            }
        }

        let old_state = mem::replace(&mut sys_data.state.0.features, vec![]);
        let new_cache = Array3::from_shape_vec([100, 100, 4], old_state).unwrap();

        sys_data.state.0.features = mem::replace(&mut self.state_cache, new_cache).into_raw_vec();

        mem::swap(&mut sys_data.state.0.typed_reward, &mut sys_data.reward.0);
        sys_data.state.0.terminal = sys_data.terminal.0;

        sys_data.state.0.reward = Some(
            sys_data
                .state
                .0
                .typed_reward
                .values()
                .fold(0.0, |acc, v| acc + v),
        );
    }
}
