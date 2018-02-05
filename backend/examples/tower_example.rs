extern crate backend as sky_rts;
extern crate prost;
extern crate scaii_defs;
extern crate websocket;

use std::error::Error;

use scaii_defs::protos::ScaiiPacket;

use std::net::TcpStream;
use websocket::sync::Client;

fn main() {
    use scaii_defs::protos::{BackendCfg, BackendEndpoint, Cfg, CoreEndpoint, Endpoint, ScaiiPacket};
    use scaii_defs::protos::Action as ScaiiAction;
    use scaii_defs::protos::endpoint::Endpoint as End;
    use scaii_defs::protos::scaii_packet::SpecificMsg;
    use scaii_defs::protos::cfg::WhichModule;

    use sky_rts::protos::{ActionList, AttackUnit, UnitAction};
    use sky_rts::protos::unit_action::Action;

    use prost::Message;

    use websocket::sync::Server;
    use std::net::{Ipv4Addr, SocketAddr};
    use std::time::Duration;
    use std::thread::sleep;

    println!("Setting up RTS\n");

    let mut rts = sky_rts::new_backend();
    rts.process_msg(&ScaiiPacket {
        src: Endpoint {
            endpoint: Some(End::Core(CoreEndpoint {})),
        },
        dest: Endpoint {
            endpoint: Some(End::Backend(BackendEndpoint {})),
        },
        specific_msg: Some(SpecificMsg::Config(Cfg {
            which_module: Some(WhichModule::BackendCfg(BackendCfg {
                cfg_msg: None,
                is_replay_mode: false,
            })),
        })),
    }).unwrap();

    rts.process_msg(&ScaiiPacket {
        src: Endpoint {
            endpoint: Some(End::Core(CoreEndpoint {})),
        },
        dest: Endpoint {
            endpoint: Some(End::Backend(BackendEndpoint {})),
        },
        specific_msg: Some(SpecificMsg::ResetEnv(true)),
    }).unwrap();

    let mm = rts.get_messages();

    println!("Setting up webserver...");

    let mut server = Server::bind(SocketAddr::new(
        From::from(Ipv4Addr::new(127, 0, 0, 1)),
        6112,
    )).expect("Could not bind server to provided ip/port");

    let conn = match server.accept() {
        Err(err) => panic!(err),
        Ok(conn) => conn,
    };

    let mut conn = conn.accept().expect("Couldn't accept the connection");

    for packet in mm.packets {
        encode_and_send_proto(&mut conn, &packet).unwrap();
    }

    let actions = ActionList {
        actions: vec![
            UnitAction {
                unit_id: 0,
                action: Some(Action::AttackUnit(AttackUnit { target_id: 1 })),
            },
        ],
    };

    let mut buf = Vec::new();

    actions.encode(&mut buf).unwrap();

    rts.process_msg(&ScaiiPacket {
        src: Endpoint {
            endpoint: Some(End::Core(CoreEndpoint {})),
        },
        dest: Endpoint {
            endpoint: Some(End::Backend(BackendEndpoint {})),
        },
        specific_msg: Some(SpecificMsg::Action(ScaiiAction {
            alternate_actions: Some(buf),
            ..Default::default()
        })),
    }).unwrap();

    let mm = rts.get_messages();

    for packet in mm.packets {
        encode_and_send_proto(&mut conn, &packet).unwrap();
    }

    let empty_action = ScaiiPacket {
        src: Endpoint {
            endpoint: Some(End::Core(CoreEndpoint {})),
        },
        dest: Endpoint {
            endpoint: Some(End::Backend(BackendEndpoint {})),
        },
        specific_msg: Some(SpecificMsg::Action(ScaiiAction {
            ..Default::default()
        })),
    };

    for _ in 0..500 {
        rts.process_msg(&empty_action).unwrap();
        let mm = rts.get_messages();

        for packet in mm.packets {
            match packet.dest {
                Endpoint {
                    endpoint: Some(End::Agent(..)),
                } => {
                    continue;
                }
                _ => {}
            }
            encode_and_send_proto(&mut conn, &packet).unwrap();
        }

        sleep(Duration::from_millis(17));
    }
}

fn encode_and_send_proto(
    client: &mut Client<TcpStream>,
    packet: &ScaiiPacket,
) -> Result<(), Box<Error>> {
    use prost::Message;
    use websocket::message;
    let mut buf: Vec<u8> = Vec::new();
    packet
        .encode(&mut buf)
        .expect("Could not encode SCAII packet (server error)");

    client.send_message(&message::Message::binary(buf))?;
    Ok(())
}
