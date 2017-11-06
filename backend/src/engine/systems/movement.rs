use specs::{Entities, Entity, Fetch, ReadStorage, System, WriteStorage};
use engine::components::{Move, MoveBehavior, MoveTarget, MovedFlag, Pos};
use engine::DeltaT;

#[derive(SystemData)]
pub struct MoveSystemData<'a> {
    positions: WriteStorage<'a, Pos>,
    moves: ReadStorage<'a, Move>,
    moved: WriteStorage<'a, MovedFlag>,
    delta_t: Fetch<'a, DeltaT>,
    ids: Entities<'a>,
}

#[derive(Default)]
pub struct MoveSystem {
    // Reduce allocations by caching the largest the list
    // of deferred target seeks has ever been
    target_cache: Vec<(Entity, Move)>,
}

impl MoveSystem {
    pub fn new() -> Self {
        MoveSystem {
            target_cache: Vec::with_capacity(100),
        }
    }
}


impl<'a> System<'a> for MoveSystem {
    type SystemData = MoveSystemData<'a>;

    fn run(&mut self, mut sys_data: Self::SystemData) {
        use specs::Join;

        let targets = &mut self.target_cache;

        for (pos, moves, id) in (&mut sys_data.positions, &sys_data.moves, &*sys_data.ids).join() {
            sys_data.moved.insert(id, MovedFlag);

            match *moves {
                // For borrow reasons, we need to defer targeted moves until later
                // (we can't get a position while iterating over positions!)
                target @ Move {
                    target: MoveTarget::Unit(_),
                    ..
                } => {
                    targets.push((id, target.clone()));
                    continue;
                }
                Move {
                    target: MoveTarget::Ground(ref tar_pos),
                    ref behavior,
                } => move_ground(pos, tar_pos, behavior, sys_data.delta_t.0),
            }
        }

        targets.clear();
    }
}


fn move_ground(pos: &mut Pos, tar_pos: &Pos, behavior: &MoveBehavior, delta_t: f64) {
    match *behavior {
        MoveBehavior::Straight => {
            let dir = **tar_pos - **pos;

            let mut new_pos = **pos + (dir.normalize() * delta_t);

            let new_dir = new_pos - **tar_pos;

            /* Simple overshoot detection */

            if dir[0].signum() != new_dir[0].signum() {
                new_pos[0] = tar_pos[0];
            }

            if dir[1].signum() != new_dir[0].signum() {
                new_pos[1] = tar_pos[1];
            }

            **pos = new_pos;
        }
        MoveBehavior::Arrive => unimplemented!(),
    }
}
