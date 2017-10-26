use scaii_defs::protos::{Viz, VizInit};
use engine::system::{Movement, Render};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum GameInit {
    Towers,
}

impl GameInit {
    pub fn init(&self, render_sys: &mut Render, move_sys: &mut Movement) -> (VizInit, Viz) {
        match *self {
            GameInit::Towers => two_towers(render_sys, move_sys),
        }
    }
}

fn two_towers(render_sys: &mut Render, move_sys: &mut Movement) -> (VizInit, Viz) {
    use engine::entity::components::{Pos, Renderable};
    use engine::graphics::{Color, Shape};
    use engine::system::System;
    use scaii_defs::protos::Entity;
    use scaii_defs::protos;

    //agent
    move_sys.add_component(
        0,
        Pos {
            x: 50.0,
            y: 50.0,
            heading: 0.0,
        },
    );

    render_sys.add_component(
        0,
        Renderable {
            pos: Pos {
                x: 50.0,
                y: 50.0,
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
            x: Some(50.0),
            y: Some(50.0),
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

    //good_tower
    move_sys.add_component(
        1,
        Pos {
            x: 5.0,
            y: 50.0,
            heading: 0.0,
        },
    );

    render_sys.add_component(
        1,
        Renderable {
            pos: Pos {
                x: 5.0,
                y: 50.0,
                heading: 0.0,
            },
            color: Color {
                r: 0,
                b: 0,
                g: 255,
                a: 255,
            },
            shape: Shape::Rect {
                width: 10.0,
                height: 10.0,
            },
        },
    );

    let good_packet = Entity {
        id: 1,
        pos: Some(protos::Pos {
            x: Some(5.0),
            y: Some(50.0),
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
                    b: 0,
                    g: 255,
                    a: 255,
                }),
                rect: Some(protos::Rect {
                    width: Some(10.0),
                    height: Some(10.0),
                }),
                triangle: None,
                delete: false,
            },
        ],
        delete: false,
    };

    // bad
    move_sys.add_component(
        2,
        Pos {
            x: 95.0,
            y: 50.0,
            heading: 0.0,
        },
    );

    render_sys.add_component(
        2,
        Renderable {
            pos: Pos {
                x: 95.0,
                y: 50.0,
                heading: 0.0,
            },
            color: Color {
                r: 255,
                b: 0,
                g: 0,
                a: 255,
            },
            shape: Shape::Rect {
                width: 10.0,
                height: 10.0,
            },
        },
    );

    let bad_packet = Entity {
        id: 2,
        pos: Some(protos::Pos {
            x: Some(95.0),
            y: Some(50.0),
        }),
        shapes: vec![
            protos::Shape {
                id: 0,
                relative_pos: Some(protos::Pos {
                    x: Some(0.0),
                    y: Some(0.0),
                }),
                color: Some(protos::Color {
                    r: 255,
                    b: 0,
                    g: 0,
                    a: 255,
                }),
                rect: Some(protos::Rect {
                    width: Some(10.0),
                    height: Some(10.0),
                }),
                triangle: None,
                delete: false,
            },
        ],
        delete: false,
    };

    let init = VizInit {
        test_mode: Some(false),
    };

    let viz = Viz {
        entities: vec![agent_packet, good_packet, bad_packet],
    };


    (init, viz)
}
