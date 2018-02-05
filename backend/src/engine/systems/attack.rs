use specs::{Fetch, ReadStorage, System, WriteStorage};
use engine::components::{Attack, Death, Hp, UnitTypeTag};
use engine::resources::{DeltaT, UnitTypeMap};

#[derive(SystemData)]
pub struct AttackSystemData<'a> {
    attack: WriteStorage<'a, Attack>,
    hp: WriteStorage<'a, Hp>,
    death: WriteStorage<'a, Death>,

    delta_t: Fetch<'a, DeltaT>,
    unit_type_map: Fetch<'a, UnitTypeMap>,
    tag: ReadStorage<'a, UnitTypeTag>,
}

pub struct AttackSystem;

impl<'a> System<'a> for AttackSystem {
    type SystemData = AttackSystemData<'a>;

    fn run(&mut self, mut sys_data: Self::SystemData) {
        use specs::Join;

        let delta_t = sys_data.delta_t.0;

        for (atk, tag) in (&mut sys_data.attack, &sys_data.tag).join() {
            let unit_type = sys_data.unit_type_map.tag_map.get(&tag.0).unwrap();

            atk.time_since_last += delta_t;

            if atk.time_since_last > unit_type.attack_delay {
                atk.time_since_last = 0.0;

                let tar_hp = sys_data.hp.get_mut(atk.target).unwrap();

                tar_hp.curr_hp -= unit_type.attack_damage;

                if tar_hp.curr_hp <= 0.0 {
                    sys_data.death.insert(atk.target, Death);
                }
            }
        }
    }
}
