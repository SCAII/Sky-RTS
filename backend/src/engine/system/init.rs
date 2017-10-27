use scaii_defs::protos::{Viz, VizInit};
use engine::system::{Movement, Render};
use engine::entity::{EntityId, PlayerId};
use rand::Rng;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum GameInit {
    Towers,
}

impl GameInit {
    pub fn init<R: Rng>(
        &self,
        render_sys: &mut Render,
        move_sys: &mut Movement,
        faction: &mut BTreeMap<EntityId, PlayerId>,
        rng: &mut R,
    ) -> (VizInit, Viz) {
        match *self {
            GameInit::Towers => towers(render_sys, move_sys, faction, rng),
        }
    }
}

fn towers<R: Rng>(
    render_sys: &mut Render,
    move_sys: &mut Movement,
    faction: &mut BTreeMap<EntityId, PlayerId>,
    rng: &mut R,
) -> (VizInit, Viz) {
    use engine::entity::components::{Pos, Renderable};
    use engine::graphics::{Color, Shape};
    use engine::system::System;
    use scaii_defs::protos::Entity;
    use scaii_defs::protos;

    let entity_x = 350.0;
    let entity_y = 350.0;

    //agent
    move_sys.add_component(
        0,
        Pos {
            x: entity_x,
            y: entity_y,
            heading: 0.0,
        },
    );

    render_sys.add_component(
        0,
        Renderable {
            pos: Pos {
                x: entity_x,
                y: entity_y,
                heading: 0.0,
            },
            color: Color {
                r: 0,
                b: 255,
                g: 0,
                a: 255,
            },
            shape: Shape::Triangle { base_len: 10.0 },
        },
    );

    let agent_packet = Entity {
        id: 0,
        pos: Some(protos::Pos {
            x: Some(entity_x),
            y: Some(entity_y),
        }),
        shapes: vec![
            protos::Shape {
                id: 0,
                relative_pos: Some(protos::Pos {
                    x: Some(0.0),
                    y: Some(0.0),
                }),
                color: Some(protos::Color {
                    r: 0,
                    b: 255,
                    g: 0,
                    a: 255,
                }),
                rect: None,
                triangle: Some(protos::Triangle {
                    base_len: Some(10.0),
                }),
                delete: false,
            },
        ],
        delete: false,
    };

    faction.insert(0, 0);
    let mut viz = Viz {
        entities: vec![agent_packet],
    };

    let tower_width = 15.0;
    let tower_height = 15.0;

            let mut x_pos = rng.gen_range(0.0, 1000.0); 
        let mut y_pos = rng.gen_range(0.0, 1000.0);

    for i in 1..rng.gen_range(8, 12) {
        let good: bool = rng.gen();
        faction.insert(i, if good { 1 } else { 2 });
        x_pos = rng.gen_range(0.0, 1000.0); 
        y_pos = rng.gen_range(0.0, 1000.0);

        while ((entity_x - x_pos).powf(2.0) + (entity_y - y_pos).powf(2.0)).sqrt() < 100.0 {
            x_pos = rng.gen_range(0.0, 1000.0);
            y_pos = rng.gen_range(0.0, 1000.0);
        }

        move_sys.add_component(
            i,
            Pos {
                x: x_pos,
                y: y_pos,
                heading: 0.0,
            }
        );

        render_sys.add_component(
        i,
        Renderable {
            pos: Pos {
                x: x_pos,
                y: y_pos,
                heading: 0.0,
            },
            color: Color {
                r: if good { 0 } else { 255 },
                g: if good { 255 } else { 0 },
                b: 0,
                a: 255,
            },
            shape: Shape::Rect {
                width: tower_width,
                height: tower_height,
            },
        },
        );

        viz.entities.push(
            Entity {
                id: i as u64,
                pos: Some(protos::Pos {
                    x: Some(x_pos),
                    y: Some(y_pos)
                }),
                shapes: vec![
                    protos::Shape {
                        id: 0,
                        relative_pos: Some(protos::Pos {
                            x: Some(0.0),
                            y: Some(0.0),
                        }),
                        color: Some(protos::Color {
                            r: if good { 0 } else  { 255 },
                            g: if good { 255 } else { 0 },
                            b: 0,
                            a: 255
                        }),
                        rect: Some(protos::Rect {
                            width: Some(tower_width),
                            height: Some(tower_height)
                        }),
                        triangle: None,
                        delete: false,
                    }
                ],
                delete: false,
            }
        )


    }


    let init = VizInit {
        test_mode: Some(false),
    };


    (init, viz)
}
